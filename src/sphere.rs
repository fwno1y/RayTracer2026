use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::onb::Onb;
use crate::ray::Ray;
use crate::rtweekend::{INFINITY, PI, random_double};
use crate::vec3::{Point3, Vec3, dot};
use std::sync::Arc;

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
    bbox: Aabb,
}
impl Sphere {
    pub fn new(static_center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        let rvec = Vec3::new_vec3(radius, radius, radius);
        Sphere {
            center: Ray::new_ray(static_center, Vec3::new_vec3(0.0, 0.0, 0.0), 0.0),
            radius: radius.max(0.0),
            mat: material,
            bbox: Aabb::from_points(static_center - rvec, static_center + rvec),
        }
    }
    pub fn new_move(
        center1: Point3,
        center2: Point3,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let center = Ray::new_ray(center1, center2 - center1, 0.0);
        let rvec = Vec3::new_vec3(radius, radius, radius);
        let box1 = Aabb::from_points(center.at(0.0) - rvec, center.at(0.0) + rvec);
        let box2 = Aabb::from_points(center.at(1.0) - rvec, center.at(1.0) + rvec);
        Sphere {
            center,
            radius: radius.max(0.0),
            mat: material,
            bbox: Aabb::aabb_merge(box1, box2),
        }
    }
    pub fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
    pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
        let r1 = random_double();
        let r2 = random_double();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);
        let phi = 2.0 * PI * r1;
        let x = phi.cos() * (1.0 - z * z).sqrt();
        let y = phi.sin() * (1.0 - z * z).sqrt();
        Vec3::new_vec3(x, y, z)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        let (u, v) = Self::get_sphere_uv(&Point3::new_vec3(
            outward_normal.x(),
            outward_normal.y(),
            outward_normal.z(),
        ));
        rec.u = u;
        rec.v = v;
        rec.mat = Some(self.mat.clone());
        true
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if !self.hit(
            &Ray::new_ray(origin, direction, 0.0),
            Interval::new(0.001, INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }
        let dist_squared = (self.center.at(0.0) - origin).length_squared();
        let cos_theta_max = (1.0 - self.radius * self.radius / dist_squared).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
        1.0 / solid_angle
    }
    fn random(&self, _origin: Vec3) -> Vec3 {
        let direction = self.center.at(0.0) - _origin;
        let distance_squared = direction.length_squared();
        let uvw = Onb::new(direction);
        uvw.transform(Self::random_to_sphere(self.radius, distance_squared))
    }
}
