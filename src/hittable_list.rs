use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::random_int_range;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

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
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;
        for object in &self.objects {
            sum += weight * object.pdf_value(origin, direction);
        }
        sum
    }

    fn random(&self, origin: Vec3) -> Vec3 {
        let int_size = self.objects.len() as i32;
        let idx = random_int_range(0, int_size - 1);
        self.objects[idx as usize].random(origin)
    }
}
