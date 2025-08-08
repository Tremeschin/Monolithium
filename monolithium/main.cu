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
// Or install [uv](https://docs.astral.sh/uv/):
//   - Run: `uv run cuda`
/* -------------------------------------------------------------------------- */

#include <cmath>
#include <cstdint>
#include <cstdio>

#include <cuda_runtime.h>

#define Any __global__
#define Gpu __device__
#define Cpu __host__

/* -------------------------------------------------------------------------- */
// Utility functions

Gpu double fade(double t) {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

Gpu double lerp(double t, double a, double b) {
    return a + t * (b - a);
}

Gpu double grad(uint8_t hash, double x, double y, double z) {
    int h = hash & 15;
    double u = h < 8 ? x : y;
    double v = h < 4 ? y : h == 12 || h == 14 ? x : z;
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

    Gpu JavaRNG(uint64_t seed) {
        this->state = ((int64_t) seed ^ A) & M;
    }

    Gpu void step() {
        this->state = (this->state * A + C) & M;
    }

    Gpu int32_t next(uint8_t bits) {
        this->step();
        return ((uint64_t) this->state >> (48 - bits));
    }

    Gpu int32_t next_i32_bound(int32_t max) {
        if (__popc(max) == 1) {
            return (int32_t)(((int64_t) max * (int64_t) this->next(31)) >> 31);
        }
        int32_t next = this->next(31);
        int32_t take = next % max;
        while (next - take + max - 1 < 0) {
            next = this->next(31);
            take = next % max;
        }
        return take;
    }

    Gpu double next_f64() {
        int64_t high = ((int64_t) this->next(26)) << 27;
        int64_t low  =  (int64_t) this->next(27);
        return (double)(high + low) / F64_DIV;
    }
};

/* -------------------------------------------------------------------------- */

struct PerlinNoise {
    uint8_t map[512];
    double xoff;
    double yoff;
    double zoff;

    Gpu void init(JavaRNG& rng) {
        this->xoff = rng.next_f64() * 256.0;
        this->yoff = rng.next_f64() * 256.0;
        this->zoff = rng.next_f64() * 256.0;

        // Start a new 'arange' array
        for (int i=0; i<512; i++) {
            this->map[i] = i & 0xFF;
        }

        // Shuffle the first half
        for (int a=0; a<256; a++) {
            int b = a + rng.next_i32_bound(256 - a);
            uint8_t temp = this->map[a];
            this->map[a] = this->map[b];
            this->map[b] = temp;
        }

        // Mirror to the second half
        for (int i=0; i<256; i++) {
            this->map[i + 256] = this->map[i];
        }
    }

    /// Sample the noise at a given coordinate
    /// - Note: For monoliths, y is often 0.0
    Gpu double sample(double x, double y, double z) {
        x += this->xoff;
        y += this->yoff;
        z += this->zoff;

        // Convert to grid coordinates (512 length)
        int xi = ((int) floor(x)) & 0xFF;
        int yi = ((int) floor(y)) & 0xFF;
        int zi = ((int) floor(z)) & 0xFF;

        // Get the fractional parts
        double xf = x - floor(x);
        double yf = y - floor(y);
        double zf = z - floor(z);

        // Smoothstep-like factors
        double u = fade(xf);
        double v = fade(yf);
        double w = fade(zf);

        // Get the hash values for the corners
        int a  = this->map[xi + 0 + 0] + yi;
        int aa = this->map[yi + a + 0] + zi;
        int ab = this->map[yi + a + 1] + zi;
        int b  = this->map[xi + 0 + 1] + yi;
        int ba = this->map[yi + b + 0] + zi;
        int bb = this->map[yi + b + 1] + zi;

        return lerp(w,
            lerp(v,
                lerp(u, grad(this->map[aa], xf, yf, zf),
                        grad(this->map[ba], xf - 1.0, yf, zf)),
                lerp(u, grad(this->map[ab], xf, yf - 1.0, zf),
                        grad(this->map[bb], xf - 1.0, yf - 1.0, zf))),
            lerp(v,
                lerp(u, grad(this->map[aa + 1], xf, yf, zf - 1.0),
                        grad(this->map[ba + 1], xf - 1.0, yf, zf - 1.0)),
                lerp(u, grad(this->map[ab + 1], xf, yf - 1.0, zf - 1.0),
                        grad(this->map[bb + 1], xf - 1.0, yf - 1.0, zf - 1.0))));
    }

    /// Roll the generator state that would have created a PerlinNoise
    /// - Fast way around without as many memory operations
    Gpu static void discard(JavaRNG& rng, int count) {
        for (int i=0; i<count; i++) {

            // Coordinates f64 offsets
            for (int j=0; j<3; j++) {
                rng.step();
                rng.step();
            }

            // Permutations swapping
            for (int max=256; max>=1; max--) {
                if (__popc(max) == 1) {
                    rng.step();
                } else {
                    int32_t next = rng.next(31);
                    int32_t take = next % max;
                    while (next - take + max - 1 < 0) {
                        next = rng.next(31);
                        take = next % max;
                    }
                }
            }
        }
    }
};

/* -------------------------------------------------------------------------- */

template<int OCTAVES> struct FractalPerlin {
    PerlinNoise noise[OCTAVES];

    Gpu void init(JavaRNG& rng) {
        for (int i=0; i<OCTAVES; i++) {
            this->noise[i].init(rng);
        }
    }

    Gpu double sample(double x, double y, double z) {
        double sum = 0.0;
        for (int i=0; i<OCTAVES; i++) {
            int j = OCTAVES - 1 - i;
            double s = (double)(1 << j);
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
        PerlinNoise::discard(rng, 48);

        this->hill.init(rng);
        this->depth.init(rng);
    }

    // Check if a given coordinate is part of a monolith
    Gpu bool is_monolith(int64_t x, int64_t z) {
        double depth = this->depth.sample(
            (double) (x/4) * 100.0, 0.0,
            (double) (z/4) * 100.0
        );

        if (fabs(depth) < 8000.0)
            return false;

        double hill = this->hill.sample(
            (double) (x/4) * 1.0, 0.0,
            (double) (z/4) * 1.0
        );

        return hill < -512.0;
    }

    // The idea is to get the total area within a region, don't care
    // for position or overcounting, that's a CPU post filter duty
    Gpu int64_t count_monoliths() {
        int64_t count = 0;
        for (int64_t x=-200; x<=200; x+=4) {
            for (int64_t z=-200; z<=200; z+=4) {
                count += this->is_monolith(x, z) ? 16 : 0;
            }
        }
        return count;
    }
};

/* -------------------------------------------------------------------------- */

Any void detect_monoliths(uint64_t start, int64_t* results, int seeds) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= seeds) return;

    printf("Seed %llu\n", start + idx);

    World world;
    world.init(start + idx);
    results[idx] = world.count_monoliths();
}

/* -------------------------------------------------------------------------- */

int main() {
    const int seeds = 500000;
    uint64_t  start = 0;

    int64_t* d_results;
    cudaMalloc(&d_results, seeds * sizeof(int64_t));

    int thread = 128;
    int blocks = (seeds + thread - 1) / thread;

    printf("Launching %d blocks × %d threads = %d total threads\n",
           blocks, thread, blocks * thread);

    detect_monoliths<<<blocks, thread>>>(start, d_results, seeds);
    cudaDeviceSynchronize();
    printf("Done\n");

    cudaError_t err = cudaGetLastError();
    if (err != cudaSuccess) {
        fprintf(stderr, "CUDA error: %s\n", cudaGetErrorString(err));
        return -1;
    }

    // Fixme: Make a iterative loop searching chunks of N seeds
    int64_t* results = new int64_t[seeds];
    cudaMemcpy(results, d_results, seeds * sizeof(int64_t), cudaMemcpyDeviceToHost);
    cudaFree(d_results);

    // Fixme: All areas are zero
    for (int i=0; i<seeds; i++) {
        if (results[i] > 0) {
            printf("Seed %llu area: %lld\n", start + i, results[i]);
        }
    }

    delete[] results;
    return 0;
}