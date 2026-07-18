use crate::hittable::Hittable;
use crate::onb::Onb;
use crate::rtweekend::{PI, random_double};
use crate::vec3::{Point3, Vec3, dot, random_cosine_direction, random_unit_vector, unit_vector};
use std::sync::Arc;

#[allow(dead_code)]
pub trait Pdf {
    fn value(&self, _direction: Vec3) -> f64 {
        0.0
    }
    fn generate(&self) -> Vec3 {
        Vec3::new_vec3(0.0, 0.0, 0.0)
    }
}

#[derive(Default)]
pub struct EmptyPdf;

impl Pdf for EmptyPdf {
    fn value(&self, _direction: Vec3) -> f64 {
        0.0
    }
    fn generate(&self) -> Vec3 {
        Vec3::new_vec3(0.0, 0.0, 0.0)
    }
}

pub struct SpherePDF;

impl SpherePDF {
    pub fn new() -> Self {
        SpherePDF
    }
}
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
    #[allow(dead_code)]
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

pub struct HittablePDF {
    objects: Arc<dyn Hittable>,
    origin: Point3,
}

impl HittablePDF {
    pub fn new(objects: Arc<dyn Hittable>, origin: Point3) -> Self {
        Self { objects, origin }
    }
}

impl Pdf for HittablePDF {
    fn value(&self, direction: Vec3) -> f64 {
        self.objects.pdf_value(self.origin, direction)
    }
    fn generate(&self) -> Vec3 {
        self.objects.random(self.origin)
    }
}

// 修改 MixturePDF：使用 Arc<dyn Pdf>，去掉生命周期
pub struct MixturePDF {
    p0: Arc<dyn Pdf>,
    p1: Arc<dyn Pdf>,
}

impl MixturePDF {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self { p0, p1 }
    }
}

impl Pdf for MixturePDF {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }
    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
