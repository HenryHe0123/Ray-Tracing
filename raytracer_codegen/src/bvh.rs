use crate::utility::random_int_range;
use crate::utility::vec3::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::cmp::Ordering;

pub struct Object {
    pub bounding_box_min: Vec3,
    pub code: TokenStream,
}

fn box_compare_order(a: &Object, b: &Object, axis: usize) -> Ordering {
    if a.bounding_box_min[axis] < b.bounding_box_min[axis] {
        Ordering::Less
    } else if a.bounding_box_min[axis] == b.bounding_box_min[axis] {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

fn box_compare(a: &Object, b: &Object, axis: usize) -> bool {
    a.bounding_box_min[axis] < b.bounding_box_min[axis]
}

pub fn bvh_build_static(objects: &mut Vec<Object>, start: usize, end: usize) -> TokenStream {
    let axis = random_int_range(0, 2) as usize;
    let span = end - start;

    match span {
        1 => {
            let sub_code = objects[start].code.clone();
            quote!(#sub_code)
        } //code should be wrapped in Box
        2 => {
            let left;
            let right;
            if box_compare(&objects[start], &objects[start + 1], axis) {
                right = &objects[start + 1];
                left = &objects[start];
            } else {
                left = &objects[start + 1];
                right = &objects[start];
            }
            let left = left.code.clone();
            let right = right.code.clone();
            quote! (Box::new(BVHNode::construct(Some(#left), Some(#right), 0.0, 1.0)))
        }
        _other => {
            objects[start..end].sort_by(|a, b| box_compare_order(a, b, axis));
            let mid = start + span / 2;
            let right = bvh_build_static(objects, mid, end);
            let left = bvh_build_static(objects, start, mid);
            quote! (Box::new(BVHNode::construct(Some(#left), Some(#right), 0.0, 1.0)))
        }
    }
}
