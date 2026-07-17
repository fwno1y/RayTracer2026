use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use std::sync::Arc;
use crate::rtweekend::random_double;
use crate::vec3::{Point3, Vec3};

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}
impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            bbox: Aabb::empty(),
        }
    }
    #[allow(dead_code)]
    pub fn new_with(objects: Vec<Arc<dyn Hittable>>, bbox: Aabb) -> Self {
        HittableList { objects, bbox }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        let bbox = object.bounding_box();
        self.objects.push(object);
        self.bbox = Aabb::aabb_merge(self.bbox, bbox);
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
        if self.objects.is_empty() { return 0.0; }
        let mut sum = 0.0;
        for obj in &self.objects {
            sum += obj.pdf_value(origin, direction);
        }
        sum / self.objects.len() as f64
    }

    fn random(&self, origin: Vec3) -> Vec3 {
        if self.objects.is_empty() {
            return Vec3::new_vec3(0.0, 0.0, 0.0);
        }
        let idx = (random_double() * self.objects.len() as f64) as usize;
        self.objects[idx].random(origin)
    }
}
