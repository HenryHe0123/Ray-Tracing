pub mod aabb;

use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::utility::random_int_range;
use crate::utility::ray::Ray;
use aabb::{surrounding_box, AABB};
use std::cmp::Ordering;

#[derive(Default)]
pub struct BVHNode {
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
    box_: AABB,
}

impl BVHNode {
    pub fn new(mut list: HittableList, time0: f64, time1: f64) -> Self {
        let len = list.objects.len();
        Self::build(&mut list.objects, 0, len, time0, time1)
    }

    pub fn construct(
        left: Option<Box<dyn Hittable>>,
        right: Option<Box<dyn Hittable>>,
        time0: f64,
        time1: f64,
    ) -> Self {
        let mut box_left = AABB::default();
        let mut box_right = AABB::default();
        let bool1 = if left.is_some() {
            left.as_ref()
                .unwrap()
                .bounding_box(time0, time1, &mut box_left)
        } else {
            false
        };
        let bool2 = if right.is_some() {
            right
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, &mut box_right)
        } else {
            false
        };

        let box_ = if bool1 {
            if bool2 {
                surrounding_box(&box_left, &box_right)
            } else {
                box_left
            }
        } else {
            box_right
        };

        BVHNode { left, right, box_ }
    }

    pub fn build(
        objects: &mut Vec<Box<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = random_int_range(0, 2);
        let object_span = end - start;
        let left: Option<Box<dyn Hittable>>;
        let right: Option<Box<dyn Hittable>>;
        match object_span {
            1 => {
                left = Some(objects.remove(start));
                right = None;
            }
            2 => {
                if Self::box_compare(
                    objects[start].as_ref(),
                    objects[start + 1].as_ref(),
                    axis as usize,
                ) {
                    right = Some(objects.remove(start + 1));
                    left = Some(objects.remove(start));
                } else {
                    left = Some(objects.remove(start + 1));
                    right = Some(objects.remove(start));
                }
            }
            _other => {
                objects[start..end].sort_by(|a, b| {
                    BVHNode::box_compare_order(a.as_ref(), b.as_ref(), axis as usize)
                });
                let mid = start + object_span / 2;
                right = Some(Box::new(Self::build(objects, mid, end, time0, time1)));
                left = Some(Box::new(Self::build(objects, start, mid, time0, time1)));
            }
        }
        Self::construct(left, right, time0, time1)
    }

    fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> bool {
        let mut box_a = AABB::default();
        let mut box_b = AABB::default();
        if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
            panic!("No bounding box in BVHNode constructor.");
        }
        box_a.min()[axis] < box_b.min()[axis]
    }

    fn box_compare_order(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> Ordering {
        let mut box_a = AABB::default();
        let mut box_b = AABB::default();
        if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
            panic!("No bounding box in BVHNode constructor.");
        }

        if box_a.min()[axis] < box_b.min()[axis] {
            Ordering::Less
        } else if box_a.min()[axis] == box_b.min()[axis] {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        if !self.box_.hit(r, t_min, t_max) {
            return None;
        }
        let hit_left = match self.left.as_ref() {
            Some(left) => left.hit(r, t_min, t_max),
            None => None,
        };
        if hit_left.is_some() {
            t_max = hit_left.as_ref().unwrap().t;
        }
        let hit_right = match self.right.as_ref() {
            Some(right) => right.hit(r, t_min, t_max),
            None => None,
        };
        if hit_right.is_some() {
            hit_right
        } else {
            hit_left
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.box_;
        true
    }
}
