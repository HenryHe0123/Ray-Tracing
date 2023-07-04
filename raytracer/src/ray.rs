use crate::vec3::{Vec3, Point3, Color, unit_vector};

#[derive(Debug, Copy, Clone)]
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

pub fn ray_color(r: &Ray) -> Color {
    let ud = unit_vector(r.dir);
    let t = 0.5 * (ud.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}