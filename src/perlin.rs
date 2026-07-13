use crate::rtweekend::{random_double, random_int_range};
use crate::vec3::Point3;

const POINT_COUNT: i32 = 256;
pub struct Perlin {
    randfloat: [f64; POINT_COUNT as usize],
    perm_x: [usize; POINT_COUNT as usize],
    perm_y: [usize; POINT_COUNT as usize],
    perm_z: [usize; POINT_COUNT as usize],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut randfloat = [0.0; POINT_COUNT as usize];
        for i in 0..POINT_COUNT {
            randfloat[i as usize] = random_double();
        }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        Perlin {
            randfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (4.0 * p.x()) as i32 & 255;
        let j = (4.0 * p.y()) as i32 & 255;
        let k = (4.0 * p.z()) as i32 & 255;
        self.randfloat[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }
    fn perlin_generate_perm() -> [usize; POINT_COUNT as usize] {
        let mut p = [0; POINT_COUNT as usize];
        for (i, val) in p.iter_mut().enumerate() {
            *val = i;
        }
        for i in 0..POINT_COUNT {
            let target = random_int_range(0, i);
            p.swap(i as usize, target as usize);
        }
        p
    }
}
