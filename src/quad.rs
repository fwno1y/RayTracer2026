use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, cross, dot, unit_vector};
use std::rc::Rc;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Rc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Self {
        let n = cross(u, v);
        let normal = unit_vector(n);
        let d = dot(normal, q);
        let w = n / dot(n, n);
        let mut quad = Self {
            q,
            u,
            v,
            w,
            mat,
            bbox: Aabb::empty(),
            normal,
            d,
        };
        quad.set_bounding_box();
        quad
    }
    pub fn set_bounding_box(&mut self) {
        let bbox_diagonal1 = Aabb::from_points(self.q, self.q + self.u + self.v);
        let bbox_diagonal2 = Aabb::from_points(self.q + self.u, self.q + self.v);
        self.bbox = Aabb::aabb_merge(bbox_diagonal1, bbox_diagonal2);
    }
    pub fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }
        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = dot(self.normal, r.direction());
        if denom.abs() < 1e-8 {
            return false;
        }
        let t = (self.d - dot(self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = dot(self.w, cross(planar_hitpt_vector, self.v));
        let beta = dot(self.w, cross(self.u, planar_hitpt_vector));
        if !Self::is_interior(alpha, beta, rec) {
            return false;
        }
        rec.t = t;
        rec.p = intersection;
        rec.mat = Some(self.mat.clone());
        rec.set_face_normal(r, self.normal);
        true
    }
}

pub fn make_box(a: Point3, b: Point3, mat: Rc<dyn Material>) -> Rc<dyn Hittable> {
    let mut sides = HittableList::new();
    let min = Point3::new_vec3(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new_vec3(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));
    let dx = Vec3::new_vec3(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new_vec3(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new_vec3(0.0, 0.0, max.z() - min.z());
    sides.add(Rc::new(Quad::new(
        Point3::new_vec3(min.x(), min.y(), max.z()),
        dx,
        dy,
        mat.clone(),
    )));
    sides.add(Rc::new(Quad::new(
        Point3::new_vec3(max.x(), min.y(), max.z()),
        -dz,
        dy,
        mat.clone(),
    )));
    sides.add(Rc::new(Quad::new(
        Point3::new_vec3(max.x(), min.y(), min.z()),
        -dx,
        dy,
        mat.clone(),
    )));
    sides.add(Rc::new(Quad::new(
        Point3::new_vec3(min.x(), min.y(), min.z()),
        dz,
        dy,
        mat.clone(),
    )));
    sides.add(Rc::new(Quad::new(
        Point3::new_vec3(min.x(), max.y(), max.z()),
        dx,
        -dz,
        mat.clone(),
    )));
    sides.add(Rc::new(Quad::new(
        Point3::new_vec3(min.x(), min.y(), min.z()),
        dx,
        dz,
        mat.clone(),
    )));
    Rc::new(sides)
}
