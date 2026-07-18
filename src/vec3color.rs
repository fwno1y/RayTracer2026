use crate::interval::Interval;
use crate::vec3::Vec3;
pub type Color = Vec3;
#[allow(dead_code)]
pub fn linear_to_gemma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}
#[allow(dead_code)]
pub fn write_color(pixel_color: &Color) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
    if r.is_nan() {
        r = 0.0;
    }
    if g.is_nan() {
        g = 0.0;
    }
    if b.is_nan() {
        b = 0.0;
    }
    r = linear_to_gemma(r);
    g = linear_to_gemma(g);
    b = linear_to_gemma(b);
    let intensity = Interval::new(0.000, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as u8;
    let gbyte = (256.0 * intensity.clamp(g)) as u8;
    let bbyte = (256.0 * intensity.clamp(b)) as u8;
    println!("{} {} {}", rbyte, gbyte, bbyte)
}
