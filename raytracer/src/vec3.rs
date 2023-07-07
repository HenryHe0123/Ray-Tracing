use crate::rt_weekend::{clamp, random_double, random_double_range};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, Neg, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }

    pub fn random() -> Self {
        Vec3(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Vec3::random_in_unit_sphere().unit()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if dot(&in_unit_sphere, normal) > 0.0 {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        self.length_squared() < 1e-15
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn rgb(&self) -> [u8; 3] {
        [
            (255.999 * self.0) as u8,
            (255.999 * self.1) as u8,
            (255.999 * self.2) as u8,
        ]
    }

    pub fn multi_samples_rgb(&self, samples: u32) -> [u8; 3] {
        let scale = 1.0 / (samples as f64);
        let r = scale * self.x();
        let g = scale * self.y();
        let b = scale * self.z();
        //adding gamma-correct for gamma = 2
        [
            (255.999 * clamp(r.sqrt(), 0.0, 0.999)) as u8,
            (255.999 * clamp(g.sqrt(), 0.0, 0.999)) as u8,
            (255.999 * clamp(b.sqrt(), 0.0, 0.999)) as u8,
        ]
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length_squared()
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Self::Output {
        Vec3(self.0 * t, self.1 * t, self.2 * t)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3(self * v.0, self * v.1, self * v.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Self::Output {
        Vec3(self.0 / t, self.1 / t, self.2 / t)
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.0 * v.0 + u.1 * v.1 + u.2 * v.2
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.1 * v.2 - u.2 * v.1,
        u.2 * v.0 - u.0 * v.2,
        u.0 * v.1 - u.1 * v.0,
    )
}

// Type aliases for Vec3
pub type Point3 = Vec3;
// 3D point
pub type Color = Vec3; // RGB color

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.0, self.1, self.2)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.0, -self.1, -self.2)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.0 *= t;
        self.1 *= t;
        self.2 *= t;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    let n = n.unit();
    *v - 2.0 * dot(v, &n) * n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&(-*uv), n).min(1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * (*n));
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * (*n);
    r_out_perp + r_out_parallel
}
