use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{dot, random_unit_vector, reflect, unit_vector};
use crate::vec3color::Color;

#[allow(dead_code)]
pub trait Material {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
}
pub struct Lambertian {
    pub albedo: Color,
}
impl Lambertian {
    #[allow(dead_code)]
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new_ray(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}
impl Metal {
    #[allow(dead_code)]
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut reflected = reflect(r_in.direction(), rec.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        let scattered = Ray::new_ray(rec.p, reflected);
        let attenuation = self.albedo;
        if dot(scattered.direction(), rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
