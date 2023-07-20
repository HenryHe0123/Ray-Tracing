use crate::utility::vec3::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub tm: f64,
}

impl Ray {
    pub fn new(orig: &Point3, dir: &Vec3, time: f64) -> Self {
        Ray {
            orig: *orig,
            dir: *dir,
            tm: time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn origin_borrow(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn direction_borrow(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }
}
