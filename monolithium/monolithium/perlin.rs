use crate::*;

#[derive(Clone, Debug)]
pub struct Perlin {
    /// Permutations map (Vector -> Grid)
    pub map: [u8; 256],
    pub xoff: f64,
    pub yoff: f64,
    pub zoff: f64,
}

/* -------------------------------------------------------------------------- */

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            map: [0; 256],
            xoff: 0.0,
            yoff: 0.0,
            zoff: 0.0,
        }
    }

    #[inline(always)]
    pub fn init(&mut self, rng: &mut JavaRNG) {
        self.xoff = rng.next_f64() * 256.0;
        self.yoff = rng.next_f64() * 256.0;
        self.zoff = rng.next_f64() * 256.0;

        // Start a new 'arange' array
        unsafe {
            for i in 0..256 {
                *self.map.get_unchecked_mut(i) = i as u8;
            }
        }

        // Shuffle the array
        unsafe {
            let ptr = self.map.as_mut_ptr();

            for a in 0..256 {
                let max = (256 - a) as i32;
                let b = rng.next_i32_bound(max) as usize;
                std::ptr::swap(ptr.add(a), ptr.add(a + b));
            }
        }
    }

    #[inline(always)]
    fn get_map(&self, index: usize) -> u8 {
        unsafe {*self.map.get_unchecked(index & 0xFF)}
    }

    /// Get the gradient vector at a given grid coordinate
    #[inline(always)]
    pub fn grid_gradient(&self,
        grid_x: usize,
        grid_y: usize,
        grid_z: usize,
    ) -> (f64, f64, f64) {
        let xi = grid_x & 0xFF;
        let yi = grid_y & 0xFF;
        let zi = grid_z & 0xFF;
        let a  = self.get_map(xi +  0) as usize;
        let aa = self.get_map(yi +  a) as usize;
        let sh = self.get_map(aa + zi) as usize;
        unsafe {*GRAD_LOOKUP.get_unchecked(sh & 0x0F)}
    }

    /// Sample the noise at a given coordinate
    /// - Note: For monoliths, y is often 0.0
    pub fn sample(&self, x: f64, y: f64, z: f64) -> f64 {
        use utils::fade;
        use utils::grad;
        use utils::lerp;

        // Apply offsets
        let x: f64 = x + self.xoff;
        let y: f64 = y + self.yoff;
        let z: f64 = z + self.zoff;

        // Convert to grid coordinates (256 length)
        let xi: usize = (x.floor() as i32 & 0xFF) as usize;
        let yi: usize = (y.floor() as i32 & 0xFF) as usize;
        let zi: usize = (z.floor() as i32 & 0xFF) as usize;

        // Get the fractional parts
        let xf: f64 = x - x.floor();
        let yf: f64 = y - y.floor();
        let zf: f64 = z - z.floor();

        // Smoothstep-like factors
        let u: f64 = fade(xf);
        let v: f64 = fade(yf);
        let w: f64 = fade(zf);

        // Get the hash values for the corners
        let a  = self.get_map(xi + 0 + 0) as usize;
        let aa = self.get_map(yi + a + 0) as usize;
        let ab = self.get_map(yi + a + 1) as usize;
        let b  = self.get_map(xi + 1 + 0) as usize;
        let ba = self.get_map(yi + b + 0) as usize;
        let bb = self.get_map(yi + b + 1) as usize;

        // Interpolate corner values relative to sample point
        return lerp(w,
            lerp(v,
                lerp(u, grad(self.get_map(aa + zi), xf,       yf, zf),
                        grad(self.get_map(ba + zi), xf - 1.0, yf, zf)),
                lerp(u, grad(self.get_map(ab + zi), xf,       yf - 1.0, zf),
                        grad(self.get_map(bb + zi), xf - 1.0, yf - 1.0, zf))
            ),
            lerp(v,
                lerp(u, grad(self.get_map(aa + zi + 1), xf,       yf, zf - 1.0),
                        grad(self.get_map(ba + zi + 1), xf - 1.0, yf, zf - 1.0)),
                lerp(u, grad(self.get_map(ab + zi + 1), xf,       yf - 1.0, zf - 1.0),
                        grad(self.get_map(bb + zi + 1), xf - 1.0, yf - 1.0, zf - 1.0))
            ),
        );
    }

    /// Roll the generator state that would have created a PerlinNoise
    /// - Fast way around without as many memory operations
    pub fn discard(rng: &mut JavaRNG, many: usize) {

        // Super fast but slightly lossy
        if cfg!(feature="skip-table") {
            rng.step_n(many*(3*2 + 256));
            return;
        }

        for _ in 0..many {

            // Coordinates f64 offsets
            for _ in 0..3 {
                rng.step();
                rng.step();
            }

            // Permutations swapping
            for max in (1..=256).rev() {
                rng.next_i32_bound(max);
            }
        }
    }
}

/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug)]
pub struct FractalPerlin<const OCTAVES: usize> {
    pub noise: [Perlin; OCTAVES],
}

/// Ignored count of lower octaves, as they contribute very little
const OCTAVES_START: usize = if cfg!(feature="most-octaves") {3} else {0};

/// Lookup table for octave amplitudes
static OCTAVE_SCALE_MUL: [f64; 32] = {
    let mut array = [0.0; 32];
    let mut i = 0;
    while i < 32 {
        array[i] = (1 << i) as f64;
        i += 1;
    }
    array
};

/// Lookup table for octave amplitudes (inverse)
static OCTAVE_SCALE_DIV: [f64; 32] = {
    let mut array = [0.0; 32];
    let mut i = 0;
    while i < 32 {
        array[i] = 1.0 / (1 << i) as f64;
        i += 1;
    }
    array
};

impl<const OCTAVES: usize> FractalPerlin<OCTAVES> {

    #[inline(always)]
    pub fn new() -> Self {
        FractalPerlin {
            noise: std::array::from_fn(|_| Perlin::new())
        }
    }

    #[inline(always)]
    pub fn init(&mut self, rng: &mut JavaRNG) {
        for i in 0..OCTAVES {
            self.noise[i].init(rng);
        }
    }

    /// Sample the fractal noise at a given coordinate
    #[inline(always)]
    pub fn sample(&self, x: f64, z: f64) -> f64 {
        (OCTAVES_START..OCTAVES).map(|i| {
            let mul = Self::octave_scale_mul_f64(i);
            let div = Self::octave_scale_div_f64(i);
            self.noise[i].sample(x*div, 0.0, z*div) * mul
        }).sum()
    }

    /// Value at which the noise wraps around and repeats.
    /// - For Perlin noise, this value is 256 without any scaling
    /// - Each octave halves the frequency, extending it
    #[inline(always)]
    pub fn repeats(&self) -> usize {
        256 * (1 << (OCTAVES - 1))
    }

    /// The maximum value a given octave can produce
    #[inline(always)]
    pub fn octave_scale_mul_f64(octave: usize) -> f64 {
        unsafe {*OCTAVE_SCALE_MUL.get_unchecked(octave)}
    }

    /// The maximum value a given octave can produce (inverse)
    #[inline(always)]
    pub fn octave_scale_div_f64(octave: usize) -> f64 {
        unsafe {*OCTAVE_SCALE_DIV.get_unchecked(octave)}
    }

    // Usual maximum value of the noise
    #[inline(always)]
    pub fn maxval(&self) -> f64 {
       Self::octave_scale_mul_f64(OCTAVES)
    }

    // When all stars align, you get a girlfriend
    // and a really big perlin noise value
    #[inline(always)]
    pub fn tmaxval(&self) -> f64 {
        (0..=OCTAVES).map(|n| {
            Self::octave_scale_mul_f64(n)
        }).sum()
    }
}

/* -------------------------------------------------------------------------- */

/// Most coordinates are nowhere close to being monoliths, optimization to
/// discard sums where reaching a target with next octave is impossible
impl<const OCTAVES: usize> FractalPerlin<OCTAVES> {

    #[inline(always)]
    pub fn is_hill_monolith(&self, x: i32, z: i32) -> bool {
        const TARGET: f64 = -512.0;
        let x = (x >> 2) as f64;
        let z = (z >> 2) as f64;
        let mut sum = 0.0;

        // Start from most influential octaves
        for octave in (OCTAVES_START..OCTAVES).rev() {
            let mul = Self::octave_scale_mul_f64(octave);
            let div = Self::octave_scale_div_f64(octave);
            sum += self.noise[octave].sample(x*div, 0.0, z*div) * mul;

            // Next octaves cannot possibly reach target
            if sum - mul > TARGET {
                return false;
            }

            // Next octaves will definitely reach target
            if sum + mul < TARGET {
                return true;
            }
        }

        sum < TARGET
    }

    #[inline(always)]
    pub fn is_depth_monolith(&self, x: i32, z: i32) -> bool {
        const TARGET: f64 = 8000.0;
        let x = (25 * x) as f64;
        let z = (25 * z) as f64;
        let mut sum = 0.0;

        // Start from most influential octaves
        for octave in (OCTAVES_START..OCTAVES).rev() {
            let mul = Self::octave_scale_mul_f64(octave);
            let div = Self::octave_scale_div_f64(octave);
            sum += self.noise[octave].sample(x*div, 0.0, z*div) * mul;

            // Next octaves cannot possibly reach target
            if (sum.abs() + mul) < TARGET {
                return false;
            }

            // Next octaves will definitely reach target
            else if (sum.abs() - mul) > TARGET {
                return true;
            }
        }

        sum.abs() > TARGET
    }
}
