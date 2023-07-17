pub mod aabb;

use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::ray::Ray;
use crate::utility::random_int_range;
use aabb::{surrounding_box, AABB};
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct BVHNode {
    left: Option<Arc<dyn Hittable>>,
    right: Option<Arc<dyn Hittable>>,
    box_: AABB,
}

impl BVHNode {
    pub fn new(list: &HittableList, time0: f64, time1: f64) -> Self {
        Self::build(
            &mut list.objects.clone(),
            0,
            list.objects.len(),
            time0,
            time1,
        )
    }

    pub fn build(
        objects: &mut [Arc<dyn Hittable>],
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = random_int_range(0, 2);
        let object_span = end - start;
        let mut node = Self::default();
        match object_span {
            1 => {
                node.left = Some(objects[start].clone());
                node.right = Some(objects[start].clone());
            }
            2 => {
                if Self::box_compare(&objects[start], &objects[start + 1], axis as usize) {
                    node.left = Some(objects[start].clone());
                    node.right = Some(objects[start + 1].clone());
                } else {
                    node.left = Some(objects[start + 1].clone());
                    node.right = Some(objects[start].clone());
                }
            }
            _other => {
                objects[start..end].sort_by(|a, b| BVHNode::box_compare_order(a, b, axis as usize));
                let mid = start + object_span / 2;
                node.left = Some(Arc::new(Self::build(objects, start, mid, time0, time1)));
                node.right = Some(Arc::new(Self::build(objects, mid, end, time0, time1)));
            }
        }
        let mut box_left = AABB::default();
        let mut box_right = AABB::default();
        if !node
            .left
            .as_ref()
            .unwrap()
            .bounding_box(time0, time1, &mut box_left)
            || !node
                .right
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, &mut box_right)
        {
            panic!("No bounding box in BVHNode constructor.");
        }
        node.box_ = surrounding_box(&box_left, &box_right);
        node
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> bool {
        let mut box_a = AABB::default();
        let mut box_b = AABB::default();
        if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
            panic!("No bounding box in BVHNode constructor.");
        }
        box_a.min()[axis] < box_b.min()[axis]
    }

    fn box_compare_order(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
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
