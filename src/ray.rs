use crate::vec3::{Point3, Vec3};
#[allow(dead_code)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f64,
}
impl Ray {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            orig: Point3::new_vec3(0.0, 0.0, 0.0),
            dir: Vec3::new_vec3(1.0, 0.0, 0.0),
            tm: 0.0,
        }
    }
    pub fn new_ray(origin: Point3, direction: Vec3, time: f64) -> Self {
        Ray {
            orig: origin,
            dir: direction,
            tm: time,
        }
    }
    #[allow(dead_code)]
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn time(&self) -> f64 {
        self.tm
    }
    #[allow(dead_code)]
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
