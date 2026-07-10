use rand::Rng;

pub const INFINITY: f64 = f64::INFINITY;
#[allow(dead_code)]
pub const PI: f64 = std::f64::consts::PI;

#[inline]
#[allow(dead_code)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
#[inline]
#[allow(dead_code)]
pub fn random_double() -> f64 {
    rand::random::<f64>()
}
#[inline]
#[allow(dead_code)]
pub fn random_double_in_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
#[inline]
#[allow(dead_code)]
pub fn random_int_range(min: i32, max: i32) -> i32 {
    random_double_in_range(min as f64, (max + 1) as f64) as i32
}