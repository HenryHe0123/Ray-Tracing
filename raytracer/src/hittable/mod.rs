pub mod aarect;
pub mod bvh;
pub mod constant_medium;
pub mod mybox;
pub mod sphere;
pub mod triangle;

use crate::hittable::bvh::aabb::{surrounding_box, AABB};
use crate::material::{Material, DEFAULT_MATERIAL};
use crate::utility::random_int_range;
use crate::utility::ray::Ray;
use crate::utility::vec3::*;
use std::f64::consts::PI;
use std::f64::INFINITY;

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,    //hit point
    pub normal: Vec3, //normal against ray direction
    pub t: f64,       //optical distance
    pub u: f64,
    pub v: f64,           //surface coordinates
    pub front_face: bool, //if ray hit to the front face
    pub mat_ptr: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(mat_ptr: &'a dyn Material) -> Self {
        HitRecord {
            p: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
            mat_ptr,
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

impl<'a> Default for HitRecord<'a> {
    fn default() -> Self {
        Self::new(&DEFAULT_MATERIAL)
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
    fn pdf_value(&self, _o: &Point3, _v: &Vec3) -> f64 {
        0.25 / PI
    }
    fn random(&self, _o: &Vec3) -> Vec3 {
        Vec3::random().unit()
    }
    fn empty(&self) -> bool {
        false
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
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

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn size(&self) -> usize {
        self.objects.len()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                temp_rec = rec;
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        }

        if hit_anything {
            Some(temp_rec)
        } else {
            None
        }
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

    fn empty(&self) -> bool {
        self.objects.is_empty()
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

//--------------------------------Transform--------------------------------------

#[derive(Clone, Default)]
pub struct Translate<H: Hittable> {
    pub ptr: H,
    pub offset: Vec3,
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(&(r.origin() - self.offset), r.direction_borrow(), r.time());
        if let Some(mut rec) = self.ptr.hit(&moved_r, t_min, t_max) {
            rec.p += self.offset;
            //let normal = rec.normal;
            //rec.set_face_normal(&moved_r, &normal);
            return Some(rec);
        }
        None
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let mut temp_box = AABB::default();
        if !self.ptr.bounding_box(time0, time1, &mut temp_box) {
            return false;
        }
        *output_box = AABB::new(
            &(temp_box.min() + self.offset),
            &(temp_box.max() + self.offset),
        );
        true
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        self.ptr.pdf_value(&(*o - self.offset), v)
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        self.ptr.random(&(*o - self.offset))
    }
}

impl<H: Hittable> Translate<H> {
    pub fn new(p: H, offset: &Vec3) -> Self {
        Self {
            ptr: p,
            offset: *offset,
        }
    }
}

#[derive(Clone, Default)]
pub struct RotateY<H: Hittable> {
    pub ptr: H,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hasbox: bool,
    pub bbox: AABB,
}

impl<H: Hittable> Hittable for RotateY<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(&origin, &direction, r.time());

        if let Some(mut rec) = self.ptr.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            rec.p = p;
            //rec.set_face_normal(&rotated_r, &normal);
            rec.normal = normal; //
            return Some(rec);
        }

        None
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        self.hasbox
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        let rotated_o = rotate_vec_y(o, self.sin_theta, self.cos_theta);
        let rotated_v = rotate_vec_y(v, self.sin_theta, self.cos_theta);
        self.ptr.pdf_value(&rotated_o, &rotated_v)
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let rotated_o = rotate_vec_y(o, self.sin_theta, self.cos_theta);
        let rotated_rand = self.ptr.random(&rotated_o);
        rotate_vec_y(&rotated_rand, -self.sin_theta, self.cos_theta)
    }
}

impl<H: Hittable> RotateY<H> {
    pub fn new(p: H, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = AABB::default();
        let hasbox = p.bounding_box(0.0, 1.0, &mut bbox);
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
            ptr: p,
            sin_theta,
            cos_theta,
            hasbox,
            bbox,
        }
    }
}

#[derive(Clone, Default)]
pub struct FlipFace<H: Hittable> {
    pub ptr: H,
}

impl<H: Hittable> FlipFace<H> {
    pub fn new(p: H) -> Self {
        Self { ptr: p }
    }
}

impl<H: Hittable> Hittable for FlipFace<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let op = self.ptr.hit(r, t_min, t_max);
        op.as_ref()?;
        let mut rec = op.unwrap();
        rec.front_face = !rec.front_face;
        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.ptr.bounding_box(time0, time1, output_box)
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        self.ptr.pdf_value(o, v)
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        self.ptr.random(o)
    }
}

//--------------------------------------------------------------------------

fn rotate_vec_y(v: &Vec3, sin: f64, cos: f64) -> Vec3 {
    Vec3::new(cos * v.x() - sin * v.z(), v.y(), sin * v.x() + cos * v.z())
}
