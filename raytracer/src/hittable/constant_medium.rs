use crate::hittable::bvh::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::{Isotropic, Material};
use crate::texture::{SolidColor, Texture};
use crate::utility::random_double;
use crate::utility::ray::Ray;
use crate::utility::vec3::*;
use std::f64::INFINITY;

#[derive(Clone, Default)]
pub struct ConstantMedium<H: Hittable, M: Material> {
    pub boundary: H,
    pub phase_function: M,
    pub neg_inv_density: f64,
}

impl<H: Hittable, T: Texture> ConstantMedium<H, Isotropic<T>> {
    pub fn new(b: H, d: f64, a: T) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Isotropic::new(a),
        }
    }
}

impl<H: Hittable> ConstantMedium<H, Isotropic<SolidColor>> {
    pub fn new_from_color(b: H, d: f64, c: &Color) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Isotropic::new_from_color(c),
        }
    }
}

impl<H: Hittable, T: Texture> Hittable for ConstantMedium<H, Isotropic<T>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let op1 = self.boundary.hit(r, -INFINITY, INFINITY);
        op1.as_ref()?;
        let mut rec1 = op1.unwrap();

        let op2 = self.boundary.hit(r, rec1.t + 0.0001, INFINITY);
        op2.as_ref()?;
        let mut rec2 = op2.unwrap();

        rec1.t = rec1.t.max(t_min);
        rec2.t = rec2.t.min(t_max);

        if rec1.t >= rec2.t {
            return None;
        }
        rec1.t = rec1.t.max(0.0);

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let mut rec = HitRecord::default();
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        rec.front_face = true; // also arbitrary
        rec.mat_ptr = &self.phase_function;

        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}
