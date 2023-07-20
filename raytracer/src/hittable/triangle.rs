use crate::hittable::bvh::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::utility::ray::Ray;
use crate::utility::vec3::*;
use crate::utility::{random_double, random_double_range};
use std::f64::INFINITY;

#[derive(Clone)]
pub struct Triangle<M: Material> {
    pub a: Point3,
    pub n: Vec3,
    pub pb: Vec3,
    pub pc: Vec3,
    //pc perpendicular to ac with length of ac/2*area
    pub bbox: AABB,
    pub mat: M,
}

impl<M: Material> Triangle<M> {
    pub fn new(a: &Point3, b: &Point3, c: &Point3, mat: M) -> Self {
        let ab = *b - *a;
        let ac = *c - *a;
        let normal = cross(&ab, &ac);
        let area2 = normal.length();
        let n = normal.unit();
        let mut min = Point3::default();
        let mut max = Point3::default();
        for i in 0..3 {
            min[i] = a[i].min(b[i]).min(c[i]) - 0.0001;
            max[i] = a[i].max(b[i]).max(c[i]) + 0.0001;
        }
        Self {
            a: *a,
            n,
            pb: cross(&n, &ab) / area2,
            pc: cross(&ac, &n) / area2,
            mat,
            bbox: AABB::new(&min, &max),
        }
    }

    pub fn area(&self) -> f64 {
        cross(&self.pb, &self.pc).length() / 2.0
    }

    pub fn get_edges(&self) -> (Vec3, Vec3) {
        let area2 = self.area() * 2.0;
        let ab = cross(&self.pb, &self.n) * area2;
        let ac = cross(&self.n, &self.pc) * area2;

        let normal = cross(&ab, &ac);
        if normal.unit() == self.n {
            (ab, ac)
        } else {
            panic!("triangle get edges error")
        }
    }
}

impl<M: Material> Hittable for Triangle<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oa = self.a - r.origin();
        let t = dot(&oa, &self.n) / dot(r.direction_borrow(), &self.n);
        if t < t_min || t_max < t {
            return None;
        }
        let p = r.at(t);
        let ap = p - self.a;
        let u = dot(&ap, &self.pc);
        let v = dot(&ap, &self.pb);
        // AP = uAB + vAC
        if u >= 0. && v >= 0. && u + v <= 1. {
            let rec = HitRecord {
                p,
                normal: self.n,
                t,
                u,
                v,
                front_face: true, //set it true if you want to emit light!!!
                mat_ptr: &self.mat,
            };
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        true
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        let op = self.hit(&Ray::new(o, v, 0.0), 0.001, INFINITY);
        if let Some(rec) = op {
            let distance_squared = rec.t * rec.t * v.length_squared();
            let cosine = (dot(v, &rec.normal) / v.length()).abs();
            distance_squared / (cosine * self.area())
        } else {
            0.0
        }
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let u = random_double();
        let v = random_double_range(0., 1.0 - u);
        let (ab, ac) = self.get_edges();
        let random_p = self.a + u * ab + v * ac;
        random_p - *origin
    }
}
