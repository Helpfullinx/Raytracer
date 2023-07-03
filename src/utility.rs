pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

#[inline]
pub fn degrees_to_radians(degrees: f32) {
    degrees * PI / 180.0;
}