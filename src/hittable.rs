use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::rtweekend::{INFINITY, degrees_to_radians};
use crate::vec3::{Point3, Vec3, dot};
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Rc<dyn Material>>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3::new_vec3(0.0, 0.0, 0.0),
            normal: Vec3::new_vec3(0.0, 0.0, 0.0),
            mat: None,
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> Aabb;
}

pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}
impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let offset_r = Ray::new_ray(r.origin() - self.offset, r.direction(), r.time());
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }
        rec.p += self.offset;
        true
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();
        let mut min = Point3::new_vec3(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new_vec3(-INFINITY, -INFINITY, -INFINITY);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f64) * bbox.x.max + (1.0 - i as f64) * bbox.x.min;
                    let y = (j as f64) * bbox.y.max + (1.0 - j as f64) * bbox.y.min;
                    let z = (k as f64) * bbox.z.max + (1.0 - k as f64) * bbox.z.min;
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new_vec3(newx, y, newz);
                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        let bbox_rotated = Aabb::from_points(min, max);
        RotateY {
            object,
            sin_theta,
            cos_theta,
            bbox: bbox_rotated,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let origin = Point3::new_vec3(
            (self.cos_theta * r.origin().x()) - (self.sin_theta * r.origin().z()),
            r.origin().y(),
            (self.sin_theta * r.origin().x()) + (self.cos_theta * r.origin().z()),
        );
        let direction = Vec3::new_vec3(
            (self.cos_theta * r.direction().x()) - (self.sin_theta * r.direction().z()),
            r.direction().y(),
            (self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().z()),
        );
        let rotated_r = Ray::new_ray(origin, direction, r.time());
        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }
        rec.p = Point3::new_vec3(
            (self.cos_theta * rec.p.x()) + (self.sin_theta * rec.p.z()),
            rec.p.y(),
            (-self.sin_theta * rec.p.x()) + (self.cos_theta * rec.p.z()),
        );
        rec.normal = Vec3::new_vec3(
            (self.cos_theta * rec.normal.x()) + (self.sin_theta * rec.normal.z()),
            rec.normal.y(),
            (-self.sin_theta * rec.normal.x()) + (self.cos_theta * rec.normal.z()),
        );
        true
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
