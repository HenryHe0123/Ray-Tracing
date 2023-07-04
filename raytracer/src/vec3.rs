use std::ops::{Add, Sub, Mul, Div};
use std::ops::{Neg, AddAssign, SubAssign, MulAssign, DivAssign};
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
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
        [(255.999 * self.0) as u8, (255.999 * self.1) as u8, (255.999 * self.2) as u8]
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

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

// Type aliases for Vec3
pub type Point3 = Vec3;
// 3D point
pub type Color = Vec3;   // RGB color

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
