use crate::onb::Onb;
use crate::rtweekend::PI;
use crate::vec3::{Vec3, dot, random_cosine_direction, random_unit_vector, unit_vector};

#[allow(dead_code)]
pub trait Pdf {
    fn value(&self, _direction: Vec3) -> f64 {
        0.0
    }
    fn generate(&self) -> Vec3 {
        Vec3::new_vec3(0.0, 0.0, 0.0)
    }
}

pub struct SpherePDF {}

impl Pdf for SpherePDF {
    fn value(&self, _direction: Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }
    fn generate(&self) -> Vec3 {
        random_unit_vector()
    }
}

pub struct CosinePDF {
    pub uvw: Onb,
}

impl CosinePDF {
    pub fn new(w: Vec3) -> CosinePDF {
        CosinePDF { uvw: Onb::new(w) }
    }
}
impl Pdf for CosinePDF {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine_theta = dot(unit_vector(direction), self.uvw.w());
        cosine_theta / PI.max(0.0)
    }
    fn generate(&self) -> Vec3 {
        self.uvw.transform(random_cosine_direction())
    }
}
