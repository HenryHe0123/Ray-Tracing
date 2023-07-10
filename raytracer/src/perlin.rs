use crate::rt_weekend::{random_double, random_int_range};
use crate::vec3::Point3;

#[derive(Clone)]
pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Perlin {
        let mut ranfloat = Vec::new();
        for _i in 0..Perlin::POINT_COUNT {
            ranfloat.push(random_double());
        }
        Perlin {
            ranfloat,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cj) in ci.iter_mut().enumerate() {
                for (dk, ck) in cj.iter_mut().enumerate() {
                    *ck = self.ranfloat[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        Perlin::trilinear_interp(&c, u, v, w)
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = vec![0; Perlin::POINT_COUNT];
        for (i, it) in p.iter_mut().enumerate().take(Perlin::POINT_COUNT) {
            *it = i as i32;
        }
        Perlin::permute(&mut p, Perlin::POINT_COUNT);
        p
    }

    fn permute(p: &mut [i32], n: usize) {
        for i in (1..n).rev() {
            let target = random_int_range(0, i as i32);
            p.swap(i, target as usize);
        }
    }

    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for (i, ci) in c.iter().enumerate() {
            for (j, cj) in ci.iter().enumerate() {
                for (k, ck) in cj.iter().enumerate() {
                    accum += ((i as f64) * u + ((1 - i) as f64) * (1.0 - u))
                        * ((j as f64) * v + ((1 - j) as f64) * (1.0 - v))
                        * ((k as f64) * w + ((1 - k) as f64) * (1.0 - w))
                        * (*ck);
                }
            }
        }
        accum
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}