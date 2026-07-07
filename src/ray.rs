use crate::vec3::{Vec3, Point3};
pub struct Ray {
    orig : Point3,
    dir : Vec3,
}
impl Ray {
    pub fn new() -> Self {
        Self {
            orig : Point3::vec3(0.0, 0.0, 0.0),
            dir : Vec3::vec3(1.0, 0.0, 0.0),
        }
    }
    pub fn ray(orig: Point3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}