pub mod my_scene;

use crate::hittable::aarect::*;
use crate::hittable::bvh::BVHNode;
use crate::hittable::constant_medium::*;
use crate::hittable::mybox::*;
use crate::hittable::sphere::*;
use crate::hittable::{FlipFace, HittableList, RotateY, Translate};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::obj_loader::*;
use crate::texture::*;
use crate::utility::vec3::*;
use crate::utility::*;
use raytracer_codegen::impl_static_final_scene;

impl_static_final_scene!();

pub fn random_scene() -> HittableList {
    let mut world = HittableList::default();
    let checker =
        CheckerTexture::new_from_color(&Color::new(0.2, 0.3, 0.1), &Color::new(0.9, 0.9, 0.9));
    let material_ground = Lambertian::new(checker);
    world.add(Box::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    ))); //ground

    let mut list = HittableList::default();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new_from_color(&albedo);
                    let center2 = center + Vec3::new(0., random_double_range(0., 0.5), 0.);
                    //list.add(Box::new(Sphere::new(&center, 0.2, sphere_material)));
                    list.add(Box::new(MovingSphere::new(
                        &center,
                        &center2,
                        0.2,
                        0.0,
                        1.0,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Metal::new(&albedo, fuzz);
                    list.add(Box::new(Sphere::new(&center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    list.add(Box::new(Sphere::new(&center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Dielectric::new(1.5);
    list.add(Box::new(Sphere::new(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Lambertian::new_from_color(&Color::new(0.4, 0.2, 0.1));
    list.add(Box::new(Sphere::new(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0);
    list.add(Box::new(Sphere::new(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    //triangle test
    //let checker = SolidColor::new(&Color::purple());
    // world.add(Box::new(Triangle::new(
    //     &Point3::new(12.0, 2.0, 2.9),
    //     &Point3::new(12.0, 2.0, 2.6),
    //     &Point3::new(12.0, 1.7, 2.75),
    //     DiffuseLight::new_from_color(&Color::new(7.0, 7.0, 7.0)),
    // )));
    //
    world.add(Box::new(BVHNode::new(list, 0.0, 1.0)));
    world
}

pub fn two_spheres() -> HittableList {
    let mut objects = HittableList::default();
    let checker =
        CheckerTexture::new_from_color(&Color::new(0.2, 0.3, 0.1), &Color::new(0.9, 0.9, 0.9));
    let material_checker = Lambertian::new(checker);
    objects.add(Box::new(Sphere::new(
        &Point3::new(0.0, -10.0, 0.0),
        10.0,
        material_checker.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        &Point3::new(0.0, 10.0, 0.0),
        10.0,
        material_checker,
    )));
    objects
}

pub fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::default();
    let perlin_text = NoiseTexture::new(4.0);
    let material = Lambertian::new(perlin_text);
    objects.add(Box::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        material,
    )));
    objects
}

pub fn earth() -> HittableList {
    let earth_text = ImageTexture::new("raytracer/sources/earthmap.jpg");
    let earth_surface = Lambertian::new(earth_text);
    let globe = Box::new(Sphere::new(&Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));
    let mut world = HittableList::default();
    world.add(globe);
    world
}

pub fn simple_light() -> HittableList {
    let mut objects = HittableList::default();
    let perlin_text = NoiseTexture::new(4.0);
    let material = Lambertian::new(perlin_text);
    objects.add(Box::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        material,
    )));

    let diffuse_light = DiffuseLight::new_from_color(&Color::new(4.0, 4.0, 4.0));
    objects.add(Box::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        diffuse_light.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        &Point3::new(0.0, 7.0, 0.0),
        2.0,
        diffuse_light,
    )));
    objects
}

pub fn cornell_box() -> HittableList {
    let mut objects = HittableList::default();
    let red = Lambertian::new_from_color(&Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new_from_color(&Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new_from_color(&Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new_from_color(&Color::new(15.0, 15.0, 15.0));
    objects.add(Box::new(YZRect::new(0., 555., 0., 555., 555., green)));
    objects.add(Box::new(YZRect::new(0., 555., 0., 555., 0., red)));
    //flip light
    objects.add(Box::new(FlipFace::new(XZRect::new(
        213., 343., 227., 332., 554., light,
    ))));
    //
    objects.add(Box::new(XZRect::new(0., 555., 0., 555., 0., white.clone())));
    objects.add(Box::new(XZRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    objects.add(Box::new(XYRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));

    //let aluminum = Box::new(Metal::new(&Color::new(0.8, 0.85, 0.88), 0.0));
    let box1 = MyBox::new(
        &Point3::new(0., 0., 0.),
        &Point3::new(165., 330., 165.),
        white,
    );
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Box::new(Translate::new(box1, &Vec3::new(265., 0., 295.)));
    objects.add(box1);

    let glass = Dielectric::new(1.5);
    objects.add(Box::new(Sphere::new(
        &Point3::new(190., 90., 190.),
        90.,
        glass,
    )));

    objects
}

pub fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::default();
    let red = Lambertian::new_from_color(&Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new_from_color(&Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new_from_color(&Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new_from_color(&Color::new(7.0, 7.0, 7.0));
    objects.add(Box::new(YZRect::new(0., 555., 0., 555., 555., green)));
    objects.add(Box::new(YZRect::new(0., 555., 0., 555., 0., red)));
    objects.add(Box::new(XZRect::new(113., 343., 127., 432., 554., light)));
    objects.add(Box::new(XZRect::new(0., 555., 0., 555., 0., white.clone())));
    objects.add(Box::new(XZRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    objects.add(Box::new(XYRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));

    let box1 = MyBox::new(
        &Point3::new(0., 0., 0.),
        &Point3::new(165., 330., 165.),
        white.clone(),
    );
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Translate::new(box1, &Vec3::new(265., 0., 295.));

    let box2 = MyBox::new(
        &Point3::new(0., 0., 0.),
        &Point3::new(165., 165., 165.),
        white,
    );
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Translate::new(box2, &Vec3::new(130., 0., 65.));
    objects.add(Box::new(ConstantMedium::new_from_color(
        box1,
        0.01,
        &Color::new(0., 0., 0.),
    )));
    objects.add(Box::new(ConstantMedium::new_from_color(
        box2,
        0.01,
        &Color::new(1., 1., 1.),
    )));
    objects
}

pub fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::default();
    let ground = Lambertian::new_from_color(&Color::new(0.48, 0.83, 0.53));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(Box::new(MyBox::new(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    let mut objects = HittableList::default();
    objects.add(Box::new(BVHNode::new(boxes1, 0.0, 1.0)));

    let light = DiffuseLight::new_from_color(&Color::new(7.0, 7.0, 7.0));
    objects.add(Box::new(FlipFace::new(XZRect::new(
        123., 423., 147., 412., 554., light,
    ))));

    let center1 = Point3::new(400., 400., 200.);
    let center2 = Point3::new(430., 400., 200.);
    let moving_sphere_material = Lambertian::new_from_color(&Color::new(0.7, 0.3, 0.1));
    objects.add(Box::new(MovingSphere::new(
        &center1,
        &center2,
        50.,
        0.,
        1.,
        moving_sphere_material,
    )));

    objects.add(Box::new(Sphere::new(
        &Point3::new(260., 150., 45.),
        50.,
        Dielectric::new(1.5),
    )));
    objects.add(Box::new(Sphere::new(
        &Point3::new(0., 150., 145.),
        50.,
        Metal::new(&Color::new(0.8, 0.8, 0.9), 1.0),
    )));

    let boundary = Sphere::new(&Point3::new(360., 150., 145.), 70., Dielectric::new(1.5));
    objects.add(Box::new(boundary.clone()));
    objects.add(Box::new(ConstantMedium::new_from_color(
        boundary,
        0.2,
        &Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Sphere::new(&Point3::new(0., 0., 0.), 5000., Dielectric::new(1.5));
    objects.add(Box::new(ConstantMedium::new_from_color(
        boundary,
        0.0001,
        &Color::new(1., 1., 1.),
    )));

    let earth_text = ImageTexture::new("raytracer/sources/earthmap.jpg");
    let earth_material = Lambertian::new(earth_text);
    let globe = Box::new(Sphere::new(
        &Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_material,
    ));
    objects.add(globe);

    let perlin_text = NoiseTexture::new(0.1);
    let perlin_material = Lambertian::new(perlin_text);
    objects.add(Box::new(Sphere::new(
        &Point3::new(220.0, 280.0, 300.0),
        80.0,
        perlin_material,
    )));

    let mut boxes2 = HittableList::default();
    let white = Lambertian::new_from_color(&Color::new(0.73, 0.73, 0.73));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Box::new(Sphere::new(
            &Point3::random_range(0., 165.),
            10.,
            white.clone(),
        )));
    }

    let bvh_ptr = BVHNode::new(boxes2, 0.0, 1.0);
    objects.add(Box::new(Translate::new(
        RotateY::new(bvh_ptr, 15.0),
        &Vec3::new(-100., 270., 395.),
    )));

    objects
}

//-----------------------------------my test work------------------------------------------------

pub fn golden_cow_in_cornell_box() -> HittableList {
    let mut objects = HittableList::default();
    let red = Lambertian::new_from_color(&Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new_from_color(&Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new_from_color(&Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new_from_color(&Color::new(15.0, 15.0, 15.0));
    objects.add(Box::new(YZRect::new(0., 555., 0., 555., 555., green)));
    objects.add(Box::new(YZRect::new(0., 555., 0., 555., 0., red)));
    //flip light
    objects.add(Box::new(FlipFace::new(XZRect::new(
        213., 343., 227., 332., 554., light,
    ))));
    //
    objects.add(Box::new(XZRect::new(0., 555., 0., 555., 0., white.clone())));
    objects.add(Box::new(XZRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    objects.add(Box::new(XYRect::new(0., 555., 0., 555., 555., white)));

    let light = DiffuseLight::new_from_color(&Color::new(0.55, 0.55, 0.55));
    objects.add(Box::new(XYRect::new(
        -90000., 90000., -90000., 90000., -4000., light,
    )));

    let cow = load_naive("objects/spot_triangulated_good.obj", Metal::gold(), 200.0);

    objects.add(Box::new(Translate::new(cow, &Vec3::new(278., 144., 178.))));

    objects
}

pub fn car_in_cornell_box() -> HittableList {
    let mut objects = HittableList::default();
    let red = Lambertian::new_from_color(&Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new_from_color(&Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new_from_color(&Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new_from_color(&Color::new(15.0, 15.0, 15.0));
    objects.add(Box::new(YZRect::new(0., 555., 0., 555., 555., green)));
    objects.add(Box::new(YZRect::new(0., 555., 0., 555., 0., red)));
    //flip light
    objects.add(Box::new(FlipFace::new(XZRect::new(
        213., 343., 227., 332., 554., light,
    ))));
    //
    objects.add(Box::new(XZRect::new(0., 555., 0., 555., 0., white.clone())));
    objects.add(Box::new(XZRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    objects.add(Box::new(XYRect::new(0., 555., 0., 555., 555., white)));

    let light = DiffuseLight::new_from_color(&Color::new(0.55, 0.55, 0.55));
    objects.add(Box::new(XYRect::new(
        -90000., 90000., -90000., 90000., -4000., light,
    )));

    let obj = load_naive("objects/car.obj", Metal::gold(), 120.0);

    objects.add(Box::new(Translate::new(
        RotateY::new(obj, 180.0),
        &Vec3::new(278., 144.0 - 100.0, 178.),
    )));

    objects
}
