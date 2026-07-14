use crate::INFINITY;
use std::ops::Add;
#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}
impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }
    pub fn merge(a: Interval, b: Interval) -> Interval {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };
        Interval { min, max }
    }
    #[allow(dead_code)]
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    #[allow(dead_code)]
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
    #[allow(dead_code)]
    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
    #[allow(dead_code)]
    pub const EMPTY: Self = Interval {
        min: INFINITY,
        max: -INFINITY,
    };
    #[allow(dead_code)]
    pub const UNIVERSE: Self = Interval {
        min: -INFINITY,
        max: INFINITY,
    };
}
impl Default for Interval {
    fn default() -> Interval {
        Interval {
            min: INFINITY,
            max: -INFINITY,
        }
    }
}

impl Add<f64> for Interval {
    type Output = Interval;
    fn add(self, rhs: f64) -> Interval {
        Interval::new(self.min + rhs, self.max + rhs)
    }
}
