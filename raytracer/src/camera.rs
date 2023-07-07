use crate::ray::Ray;
use crate::vec3::{cross, Point3, Vec3};
use std::f64;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: &Point3,
        lookat: &Point3,
        vup: &Vec3,
        vfov: f64, //vertical field-of-view in degrees
        aspect_ratio: f64,
    ) -> Self {
        let theta = vfov.to_radians() / 2.0;
        let h = theta.tan();
        let viewport_h = 2.0 * h;
        let viewport_w = aspect_ratio * viewport_h;

        let w = (*lookfrom - *lookat).unit();
        let u = cross(vup, &w).unit();
        let v = cross(&w, &u).unit();

        let origin = *lookfrom;
        let horizontal = viewport_w * u;
        let vertical = viewport_h * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let dir = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray::new(&self.origin, &dir)
    }
}
