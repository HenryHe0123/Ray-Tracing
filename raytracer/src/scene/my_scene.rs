use crate::camera::Camera;
use crate::hittable::aarect::XYRect;
use crate::hittable::*;
use crate::material::*;
use crate::obj_loader::*;
use crate::texture::ImageTexture;
use crate::utility::vec3::*;

pub fn obj_in_cornell_box() -> (HittableList, Camera) {
    let mut objects = HittableList::standard_cornell_box();

    let material = Lambertian::new(ImageTexture::new("raytracer/sources/spot_texture.jpg"));
    let cow = load("objects/spot_triangulated_good.obj", material, 200.0);

    let light = DiffuseLight::new_from_color(&Color::new(0.55, 0.55, 0.55));
    objects.add(Box::new(XYRect::new(
        -90000., 90000., -90000., 90000., -4000., light,
    )));

    objects.add(Box::new(Translate::new(cow, &Vec3::new(278., 144., 178.))));

    //-----------------------------------------
    (objects, Camera::default_cornell_box())
}
