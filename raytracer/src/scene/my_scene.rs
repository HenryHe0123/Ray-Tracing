use crate::camera::Camera;
use crate::hittable::aarect::XYRect;
use crate::hittable::*;
use crate::material::*;
use crate::obj_loader::*;
use crate::utility::vec3::*;

pub fn obj_in_cornell_box() -> (HittableList, Camera) {
    let mut objects = HittableList::standard_cornell_box();

    let rocket = load_pro("rocket", 0.24, &Color::black());

    let light = DiffuseLight::new_from_color(&Color::new(0.55, 0.55, 0.55));
    objects.add(Box::new(XYRect::new(
        -90000., 90000., -90000., 90000., -4000., light,
    )));

    objects.add(Box::new(Translate::new(
        RotateX::new(rocket, -90.0),
        &Vec3::new(278., 150., 200.),
    )));

    //-----------------------------------------
    (objects, Camera::default_cornell_box())
}
