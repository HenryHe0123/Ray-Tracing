use crate::hittable::bvh::BVHNode;
use crate::hittable::triangle::Triangle;
use crate::hittable::HittableList;
use crate::material::Material;
use crate::utility::vec3::*;
use tobj::{load_obj, LoadOptions};

pub fn load<M: Material + Clone + 'static>(pathname: &str, mat: M, scale: f64) -> HittableList {
    let (models, _) =
        load_obj(pathname, &LoadOptions::default()).expect("Failed to load .obj file.");
    let mut objects = HittableList::new();
    for m in models {
        let positions = &m.mesh.positions; //points position
        let indices = &m.mesh.indices; //points index (maybe joint)
        let mut points = Vec::new();
        let mut triangles = HittableList::new();

        for i in (0..positions.len()).step_by(3) {
            points.push(Point3::new(positions[i], positions[i + 1], positions[i + 2]) * scale);
        }
        for i in (0..indices.len() - indices.len() % 3).step_by(3) {
            triangles.add(Box::new(Triangle::new(
                &points[indices[i] as usize],
                &points[indices[i + 1] as usize],
                &points[indices[i + 2] as usize],
                mat.clone(),
            )));
        }
        objects.add(Box::new(BVHNode::new(triangles, 0., 1.)));
    }
    objects
}
