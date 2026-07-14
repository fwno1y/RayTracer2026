use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn from_list(list: HittableList) -> Arc<dyn Hittable> {
        let mut objects = list.objects;
        if objects.is_empty() {
            panic!("Cannot build BVH from empty list");
        }
        let len = objects.len();
        Self::build_recursive(&mut objects, 0, len)
    }
    fn build_recursive(
        objects: &mut [Arc<dyn Hittable>],
        start: usize,
        end: usize,
    ) -> Arc<dyn Hittable> {
        let object_span = end - start;
        if object_span == 1 {
            return Arc::clone(&objects[start]);
        }
        let mut bbox = Aabb::empty();
        for obj in &objects[start..end] {
            bbox = Aabb::aabb_merge(bbox, obj.bounding_box());
        }
        let axis = bbox.longest_axis() as usize;
        let comparator: fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => unreachable!(),
        };
        let (left_child, right_child) = if object_span == 2 {
            (Arc::clone(&objects[start]), Arc::clone(&objects[start + 1]))
        } else {
            objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            let left = Self::build_recursive(objects, start, mid);
            let right = Self::build_recursive(objects, mid, end);
            (left, right)
        };
        Arc::new(BvhNode {
            left: left_child,
            right: right_child,
            bbox,
        })
    }
    pub fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: u32) -> Ordering {
        let abox = a.bounding_box();
        let bbox = b.bounding_box();
        let a_axis_interval = abox.axis_interval(axis_index);
        let b_axis_interval = bbox.axis_interval(axis_index);
        a_axis_interval.min.total_cmp(&b_axis_interval.min)
    }
    pub fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }
    pub fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }
    pub fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }
        let hit_left = self.left.hit(r, ray_t, rec);
        let tmp = if hit_left { rec.t } else { ray_t.max };
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, tmp), rec);
        hit_left || hit_right
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
