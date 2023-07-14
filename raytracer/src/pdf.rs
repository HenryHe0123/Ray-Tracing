use crate::onb::ONB;
use crate::vec3::{dot, Vec3};
use std::f64::consts::PI;

pub trait PDF {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

#[derive(Copy, Clone, Default)]
pub struct CosPDF {
    pub uvw: ONB,
}

impl CosPDF {
    pub fn new(w: &Vec3) -> Self {
        Self {
            uvw: ONB::build_from_w(w),
        }
    }
}

impl PDF for CosPDF {
    fn value(&self, direction: &Vec3) -> f64 {
        let cos = dot(&direction.unit(), &self.uvw.w());
        if cos < 0.0 {
            0.0
        } else {
            cos / PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_vec(&Vec3::random_cosine_direction())
    }
}
