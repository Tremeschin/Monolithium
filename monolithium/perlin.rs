use crate::*;

#[derive(Clone)]
pub struct PerlinNoise {
    /// Permutations map (Vector -> Grid)
    pub map: [u8; 512],
    pub xoff: f64,
    pub yoff: f64,
    pub zoff: f64,
}

/* -------------------------------------------------------------------------- */

impl PerlinNoise {
    pub fn new() -> Self {
        PerlinNoise {
            map: [0; 512],
            xoff: 0.0,
            yoff: 0.0,
            zoff: 0.0,
        }
    }

    pub fn init(&mut self, rng: &mut JavaRNG) {
        self.xoff = rng.next_f64() * 256.0;
        self.yoff = rng.next_f64() * 256.0;
        self.zoff = rng.next_f64() * 256.0;

        // Start a new 'arange' array
        for i in 0..512 {
            self.map[i] = i as u8;
        }

        // Shuffle the first half
        for a in 0..256 {
            let b = rng.next_i32_bound((256 - a) as i32) as usize;
            self.map.swap(a, a + b);
        }

        // Mirror to the second half (faster than for loop)
        unsafe {std::ptr::copy_nonoverlapping(
            self.map.as_ptr(),
            self.map.as_mut_ptr().add(256),
            256
        )};
    }

    /// Sample the noise at a given coordinate
    /// - Note: For monoliths, y is often 0.0
    pub fn sample(&self, x: f64, y: f64, z: f64) -> f64 {

        // Apply offsets
        let x: f64 = x + self.xoff;
        let y: f64 = y + self.yoff;
        let z: f64 = z + self.zoff;

        // Convert to grid coordinates (512 length)
        let xi = (x.floor() as i32 & 0xFF) as usize;
        let yi = (y.floor() as i32 & 0xFF) as usize;
        let zi = (z.floor() as i32 & 0xFF) as usize;

        // Get the fractional parts
        let xf: f64 = x - x.floor();
        let yf: f64 = y - y.floor();
        let zf: f64 = z - z.floor();

        // Smoothstep-like factors
        let u = utils::fade(xf);
        let v = utils::fade(yf);
        let w = utils::fade(zf);

        // Get the hash values for the corners
        let a  = self.map[xi + 0 + 0] as usize;
        let aa = self.map[yi + a + 0] as usize;
        let ab = self.map[yi + a + 1] as usize;
        let b  = self.map[xi + 0 + 1] as usize;
        let ba = self.map[yi + b + 0] as usize;
        let bb = self.map[yi + b + 1] as usize;

        // Interpolate corner values relative to sample point
        return utils::lerp(w,
            utils::lerp(v, utils::lerp(u,
                utils::grad(self.map[aa + zi], xf,       yf, zf),
                utils::grad(self.map[ba + zi], xf - 1.0, yf, zf),
            ), utils::lerp(u,
                utils::grad(self.map[ab + zi], xf,       yf - 1.0, zf),
                utils::grad(self.map[bb + zi], xf - 1.0, yf - 1.0, zf),
            )),
            utils::lerp(v, utils::lerp(u,
                utils::grad(self.map[aa + zi + 1], xf,       yf, zf - 1.0),
                utils::grad(self.map[ba + zi + 1], xf - 1.0, yf, zf - 1.0),
            ), utils::lerp(u,
                utils::grad(self.map[ab + zi + 1], xf,       yf - 1.0, zf - 1.0),
                utils::grad(self.map[bb + zi + 1], xf - 1.0, yf - 1.0, zf - 1.0),
            )),
        );
    }

    /// Roll the generator state that would have created a PerlinNoise
    /// - Fast way around without as many memory operations
    pub fn discard(rng: &mut JavaRNG, many: usize) {
        for _ in 0..many {

            // Coordinates f64 offsets
            for _ in 0..3 {
                rng.step();
                rng.step();
            }

            // Permutations swapping
            for max in (1..=256).rev() {
                if cfg!(feature="strict") {
                    rng.next_i32_bound(max as i32);

                // Monte Carlo says 0.03474% error rate
                } else if cfg!(feature="lossy") {
                    rng.step()

                } else {
                    rng.skip_next_i32_bound(max as i32);
                }
            }
        }
    }
}

/* -------------------------------------------------------------------------- */

#[derive(Clone)]
pub struct FractalPerlin<const OCTAVES: usize> {
    pub noise: [PerlinNoise; OCTAVES],
}

impl<const OCTAVES: usize> FractalPerlin<OCTAVES> {
    pub fn new() -> Self {
        FractalPerlin {
            noise: std::array::from_fn(|_| PerlinNoise::new())
        }
    }

    pub fn init(&mut self, rng: &mut JavaRNG) {
        for i in 0..OCTAVES {
            self.noise[i].init(rng);
        }
    }

    /// Sample the fractal noise at a given coordinate
    pub fn sample(&self, x: f64, z: f64) -> f64 {
        (0..OCTAVES).map(|i| {
            let s = self.octave_scale(i);
            self.noise[i].sample(x/s, 0.0, z/s) * s
        }).sum()
    }

    /// Value at which the noise wraps around and repeats.
    /// - For Perlin noise, this value is 256 without any scaling
    /// - Each octave halves the frequency, extending it
    pub fn repeats(&self) -> usize {
        256 * (1 << (OCTAVES - 1))
    }

    /// The maximum value a given octave can produce
    pub fn octave_scale(&self, octave: usize) -> f64 {
        (1 << octave) as f64
    }

    // Usual maximum value of the noise
    pub fn maxval(&self) -> f64 {
       self.octave_scale(OCTAVES)
    }

    // When all stars align, you get a girlfriend
    // and a really big perlin noise value
    pub fn tmaxval(&self) -> f64 {
        (0..=OCTAVES).map(|n| {
            self.octave_scale(n)
        }).sum()
    }
}

/* -------------------------------------------------------------------------- */

pub enum SmartSample {
    Depth,
    Hill,
}

impl<const OCTAVES: usize> FractalPerlin<OCTAVES> {

    /// Most coordinates are nowhere close to being monoliths, staging
    /// optimization to discard sums where reaching a target is impossible
    pub fn smart_sample(&self, x: f64, z: f64, kind: SmartSample) -> bool {
        let mut sum = 0.0;

        for i in (0..OCTAVES).rev() {
            let s = self.octave_scale(i);
            sum  += self.noise[i].sample(x/s, 0.0, z/s) * s;

            if match kind {
                SmartSample::Depth =>
                    sum.abs() + 0.5*self.octave_scale(i) < 8000.0,
                SmartSample::Hill =>
                    sum - 0.5*self.octave_scale(i) > -512.0
            } {
                return false;
            }
        }

        match kind {
            SmartSample::Depth => sum.abs() > 8000.0,
            SmartSample::Hill  => sum < -512.0,
        }
    }
}
