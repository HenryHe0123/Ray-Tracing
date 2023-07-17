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
pub struct HittablePDF<'a> {
    o: Point3,
    ptr: &'a dyn Hittable,
}

impl<'a> HittablePDF<'a> {
    pub fn new(ptr: &'a dyn Hittable, origin: &Point3) -> Self {
        Self { o: *origin, ptr }
    }
}

impl<'a> PDF for HittablePDF<'a> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}

#[derive(Clone)]
pub struct MixturePDF<'a> {
    p: [&'a dyn PDF; 2],
}

impl<'a> MixturePDF<'a> {
    pub fn new(p0: &'a dyn PDF, p1: &'a dyn PDF) -> Self {
        Self { p: [p0, p1] }
    }
}

impl<'a> PDF for MixturePDF<'a> {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * (self.p[0].value(direction) + self.p[1].value(direction))
    }

    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
