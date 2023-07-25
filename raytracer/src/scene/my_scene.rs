use crate::camera::Camera;
use crate::hittable::aarect::*;
use crate::hittable::sphere::Sphere;
use crate::hittable::*;
use crate::material::*;
use crate::obj_loader::*;
use crate::utility::vec3::*;
use std::f64::INFINITY;

pub fn final_work() -> (HittableList, Camera) {
    let mut objects = HittableList::new();

    let ocean = load_pro("Ocean", Vec3::new(6000., 1500., 6000.), &Color::blue());
    objects.add(Box::new(ocean));
    let carrier = load_pro("carrier", Vec3::same(500.0), &Color::black());
    objects.add(Box::new(Translate::new(
        RotateZ::new(RotateY::new(carrier, -30.0), -0.5),
        &Vec3::new(-1000., 178., -120.),
    )));

    let light = DiffuseLight::new_from_color(&(Color::new(0.5, 0.7, 1.0) * 0.8));
    objects.add(Box::new(FlipFace::new(XZRect::new(
        -INFINITY, INFINITY, -INFINITY, INFINITY, 4000., light,
    ))));

    let extra_light = DiffuseLight::new_from_color(&Color::same(1.2));
    objects.add(Box::new(Sphere::new(
        &Point3::new(3000., 2800., 0.0),
        1200.,
        extra_light,
    )));

    //------------------------------------------------------------------------------------------

    let fighter0 = su27();
    let fighter1 = su27();
    let fighter2 = su27();
    let fighter3 = su27();
    let fighter4 = su27();
    let fighter5 = su27();
    let fighter6 = su27();
    let fighter7 = su27();
    objects.add(Box::new(Translate::new(
        RotateZ::new(RotateY::new(fighter0, 60.0), 5.0),
        &Vec3::new(60., 210., 95.),
    )));
    objects.add(Box::new(Translate::new(
        RotateZ::new(RotateY::new(fighter1, 60.0), 5.0),
        &Vec3::new(20., 240., 65.),
    )));
    objects.add(Box::new(Translate::new(
        RotateY::new(fighter2, 45.0),
        &Vec3::new(320., 141., 105.),
    )));
    objects.add(Box::new(Translate::new(
        RotateZ::new(RotateX::new(RotateY::new(fighter3, 115.0), -7.0), 2.0),
        &Vec3::new(520., 140., 30.),
    )));
    objects.add(Box::new(Translate::new(
        RotateZ::new(RotateX::new(RotateY::new(fighter4, 115.0), -7.0), 2.0),
        &Vec3::new(500., 140., -30.),
    )));
    objects.add(Box::new(Translate::new(
        RotateX::new(RotateY::new(fighter5, 115.0), -7.0),
        &Vec3::new(560., 140., -10.),
    )));
    objects.add(Box::new(Translate::new(
        RotateZ::new(RotateY::new(fighter6, 65.0), 2.0),
        &Vec3::new(110., 148., -94.),
    )));
    objects.add(Box::new(Translate::new(
        RotateZ::new(RotateY::new(fighter7, 65.0), -12.0),
        &Vec3::new(-200., 188., -200.),
    )));

    let missile = load_pro("missile", Vec3::same(0.2), &Color::black());
    objects.add(Box::new(Translate::new(
        RotateY::new(missile, -60.0),
        &Vec3::new(550., 128., -5.),
    )));

    let helicopter1 = load_pro("Helicopter", Vec3::same(0.9), &Color::black());
    objects.add(Box::new(Translate::new(
        RotateY::new(helicopter1, -90.0),
        &Vec3::new(-800., 200., -200.),
    )));
    let helicopter2 = load_pro("Helicopter", Vec3::same(0.9), &Color::black());
    objects.add(Box::new(Translate::new(
        RotateY::new(helicopter2, -90.0),
        &Vec3::new(-800., 140., -150.),
    )));

    let mat = Metal::silver();
    let destroyer = load_naive("objects/Destroyer.obj", mat, 30.0);
    objects.add(Box::new(Translate::new(
        RotateY::new(RotateX::new(destroyer, -90.0), -15.0),
        &Vec3::new(-1000., 90., 550.),
    )));
    let destroyer2 = load_naive("objects/Destroyer.obj", mat, 30.0);
    objects.add(Box::new(Translate::new(
        RotateY::new(RotateX::new(destroyer2, -90.0), -35.0),
        &Vec3::new(-600., 90., -700.),
    )));

    let mat2 = Lambertian::new_from_color(&Color::same(0.1));
    let b2 = load_naive("objects/B2.obj", mat2, 20.0);
    objects.add(Box::new(Translate::new(
        RotateX::new(RotateY::new(RotateZ::new(b2, 6.0), 30.0), -10.0),
        &Vec3::new(-1200., 440., 330.),
    )));

    //------------------------------------------------------------------------------------------

    let world = HittableList::bvh(objects);
    let lookfrom = Point3::new(600.0, 150.0, 0.0);
    let lookat = Point3::new(0.0, 70.0, 0.0);
    (world, Camera::for_final(&lookfrom, &lookat))
}
