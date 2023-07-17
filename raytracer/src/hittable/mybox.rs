use crate::hittable::aarect::*;
use crate::hittable::bvh::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::material::Material;
use crate::utility::ray::Ray;
use crate::utility::vec3::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct MyBox {
    pub box_min: Point3,
    pub box_max: Point3,
    pub sides: HittableList,
}

impl MyBox {
    pub fn new<M: Material + Clone + 'static>(p0: &Point3, p1: &Point3, material: M) -> Self {
        let mut sides = HittableList::default();
        sides.add(Arc::new(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            material.clone(),
        )));
        sides.add(Arc::new(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            material.clone(),
        )));

        sides.add(Arc::new(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            material.clone(),
        )));
        sides.add(Arc::new(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            material.clone(),
        )));
        sides.add(Arc::new(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            material.clone(),
        )));
        sides.add(Arc::new(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            material,
        )));

        Self {
            box_min: *p0,
            box_max: *p1,
            sides,
        }
    }
}

impl Hittable for MyBox {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(&self.box_min, &self.box_max);
        true
    }
}
