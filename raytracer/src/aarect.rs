use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::rt_weekend::random_double_range;
use crate::vec3::{dot, Point3, Vec3};
use std::f64::INFINITY;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct XYRect {
    mp: Option<Arc<dyn Material>>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64, //z = k
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, p_clone: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mp: Some(p_clone),
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            &Point3::new(self.x0, self.y0, self.k - 0.0001),
            &Point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }
}

#[derive(Clone, Default)]
pub struct XZRect {
    mp: Option<Arc<dyn Material>>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64, //y = k
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, p_clone: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            mp: Some(p_clone),
        }
    }
}

impl Hittable for XZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            &Point3::new(self.x0, self.k - 0.0001, self.z0),
            &Point3::new(self.x1, self.k + 0.0001, self.z1),
        );
        true
    }

    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if !self.hit(&Ray::new(origin, v, 0.0), 0.001, INFINITY, &mut rec) {
            return 0.0;
        }
        let area = (self.x1 - self.x0) * (self.z1 - self.z0);
        let distance_squared = rec.t * rec.t * v.length_squared();
        let cosine = (dot(v, &rec.normal) / v.length()).abs();
        distance_squared / (cosine * area)
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let random_point = Point3::new(
            random_double_range(self.x0, self.x1),
            self.k,
            random_double_range(self.z0, self.z1),
        );
        random_point - *origin
    }
}

#[derive(Clone, Default)]
pub struct YZRect {
    mp: Option<Arc<dyn Material>>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64, //x = k
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, p_clone: Arc<dyn Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            mp: Some(p_clone),
        }
    }
}

impl Hittable for YZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            return false;
        }
        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if z < self.z0 || z > self.z1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            &Point3::new(self.k - 0.0001, self.y0, self.z0),
            &Point3::new(self.k + 0.0001, self.y1, self.z1),
        );
        true
    }
}
