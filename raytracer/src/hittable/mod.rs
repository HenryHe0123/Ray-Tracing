pub mod aarect;
pub mod constant_medium;
pub mod mybox;
pub mod sphere;

use crate::bvh::aabb::{surrounding_box, AABB};
use crate::material::Material;
use crate::ray::Ray;
use crate::utility::random_int_range;
use crate::utility::vec3::*;
use std::f64::INFINITY;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,    //hit point
    pub normal: Vec3, //normal against ray direction
    pub t: f64,       //optical distance
    pub u: f64,
    pub v: f64,           //surface coordinates
    pub front_face: bool, //if ray hit to the front face
    pub mat_ptr: Option<Arc<dyn Material>>,
}

impl HitRecord {
    pub fn new(p_clone: Arc<dyn Material>) -> Self {
        HitRecord {
            p: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
            mat_ptr: Some(p_clone),
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.dir, outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
    fn pdf_value(&self, _o: &Point3, _v: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, _o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Arc<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut tmp_box = AABB::default();
        let mut first_box = true;

        for object in &self.objects {
            if !object.bounding_box(time0, time1, &mut tmp_box) {
                return false;
            }
            *output_box = if first_box {
                tmp_box
            } else {
                surrounding_box(output_box, &tmp_box)
            };
            first_box = false;
        }
        true
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        let weight = 1.0 / (self.objects.len() as f64);
        let mut sum = 0.0;
        for object in &self.objects {
            sum += weight * object.pdf_value(o, v);
        }
        sum
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let size = self.objects.len() as i32;
        self.objects[random_int_range(0, size - 1) as usize].random(o)
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct Translate {
    pub ptr: Option<Arc<dyn Hittable>>,
    pub offset: Vec3,
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new(&(r.origin() - self.offset), &r.direction(), r.time());
        if !self.ptr.as_ref().unwrap().hit(&moved_r, t_min, t_max, rec) {
            return false;
        }
        rec.p += self.offset;
        let normal = rec.normal;
        rec.set_face_normal(&moved_r, &normal);
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let mut temp_box = AABB::default();
        if !self
            .ptr
            .as_ref()
            .unwrap()
            .bounding_box(time0, time1, &mut temp_box)
        {
            return false;
        }
        *output_box = AABB::new(
            &(temp_box.min() + self.offset),
            &(temp_box.max() + self.offset),
        );
        true
    }
}

impl Translate {
    pub fn new(p_clone: Arc<dyn Hittable>, displacement: &Vec3) -> Self {
        Self {
            ptr: Some(p_clone),
            offset: *displacement,
        }
    }
}

#[derive(Clone, Default)]
pub struct RotateY {
    pub ptr: Option<Arc<dyn Hittable>>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hasbox: bool,
    pub bbox: AABB,
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(&origin, &direction, r.time());

        if !self
            .ptr
            .as_ref()
            .unwrap()
            .hit(&rotated_r, t_min, t_max, rec)
        {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.set_face_normal(&rotated_r, &normal);

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        self.hasbox
    }
}

impl RotateY {
    pub fn new(p_clone: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = AABB::default();
        let hasbox = p_clone.bounding_box(0.0, 1.0, &mut bbox);
        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;
                    let x = i * bbox.max().x() + (1.0 - i) * bbox.min().x();
                    let y = j * bbox.max().y() + (1.0 - j) * bbox.min().y();
                    let z = k * bbox.max().z() + (1.0 - k) * bbox.min().z();
                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(new_x, y, new_z);
                    for c in 0..3 {
                        min[c] = tester[c].min(min[c]);
                        max[c] = tester[c].max(max[c]);
                    }
                }
            }
        }
        bbox = AABB::new(&min, &max);
        Self {
            ptr: Some(p_clone),
            sin_theta,
            cos_theta,
            hasbox,
            bbox,
        }
    }
}
