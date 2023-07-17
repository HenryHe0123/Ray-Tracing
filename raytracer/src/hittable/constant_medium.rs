use crate::hittable::bvh::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::{Isotropic, Material};
use crate::texture::Texture;
use crate::utility::random_double;
use crate::utility::ray::Ray;
use crate::utility::vec3::*;
use std::f64::INFINITY;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct ConstantMedium {
    pub boundary: Option<Arc<dyn Hittable>>,
    pub phase_function: Option<Arc<dyn Material>>,
    pub neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(b: Arc<dyn Hittable>, d: f64, c: &Color) -> Self {
        Self {
            boundary: Some(b),
            neg_inv_density: -1.0 / d,
            phase_function: Some(Arc::new(Isotropic::new(c))),
        }
    }

    pub fn new_from_texture_ptr(
        b: Arc<dyn Hittable>,
        d: f64,
        a: &Arc<dyn Texture + Send + Sync>,
    ) -> Self {
        Self {
            boundary: Some(b),
            neg_inv_density: -1.0 / d,
            phase_function: Some(Arc::new(Isotropic::new_from_ptr(a))),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Print occasional samples when debugging. To enable, set enable_debug true.
        let enable_debug: bool = false;
        let debugging: bool = enable_debug && random_double() < 0.00001;

        let op1 = self.boundary.as_ref().unwrap().hit(r, -INFINITY, INFINITY);
        op1.as_ref()?;
        let mut rec1 = op1.unwrap();

        let op2 = self
            .boundary
            .as_ref()
            .unwrap()
            .hit(r, rec1.t + 0.0001, INFINITY);
        op2.as_ref()?;
        let mut rec2 = op2.unwrap();

        if debugging {
            println!("\nt_min = {}, t_max = {}\n", rec1.t, rec2.t);
        }

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

        if debugging {
            println!(
                "\nhit_distance = {}, rec.t = {}, rec.p = {}\n",
                hit_distance, rec.t, rec.p
            );
        }

        rec.normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        rec.front_face = true; // also arbitrary
        rec.mat_ptr = self.phase_function.clone();

        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.boundary
            .as_ref()
            .unwrap()
            .bounding_box(time0, time1, output_box)
    }
}
