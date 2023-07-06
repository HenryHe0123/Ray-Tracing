use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_h = 2.0;
        let viewport_w = aspect_ratio * viewport_h;
        let focal_length = 1.0;
        let origin = Point3::default();
        let horizontal = Vec3::new(viewport_w, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_h, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

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

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
