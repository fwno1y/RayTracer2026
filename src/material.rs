use crate::hittable::HitRecord;
use crate::onb::Onb;
use crate::ray::Ray;
use crate::rtweekend::{PI, random_double};
use crate::texture::{SolidColor, Texture};
use crate::vec3::{
    Point3, Vec3, dot, random_cosine_direction, random_unit_vector, reflect, refract, unit_vector,
};
use crate::vec3color::Color;
use std::sync::Arc;

#[allow(dead_code)]
pub trait Material: Send + Sync {
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new_vec3(0.0, 0.0, 0.0)
    }
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        None
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}
impl Lambertian {
    #[allow(dead_code)]
    pub fn from_color(albedo: Color) -> Self {
        Lambertian {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
    #[allow(dead_code)]
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        let uvw = Onb::new(rec.normal);
        let scatter_direction = uvw.transform(random_cosine_direction());
        let scattered = Ray::new_ray(rec.p, scatter_direction, _r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        let pdf = dot(uvw.w(), scattered.direction()) / PI;
        Some((attenuation, scattered, pdf))
    }
    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = dot(rec.normal, unit_vector(scattered.direction())).max(0.0);
        cos_theta / PI
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        let mut reflected = reflect(r_in.direction(), rec.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        let scattered = Ray::new_ray(rec.p, reflected, r_in.time());
        let attenuation = self.albedo;
        if dot(scattered.direction(), rec.normal) > 0.0 {
            Some((attenuation, scattered, 1.0))
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
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
        Some((attenuation, scattered, 1.0))
    }
}

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    #[allow(dead_code)]
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        DiffuseLight { tex }
    }
    #[allow(dead_code)]
    pub fn from_color(emit: Color) -> Self {
        DiffuseLight {
            tex: Arc::new(SolidColor::new(emit)),
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
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    #[allow(dead_code)]
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Isotropic { tex }
    }
    #[allow(dead_code)]
    pub fn from_color(albedo: Color) -> Self {
        Isotropic {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        let scattered = Ray::new_ray(rec.p, random_unit_vector(), _r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        let pdf = 1.0 / (4.0 * PI);
        Some((attenuation, scattered, pdf))
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
