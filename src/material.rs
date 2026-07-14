use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rtweekend::random_double;
use crate::texture::{SolidColor, Texture};
use crate::vec3::{Point3, Vec3, dot, random_unit_vector, reflect, refract, unit_vector};
use crate::vec3color::Color;
use std::rc::Rc;

#[allow(dead_code)]
pub trait Material {
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new_vec3(0.0, 0.0, 0.0)
    }
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
}
pub struct Lambertian {
    tex: Rc<dyn Texture>,
}
impl Lambertian {
    #[allow(dead_code)]
    pub fn from_color(albedo: Color) -> Self {
        Lambertian {
            tex: Rc::new(SolidColor::new(albedo)),
        }
    }
    #[allow(dead_code)]
    pub fn new(tex: Rc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new_ray(rec.p, scatter_direction, _r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
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
        let scattered = Ray::new_ray(rec.p, reflected, r_in.time());
        let attenuation = self.albedo;
        if dot(scattered.direction(), rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refractive_index: f64,
}

impl Dielectric {
    #[allow(dead_code)]
    pub fn new(refraction_index: f64) -> Self {
        Dielectric {
            refractive_index: refraction_index,
        }
    }
    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new_vec3(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        #[allow(unused_assignments)]
        let mut direction = Vec3::new_vec3(0.0, 0.0, 0.0);
        if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            direction = reflect(unit_direction, rec.normal);
        } else {
            direction = refract(unit_direction, rec.normal, ri);
        }
        let scattered = Ray::new_ray(rec.p, direction, r_in.time());
        Some((attenuation, scattered))
    }
}

pub struct DiffuseLight {
    tex: Rc<dyn Texture>,
}

impl DiffuseLight {
    #[allow(dead_code)]
    pub fn new(tex: Rc<dyn Texture>) -> Self {
        DiffuseLight { tex }
    }
    #[allow(dead_code)]
    pub fn from_color(emit: Color) -> Self {
        DiffuseLight {
            tex: Rc::new(SolidColor::new(emit)),
        }
    }
}

impl Material for DiffuseLight {
    #[allow(dead_code)]
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.tex.value(u, v, p)
    }
}

pub struct Isotropic {
    tex: Rc<dyn Texture>,
}

impl Isotropic {
    #[allow(dead_code)]
    pub fn new(tex: Rc<dyn Texture>) -> Self {
        Isotropic { tex }
    }
    #[allow(dead_code)]
    pub fn from_color(albedo: Color) -> Self {
        Isotropic {
            tex: Rc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new_ray(rec.p, random_unit_vector(), _r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        Some((attenuation, scattered))
    }
}
