use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Debug, Copy, Clone, Default)]
pub struct AABB {
    //Axis-Aligned Bounding Boxes
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    pub fn new(min: &Point3, max: &Point3) -> Self {
        Self {
            minimum: *min,
            maximum: *max,
        }
    }

    pub fn center_radius_new(cen: &Point3, radius: f64) -> Self {
        Self {
            minimum: *cen - Point3::new(radius, radius, radius),
            maximum: *cen + Point3::new(radius, radius, radius),
        }
    }

    pub fn min(&self) -> Point3 {
        self.minimum
    }

    pub fn max(&self) -> Point3 {
        self.maximum
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for a in 0..3 {
            let divider = 1.0 / r.direction().index(a);
            let mut t0 = (self.min().index(a) - r.origin().index(a)) * divider;
            let mut t1 = (self.max().index(a) - r.origin().index(a)) * divider;
            if divider < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Point3::new(
        f64::min(box0.min().x(), box1.min().x()),
        f64::min(box0.min().y(), box1.min().y()),
        f64::min(box0.min().z(), box1.min().z()),
    );
    let big = Point3::new(
        f64::max(box0.max().x(), box1.max().x()),
        f64::max(box0.max().y(), box1.max().y()),
        f64::max(box0.max().z(), box1.max().z()),
    );
    AABB::new(&small, &big)
}
