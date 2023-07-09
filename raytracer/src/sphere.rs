use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(cen: &Vec3, r: f64, p_clone: Rc<dyn Material>) -> Self {
        Sphere {
            center: *cen,
            radius: r,
            mat_ptr: p_clone,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let mut root: f64 = (-half_b - discriminant.sqrt()) / a; //nearest
        if root < t_min || root > t_max {
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(root);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = Some(Rc::clone(&self.mat_ptr));
        true
    }
}

#[derive(Clone)]
pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub radius: f64,
    pub time0: f64,
    pub time1: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        cen0: &Vec3,
        cen1: &Vec3,
        r: f64,
        time0: f64,
        time1: f64,
        p_clone: Rc<dyn Material>,
    ) -> Self {
        MovingSphere {
            center0: *cen0,
            center1: *cen1,
            radius: r,
            time0,
            time1,
            mat_ptr: p_clone,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().length_squared();
        let half_b = dot(&r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let mut root: f64 = (-half_b - discriminant.sqrt()) / a; //nearest
        if root < t_min || root > t_max {
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(root);
        let outward_normal = (rec.p - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = Some(Rc::clone(&self.mat_ptr));
        true
    }
}
