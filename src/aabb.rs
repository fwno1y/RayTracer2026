use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }
    pub fn empty() -> Self {
        Self::default()
    }
    pub fn aabb(a: Point3, b: Point3) -> Self {
        let x = if a[0] <= b[0] { Interval::new(a[0], b[0]) } else { Interval::new(b[0], a[0]) };
        let y = if a[1] <= b[1] { Interval::new(a[1], b[1]) } else { Interval::new(b[1], a[1]) };
        let z = if a[2] <= b[2] { Interval::new(a[2], b[2]) } else { Interval::new(b[2], a[2]) };
        Self { x, y, z }
    }
    pub fn axis_interval(&self, n: u32) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("invalid number of axis"),
        }
    }
    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_orig[axis as usize];
            let t0 = (ax.min - ray_orig[axis as usize]) * adinv;
            let t1 = (ax.max - ray_orig[axis as usize]) * adinv;
            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            x: Interval::EMPTY,
            y: Interval::EMPTY,
            z: Interval::EMPTY,
        }
    }
}