use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, cross, dot, unit_vector};
use std::sync::Arc;

pub struct Triangle {
    v0: Point3,
    v1: Point3,
    v2: Point3,
    normal: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
}

impl Triangle {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, mat: Arc<dyn Material>) -> Self {
        let normal = unit_vector(cross(v1 - v0, v2 - v0));
        let bbox = Aabb::aabb_merge(Aabb::from_points(v0, v1), Aabb::from_points(v0, v2));
        Triangle {
            v0,
            v1,
            v2,
            normal,
            mat,
            bbox,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = cross(r.direction(), edge2);
        let a = dot(edge1, h);
        if a.abs() < 1e-8 {
            return false;
        } // 射线与三角形平行
        let f = 1.0 / a;
        let s = r.origin() - self.v0;
        let u = f * dot(s, h);
        if !(0.0..=1.0).contains(&u) {
            return false;
        }
        let q = cross(s, edge1);
        let v = f * dot(r.direction(), q);
        if v < 0.0 || u + v > 1.0 {
            return false;
        }
        let t = f * dot(edge2, q);
        if !ray_t.contains(t) {
            return false;
        }

        rec.t = t;
        rec.p = r.at(t);
        rec.normal = self.normal;
        rec.set_face_normal(r, self.normal);
        rec.mat = Some(self.mat.clone());
        rec.u = u;
        rec.v = v;
        true
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
