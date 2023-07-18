pub mod onb;

use crate::hittable::Hittable;
use crate::pdf::onb::ONB;
use crate::utility::random_double;
use crate::utility::vec3::*;
use std::f64::consts::PI;

pub trait PDF {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

#[derive(Copy, Clone, Default)]
pub struct CosPDF {
    uvw: ONB,
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

#[derive(Clone)]
pub struct HittablePDF<'a, H: Hittable> {
    o: Point3,
    ptr: &'a H,
}

impl<'a, H: Hittable> HittablePDF<'a, H> {
    pub fn new(ptr: &'a H, origin: &Point3) -> Self {
        Self { o: *origin, ptr }
    }
}

impl<'a, H: Hittable> PDF for HittablePDF<'a, H> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}

#[derive(Clone)]
pub struct MixturePDF<'a, P0: PDF, P1: PDF> {
    p0: &'a P0,
    p1: &'a P1,
}

impl<'a, P0: PDF, P1: PDF> MixturePDF<'a, P0, P1> {
    pub fn new(p0: &'a P0, p1: &'a P1) -> Self {
        Self { p0, p1 }
    }
}

impl<'a, P0: PDF, P1: PDF> PDF for MixturePDF<'a, P0, P1> {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * (self.p0.value(direction) + self.p1.value(direction))
    }

    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
