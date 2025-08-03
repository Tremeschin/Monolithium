use crate::*;

#[derive(Clone)]
pub struct PerlinNoise {
    /// Permutations map (Vector -> Grid)
    map: [u8; 512],
    xoff: f64,
    yoff: f64,
    zoff: f64,
}

/* -------------------------------------------------------------------------- */

impl PerlinNoise {
    pub fn new(rng: &mut Random) -> Self {
        let xoff = rng.next_f64() * 256.0;
        let yoff = rng.next_f64() * 256.0;
        let zoff = rng.next_f64() * 256.0;

        // Start a new 'arange' array
        let mut map = [0u8; 512];
        for i in 0..512 {
            map[i] = i as u8;
        }

        // Shuffle the first half
        for a in 0..256 {
            let b = rng.next_i32_bound(256 - a as i32) as usize + a;
            map.swap(a, b);
        }

        // Mirror to the second half
        for i in 0..256 {
            map[i + 256] = map[i];
        }

        PerlinNoise {
            map: map,
            xoff: xoff,
            yoff: yoff,
            zoff: zoff,
        }
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
}

/* -------------------------------------------------------------------------- */

#[derive(Clone)]
pub struct FractalPerlin<const OCTAVES: usize> {
    noise: [PerlinNoise; OCTAVES],
}

impl<const OCTAVES: usize> FractalPerlin<OCTAVES> {

    pub fn new(rng: &mut Random) -> Self {
        FractalPerlin {
            noise: std::array::from_fn(|_| PerlinNoise::new(rng))
        }
    }

    /// Sample the fractal noise at a given coordinate
    pub fn sample(&self, x: f64, y: f64, z: f64) -> f64 {
        (0..OCTAVES).map(|i| {
            let i = OCTAVES - 1 - i;
            let s = (1 << i) as f64;
            self.noise[i].sample(x/s, y/s, z/s) * s
        }).sum()
    }
}
