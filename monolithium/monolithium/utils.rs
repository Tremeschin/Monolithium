use crate::*;

/// Round an integer to a nearest multiple of another
pub fn nearest(num: i32, mul: i32) -> i32 {
    (num + mul/2) / mul * mul
}

/// Standard linear interpolation function
#[inline(always)]
pub fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}

/// Common progress bar style
pub fn progress(message: &str) -> ProgressStyle {
    ProgressStyle::default_bar().template(
        &format!("{message} ({{elapsed_precise}} • ETA {{eta_precise}}) {{wide_bar:.cyan/blue}} ({{percent_precise}}%) • {{pos}}/{{len}} ({{per_sec:0.}})")).unwrap()
        .progress_chars("##•")
}
