use crate::vec3::{Vec3, cross, unit_vector};
#[allow(dead_code)]
pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    #[allow(dead_code)]
    pub fn new(n: Vec3) -> Self {
        let w = unit_vector(n);
        let a = if w.x.abs() > 0.9 {
            Vec3::new_vec3(0.0, 1.0, 0.0)
        } else {
            Vec3::new_vec3(1.0, 0.0, 0.0)
        };
        let v = unit_vector(cross(w, a));
        let u = cross(w, v);
        Self { axis: [u, w, v] }
    }
    #[allow(dead_code)]
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    #[allow(dead_code)]
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    #[allow(dead_code)]
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }
    #[allow(dead_code)]
    pub fn transform(&self, v: &Vec3) -> Vec3 {
        v[0] * self.axis[0] + v[1] * self.axis[1] + v[2] * self.axis[2]
    }
}
