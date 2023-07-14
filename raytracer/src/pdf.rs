use crate::hittable::Hittable;
use crate::onb::ONB;
use crate::vec3::{dot, Point3, Vec3};
use std::f64::consts::PI;
use std::sync::Arc;

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

#[derive(Clone, Default)]
pub struct HittablePDF {
    pub o: Point3,
    pub ptr: Option<Arc<dyn Hittable>>,
}

impl HittablePDF {
    pub fn new(p_clone: Arc<dyn Hittable>, origin: &Point3) -> Self {
        Self {
            o: *origin,
            ptr: Some(p_clone),
        }
    }
}

impl PDF for HittablePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.as_ref().unwrap().pdf_value(&self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.as_ref().unwrap().random(&self.o)
    }
}