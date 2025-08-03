
// Round an integer to a nearest multiple of another
pub fn nearest(num: i64, mul: i64) -> i64 {
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

/// Computes the dot product between a pseudorandom
/// gradient vector and the distance vector
#[inline(always)]
pub fn grad(hash: u8, x: f64, y: f64, z: f64) -> f64 {
    let h = hash & 0x0F;
    let u = if h < 8 {x} else {y};
    let v = if h < 4 {y} else if h == 12 || h == 14 {x} else {z};
    let u = if h & 1 == 0 {u} else {-u};
    let v = if h & 2 == 0 {v} else {-v};
    return u + v;
}
