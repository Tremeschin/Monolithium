/* -------------------------------------------------------------------------- */
// Failed attempt to find monoliths with cuda.
//
// I'm somewhat sure the lack of memory locality between different seeds and
// the large-ish fractal perlin structs for the tiny cores killed performance.
//
// Maybe it's possible to fix it, or perhaps certain computations are just
// better suited for the CPU even if embarrassingly parallel. This was my first
// time toying with cuda, chances are I did something wrong, PRs are welcome
// to improve this code for the endeavor to find the largest monoliths :^)
//
// Compiling and running:
// - Install CUDA from your package manager (nvcc), have it on path, good luck
//
// Either manually:
//   - Run: `meson setup --buildtype release ./build --reconfigure`
//   - Run: `ninja -C ./build`
//   - Run: `./build/monolithium`
//
// Or install [uv](https://docs.astral.sh/uv/):
//   - Run: `uv run monolithium-cuda`
/* -------------------------------------------------------------------------- */

#include <cmath>
#include <cstdint>
#include <cstdio>
#include <unistd.h>

#include <cuda_runtime.h>

#define Gpu __device__
#define Cpu __host__

#define SKIP_REJECTION 1
#define SKIP_TABLE 1

/* -------------------------------------------------------------------------- */
// Utility functions

Gpu inline float fade(float t) {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

Gpu inline float lerp(float t, float a, float b) {
    // return a + t * (b - a);
    return fma(t, b - a, a);
}

Gpu inline float grad(uint8_t hash, float x, float y, float z) {
    int h = hash & 15;
    float u = h < 8 ? x : y;
    float v = h < 4 ? y : h == 12 || h == 14 ? x : z;
    return ((h & 1) == 0 ? u : -u) + ((h & 2) == 0 ? v : -v);
}

/* -------------------------------------------------------------------------- */
// Java RNG implementation

constexpr int64_t M = (1LL << 48) - 1;
constexpr int64_t A = 0x5DEECE66DLL;
constexpr int64_t C = 11LL;

constexpr double F64_DIV = (1ULL << 53);

struct JavaRNG {
    int64_t state;

    Gpu inline JavaRNG(uint64_t seed) {
        this->state = ((int64_t) seed ^ A) & M;
    }

    /// Roll the state, same effect as ignoring a `.next()` call
    Gpu inline void step() {
        this->state = (this->state * A + C) & M;
    }

    /// Rolls the state and returns N low bits
    Gpu inline int32_t next(uint8_t bits) {
        this->step();
        return (int32_t) (this->state >> (48 - bits));
    }

    Gpu inline int32_t next_i32_bound(int32_t max) {
        if (__popc(max) == 1) {
            return (int32_t)(((int64_t) max * (int64_t) this->next(31)) >> 31);
        } else {
            int32_t next = this->next(31);
            int32_t take = next % max;

            #if SKIP_REJECTION
            #else
                while (next - take + max - 1 < 0) {
                    next = this->next(31);
                    take = next % max;
                }
            #endif

            return take;
        }
    }

    Gpu inline double next_f64() {
        int64_t high = ((int64_t) this->next(26)) << 27;
        int64_t low  =  (int64_t) this->next(27);
        return (double)(high | low) / F64_DIV;
    }
};

/* -------------------------------------------------------------------------- */

struct PerlinNoise {
    uint8_t map[256];
    float xoff;
    float yoff;
    float zoff;

    Gpu void init(JavaRNG* rng) {
        this->xoff = (float) (rng->next_f64() * 256.0);
        this->yoff = (float) (rng->next_f64() * 256.0);
        this->zoff = (float) (rng->next_f64() * 256.0);

        // Start a new 'arange' array
        for (int i=0; i<256; i++) {
            this->map[i] = i & 0xFF;
        }

        // Shuffle the first half
        for (int a=0; a<256; a++) {
            int b = a + rng->next_i32_bound(256 - a);
            uint8_t temp = this->map[a];
            this->map[a] = this->map[b];
            this->map[b] = temp;
        }
    }

    Gpu float get_map(int index) {
        return this->map[index & 0xFF];
    }

    /// Sample the noise at a given coordinate
    /// - Note: For monoliths, y is often 0.0
    Gpu float sample(float x, float y, float z) {
        x += this->xoff;
        y += this->yoff;
        z += this->zoff;

        // Convert to grid coordinates (512 length)
        int xi = __float2int_rd(x) & 0xFF;
        int yi = __float2int_rd(y) & 0xFF;
        int zi = __float2int_rd(z) & 0xFF;

        // Get the fractional parts
        float xf = x - floor(x);
        float yf = y - floor(y);
        float zf = z - floor(z);

        // Smoothstep-like factors
        float u = fade(xf);
        float v = fade(yf);
        float w = fade(zf);

        // Get the hash values for the corners
        int a  = this->get_map(xi + 0 + 0);
        int aa = this->get_map(yi + a + 0);
        int ab = this->get_map(yi + a + 1);
        int b  = this->get_map(xi + 0 + 1);
        int ba = this->get_map(yi + b + 0);
        int bb = this->get_map(yi + b + 1);

        return lerp(w,
            lerp(v,
                lerp(u, grad(this->get_map(aa + zi), xf, yf, zf),
                        grad(this->get_map(ba + zi), xf - 1.0, yf, zf)),
                lerp(u, grad(this->get_map(ab + zi), xf, yf - 1.0, zf),
                        grad(this->get_map(bb + zi), xf - 1.0, yf - 1.0, zf))),
            lerp(v,
                lerp(u, grad(this->get_map(aa + zi + 1), xf, yf, zf - 1.0),
                        grad(this->get_map(ba + zi + 1), xf - 1.0, yf, zf - 1.0)),
                lerp(u, grad(this->get_map(ab + zi + 1), xf, yf - 1.0, zf - 1.0),
                        grad(this->get_map(bb + zi + 1), xf - 1.0, yf - 1.0, zf - 1.0))));
    }

    /// Roll the generator state that would have created a PerlinNoise
    /// - Fast way around without as many memory operations
    Gpu static void discard(JavaRNG* rng, int count) {

        // Gotta love magic numbers!
        #if SKIP_TABLE
            rng->state *= 249870891710593LL;
            rng->state += 44331453843488LL;
            rng->state &= M;
            return;
        #endif

        for (int i=0; i<count; i++) {

            // Coordinates f64 offsets
            for (int j=0; j<3; j++) {
                rng->next_f64();
            }

            // Permutations swapping
            for (int max=256; max>=1; max--) {
                rng->next_i32_bound(max);
            }
        }
    }
};

/* -------------------------------------------------------------------------- */

template<int OCTAVES> struct FractalPerlin {
    PerlinNoise noise[OCTAVES];

    Gpu void init(JavaRNG* rng) {
        for (int i=0; i<OCTAVES; i++) {
            this->noise[i].init(rng);
        }
    }

    Gpu float sample(float x, float y, float z) {
        float sum = 0.0f;
        for (int i=0; i<OCTAVES; i++) {
            int   j = OCTAVES - 1 - i;
            float s = (float) (1 << j);
            sum += this->noise[j].sample(x/s, y/s, z/s) * s;
        }
        return sum;
    }
};

/* -------------------------------------------------------------------------- */

struct World {
    FractalPerlin<10> hill;
    FractalPerlin<16> depth;

    Gpu void init(uint64_t seed) {
        JavaRNG rng(seed);

        // Skip 48 generators priorly used elsewhere
        PerlinNoise::discard(&rng, 48);

        this->hill.init(&rng);
        this->depth.init(&rng);
    }

    // Check if a given coordinate is part of a monolith
    Gpu bool is_monolith(int64_t x, int64_t z) {
        float depth = this->depth.sample(
            (float) (x/4) * 100.0, 0.0,
            (float) (z/4) * 100.0
        );

        if (fabs(depth) < 8000.0)
            return false;

        float hill = this->hill.sample(
            (float) (x/4) * 1.0, 0.0,
            (float) (z/4) * 1.0
        );

        return hill < -512.0;
    }

    Gpu bool around_spawn(int64_t radius, int64_t step) {
        for (int x=-radius; x<=radius; x+=step) {
            for (int z=-radius; z<=radius; z+=step) {
                if (this->is_monolith(x, z)) {
                    return true;
                }
            }
        }
        return false;
    }
};

/* -------------------------------------------------------------------------- */

__global__ void get_monoliths_world_per_block(
    int start, int seeds,
    float* results
) {
    int idx  = threadIdx.x;
    int dim  = blockDim.x;
    int blk  = blockIdx.x;
    int seed = start + blk;

    __shared__ World world;

    if (threadIdx.x == 0) {
        world.init(seed);

        if (blockIdx.x % 1000 == 0)
            printf("Block %d seed %d\n", blk, seed);

        if (!world.around_spawn(200, 100))
            return;
    }

    __syncthreads();

    int64_t side = 4096;
    int64_t step = 32;
    float   area = 0;

    // Each thread sums its strip
    for (int64_t x=-side+idx; x<=side; x+=step*dim) {
        for (int64_t z=-side; z<=side; z+=step) {
            area += world.is_monolith(x, z) ? step*step : 0.0f;
        }
    }

    atomicAdd(&results[blk], area);
}

__global__ void get_monoliths_world_per_thread(
    int start, int seeds,
    float* results
) {
    int tdx  = threadIdx.x;
    int dim  = blockDim.x;
    int blk  = blockIdx.x;
    int tid  = (blk * dim) + tdx;
    int seed = start + tid;

    World world;
    world.init(seed);

    if (tid % 10000 == 0)
        printf("Block %d seed %d\n", blk, seed);

    if (!world.around_spawn(200, 200))
        return;

    int64_t side = 256;
    int64_t step = 4;

    for (int64_t x=-side; x<=side; x+=step) {
        for (int64_t z=-side; z<=side; z+=step) {
            results[tid] += world.is_monolith(x, z) ? step*step : 0.0f;
        }
    }
}

/* -------------------------------------------------------------------------- */

enum Variant {
    WORLD_PER_THREAD,
    WORLD_PER_BLOCK,
};

int main() {
    int start  = 0;
    int seeds  = 10000000;
    int thread = 64;

    float* d_results;
    cudaMalloc(&d_results, seeds * sizeof(float));

    Variant variant = WORLD_PER_THREAD;
    // Variant variant = WORLD_PER_BLOCK;

    if (variant == WORLD_PER_THREAD) {
        int blocks = (seeds + thread - 1) / thread;
        get_monoliths_world_per_thread<<<blocks, thread>>>(start, seeds, d_results);
    } else if (variant == WORLD_PER_BLOCK) {
        int blocks = seeds;
        get_monoliths_world_per_block<<<blocks, thread>>>(start, seeds, d_results);
    }

    cudaDeviceSynchronize();

    cudaError_t err = cudaGetLastError();
    if (err != cudaSuccess) {
        fprintf(stderr, "CUDA error: %s\n", cudaGetErrorString(err));
        return -1;
    }

    // Fixme: Make a iterative loop searching chunks of N seeds
    float* results = new float[seeds];
    cudaMemcpy(results, d_results, seeds * sizeof(float), cudaMemcpyDeviceToHost);
    cudaFree(d_results);

    // Print findings
    for (int i=0; i<seeds; i++) {
        if (results[i] > 0) {
            printf("Seed %llu area: %f\n", start + i, results[i]);
        }
    }

    return 0;
}