use crate::*;

/// Round an integer to a nearest multiple of another
pub fn nearest(num: i32, mul: i32) -> i32 {
    (num + mul/2) / mul * mul
}

/// Similar function to a smoothstep, specific for perlin
/// - https://en.wikipedia.org/wiki/Smoothstep
#[inline(always)]
pub fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Standard linear interpolation function
#[inline(always)]
pub fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}

pub const GRAD_LOOKUP: [(f64, f64, f64); 16] = [
    ( 1.0,  1.0,  0.0), //  0:  x + y
    (-1.0,  1.0,  0.0), //  1: -x + y
    ( 1.0, -1.0,  0.0), //  2:  x - y
    (-1.0, -1.0,  0.0), //  3: -x - y
    ( 1.0,  0.0,  1.0), //  4:  x + z
    (-1.0,  0.0,  1.0), //  5: -x + z
    ( 1.0,  0.0, -1.0), //  6:  x - z
    (-1.0,  0.0, -1.0), //  7: -x - z
    ( 0.0,  1.0,  1.0), //  8:  y + z
    ( 0.0, -1.0,  1.0), //  9: -y + z
    ( 0.0,  1.0, -1.0), // 10:  y - z
    ( 0.0, -1.0, -1.0), // 11: -y - z
    ( 1.0,  1.0,  0.0), // 12:  y + x
    ( 0.0, -1.0,  1.0), // 13: -y + z
    (-1.0,  1.0,  0.0), // 14:  y - x
    ( 0.0, -1.0, -1.0), // 15: -y - z
];

/// Computes the dot product between a pseudorandom
/// gradient vector and the distance vector
#[inline(always)]
pub fn grad(hash: u8, x: f64, y: f64, z: f64) -> f64 {
    if cfg!(feature="grad-lookup") {
        unsafe {
            let (cx, cy, cz) = GRAD_LOOKUP.get_unchecked(hash as usize & 0x0F);
            return (cx * x) + (cy * y) + (cz * z);
        }
    } else {
        let h = hash & 0x0F;
        let u = if h < 8 {x} else {y};
        let v = if h < 4 {y} else if h == 12 || h == 14 {x} else {z};
        let u = if h & 1 == 0 {u} else {-u};
        let v = if h & 2 == 0 {v} else {-v};
        return u + v;
    }
}

/// Common progress bar style
pub fn progress(message: &str) -> ProgressStyle {
    ProgressStyle::default_bar().template(
        &format!("{message} ({{elapsed_precise}} • ETA {{eta_precise}}) {{wide_bar:.cyan/blue}} ({{percent_precise}}%) • {{pos}}/{{len}} ({{per_sec:0.}})")).unwrap()
        .progress_chars("##•")
}
