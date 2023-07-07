use crate::hittable::{HitRecord, Hittable};
use crate::vec3::{Color, Point3, Vec3};
use std::f64::INFINITY;

#[derive(Debug, Copy, Clone, Default)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: &Point3, dir: &Vec3) -> Self {
        Ray {
            orig: *orig,
            dir: *dir,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

pub fn ray_color(r: &Ray, world: &impl Hittable, depth: i32) -> Color {
    if depth <= 0 {
        //prevent endless recursion
        return Color::default();
    }
    let mut rec = HitRecord::default();
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if let Some(mat) = rec.mat_ptr.clone() {
            if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }
        }
        return Color::default();
    }
    let ud = r.direction().unit();
    let t = 0.5 * (ud.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
