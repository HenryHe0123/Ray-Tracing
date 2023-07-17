use crate::utility::vec3::*;
use std::ops::Index;

#[derive(Copy, Clone, Default)]
pub struct ONB {
    pub axis: [Vec3; 3],
}

impl ONB {
    pub fn build_from_w(w: &Vec3) -> Self {
        let v2 = w.unit();
        let a = if v2.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v1 = cross(&v2, &a).unit();
        let v0 = cross(&v2, &v1);
        Self { axis: [v0, v1, v2] }
    }

    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }

    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }

    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a * self.axis[0] + b * self.axis[1] + c * self.axis[2]
    }

    pub fn local_vec(&self, a: &Vec3) -> Vec3 {
        a.x() * self.axis[0] + a.y() * self.axis[1] + a.z() * self.axis[2]
    }
}

impl Index<usize> for ONB {
    type Output = Vec3;

    fn index(&self, i: usize) -> &Self::Output {
        &self.axis[i]
    }
}
