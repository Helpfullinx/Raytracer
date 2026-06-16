use rand::RngExt;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

#[inline]
pub fn random_f64() -> f64 {
    rand::rng().random_range(0.0..1.0)
}

#[inline]
pub fn random_float_range(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..max)
}