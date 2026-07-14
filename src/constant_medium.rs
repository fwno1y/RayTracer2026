use crate::hittable::{HitRecord, Hittable};
use std::rc::Rc;

use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::rtweekend::{INFINITY, random_double};
use crate::texture::Texture;
use crate::vec3::Vec3;
use crate::vec3color::Color;

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Rc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Rc<dyn Hittable>, density: f64, tex: Rc<dyn Material>) -> ConstantMedium {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: tex,
        }
    }
    #[allow(dead_code)]
    pub fn from_texture(
        boundary: Rc<dyn Hittable>,
        density: f64,
        tex: Rc<dyn Texture>,
    ) -> ConstantMedium {
        let phase = Rc::new(Isotropic::new(tex));
        Self::new(boundary, density, phase)
    }
    pub fn from_color(boundary: Rc<dyn Hittable>, density: f64, albedo: Color) -> ConstantMedium {
        let phase = Rc::new(Isotropic::from_color(albedo));
        Self::new(boundary, density, phase)
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();
        if !self.boundary.hit(r, Interval::UNIVERSE, &mut rec1) {
            return false;
        }
        if !self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, INFINITY), &mut rec2)
        {
            return false;
        }
        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }
        if rec1.t >= rec2.t {
            return false;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }
        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();
        if hit_distance > distance_inside_boundary {
            return false;
        }
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);
        rec.normal = Vec3::new_vec3(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = Some(self.phase_function.clone());
        true
    }
    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
