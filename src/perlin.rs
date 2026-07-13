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
    #[allow(clippy::needless_range_loop)]
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randfloat[self.perm_x[(i as usize + di) & 255]
                        ^ self.perm_y[(j as usize + dj) & 255]
                        ^ self.perm_z[(k as usize + dk) & 255]];
                }
            }
        }
        Self::trilinear_interp(c, u, v, w)
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
    #[allow(clippy::needless_range_loop)]
    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        accum
    }
}
