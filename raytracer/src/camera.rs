use crate::ray::Ray;
use crate::rt_weekend::random_double_range;
use crate::vec3::{cross, Point3, Vec3};
use std::f64;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: &Point3,
        lookat: &Point3,
        vup: &Vec3,
        vfov: f64, //vertical field-of-view in degrees
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians() / 2.0;
        let h = theta.tan();
        let viewport_h = 2.0 * h;
        let viewport_w = aspect_ratio * viewport_h;

        let w = (*lookfrom - *lookat).unit();
        let u = cross(vup, &w).unit();
        let v = cross(&w, &u); //already unit

        let origin = *lookfrom;
        let horizontal = focus_dist * viewport_w * u;
        let vertical = focus_dist * viewport_h * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            _w: w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, time0: f64, time1: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        let orig = self.origin + offset;
        let dir = self.lower_left_corner + s * self.horizontal + t * self.vertical - orig;
        Ray::new(&orig, &dir, random_double_range(time0, time1))
    }
}
