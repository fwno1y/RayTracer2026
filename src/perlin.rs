use crate::rtweekend::random_int_range;
use crate::vec3::{Point3, Vec3, dot, unit_vector};

const POINT_COUNT: i32 = 256;
pub struct Perlin {
    randvec: [Vec3; POINT_COUNT as usize],
    perm_x: [usize; POINT_COUNT as usize],
    perm_y: [usize; POINT_COUNT as usize],
    perm_z: [usize; POINT_COUNT as usize],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut randvec = [Vec3::new_vec3(0.0, 0.0, 0.0); POINT_COUNT as usize];
        for i in 0..POINT_COUNT {
            randvec[i as usize] = unit_vector(Vec3::random_in_range(-1.0, 1.0));
        }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        Perlin {
            randvec,
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
        let mut c = [[[Vec3::new_vec3(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[self.perm_x[(i as usize + di) & 255]
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
    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new_vec3(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * dot(c[i][j][k], weight_v);
                }
            }
        }
        accum
    }
}
