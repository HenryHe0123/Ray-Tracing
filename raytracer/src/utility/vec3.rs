use crate::utility::{clamp, random_double, random_double_range};
use std::f64::consts::PI;
use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, Neg, SubAssign};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn random() -> Self {
        Vec3::new(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3::new(
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

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(
                random_double_range(-1.0, 1.0),
                random_double_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    pub fn random_cosine_direction() -> Self {
        let r1 = random_double();
        let r2 = random_double();
        let z = (1.0 - r2).sqrt();
        //generate z with pdf(z) = z, so z = sqrt(random())

        let phi = 2.0 * PI * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();

        Vec3::new(x, y, z)
    }

    pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Self {
        let r1 = random_double();
        let r2 = random_double();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

        let phi = 2.0 * PI * r1;
        let x = phi.cos() * (1.0 - z * z).sqrt();
        let y = phi.sin() * (1.0 - z * z).sqrt();

        Vec3::new(x, y, z)
    }

    pub fn near_zero(&self) -> bool {
        self.length_squared() < 1e-15
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn rgb(&self) -> [u8; 3] {
        [
            (255.999 * self.e[0]) as u8,
            (255.999 * self.e[1]) as u8,
            (255.999 * self.e[2]) as u8,
        ]
    }

    pub fn multi_samples_rgb(&self, samples: u32) -> [u8; 3] {
        let r = if self.x().is_nan() { 0.0 } else { self.x() };
        let g = if self.y().is_nan() { 0.0 } else { self.y() };
        let b = if self.z().is_nan() { 0.0 } else { self.z() };

        let scale = 1.0 / (samples as f64);
        let r = scale * r;
        let g = scale * g;
        let b = scale * b;
        //adding gamma-correct for gamma = 2
        [
            (255.999 * clamp(r.sqrt(), 0.0, 0.999)) as u8,
            (255.999 * clamp(g.sqrt(), 0.0, 0.999)) as u8,
            (255.999 * clamp(b.sqrt(), 0.0, 0.999)) as u8,
        ]
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn red() -> Self {
        Self { e: [1., 0., 0.] }
    }

    pub fn green() -> Self {
        Self { e: [0., 1., 0.] }
    }

    pub fn blue() -> Self {
        Self { e: [0., 0., 1.] }
    }

    pub fn yellow() -> Self {
        Self { e: [1., 1., 0.] }
    }

    pub fn purple() -> Self {
        Self { e: [1., 0., 1.] }
    }

    pub fn black() -> Self {
        Self { e: [0., 0., 0.] }
    }

    pub fn white() -> Self {
        Self { e: [1., 1., 1.] }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Self::Output {
        Vec3::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3::new(self * v.e[0], self * v.e[1], self * v.e[2])
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Self::Output {
        Vec3::new(self.e[0] / t, self.e[1] / t, self.e[2] / t)
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

// Type aliases for Vec3
pub type Point3 = Vec3;
// 3D point
pub type Color = Vec3; // RGB color

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.e[0], self.e[1], self.e[2])
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
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
