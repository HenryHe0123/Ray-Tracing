use crate::aarect::{XYRect, XZRect, YZRect};
use crate::bvh::BVHNode;
use crate::constant_medium::ConstantMedium;
use crate::hittable::{HittableList, RotateY, Translate};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::mybox::MyBox;
use crate::rt_weekend::{random_double, random_double_range};
use crate::sphere::{MovingSphere, Sphere};
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture, Texture};
use crate::vec3::{Color, Point3, Vec3};
use std::rc::Rc;

pub fn random_scene() -> HittableList {
    let mut world = HittableList::default();
    let checker = Rc::new(CheckerTexture::new(
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    )) as Rc<dyn Texture>;
    let material_ground = Rc::new(Lambertian::new_from_ptr(&checker));
    world.add(Rc::new(Sphere::new(
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
                    let sphere_material = Rc::new(Lambertian::new(&albedo));
                    let center2 = center + Vec3::new(0., random_double_range(0., 0.5), 0.);
                    //list.add(Rc::new(Sphere::new(&center, 0.2, sphere_material)));
                    list.add(Rc::new(MovingSphere::new(
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
                    let sphere_material = Rc::new(Metal::new(&albedo, fuzz));
                    list.add(Rc::new(Sphere::new(&center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    list.add(Rc::new(Sphere::new(&center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Rc::new(Dielectric::new(1.5));
    list.add(Rc::new(Sphere::new(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Rc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    list.add(Rc::new(Sphere::new(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Rc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    list.add(Rc::new(Sphere::new(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world.add(Rc::new(BVHNode::new(&list, 0.0, 1.0)));
    world
}

pub fn two_spheres() -> HittableList {
    let mut objects = HittableList::default();
    let checker = Rc::new(CheckerTexture::new(
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    )) as Rc<dyn Texture>;
    let material_checker = Rc::new(Lambertian::new_from_ptr(&checker));
    objects.add(Rc::new(Sphere::new(
        &Point3::new(0.0, -10.0, 0.0),
        10.0,
        material_checker.clone(),
    )));
    objects.add(Rc::new(Sphere::new(
        &Point3::new(0.0, 10.0, 0.0),
        10.0,
        material_checker,
    )));
    objects
}

pub fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::default();
    let perlin_text = Rc::new(NoiseTexture::new(4.0)) as Rc<dyn Texture>;
    let material = Rc::new(Lambertian::new_from_ptr(&perlin_text));
    objects.add(Rc::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material.clone(),
    )));
    objects.add(Rc::new(Sphere::new(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        material,
    )));
    objects
}

pub fn earth() -> HittableList {
    let earth_text =
        Rc::new(ImageTexture::new("raytracer/sources/earthmap.jpg")) as Rc<dyn Texture>;
    let earth_surface = Rc::new(Lambertian::new_from_ptr(&earth_text));
    let globe = Rc::new(Sphere::new(&Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));
    let mut world = HittableList::default();
    world.add(globe);
    world
}

pub fn simple_light() -> HittableList {
    let mut objects = HittableList::default();
    let perlin_text = Rc::new(NoiseTexture::new(4.0)) as Rc<dyn Texture>;
    let material = Rc::new(Lambertian::new_from_ptr(&perlin_text));
    objects.add(Rc::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material.clone(),
    )));
    objects.add(Rc::new(Sphere::new(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        material,
    )));

    let diffuse_light = Rc::new(DiffuseLight::new(&Color::new(4.0, 4.0, 4.0)));
    objects.add(Rc::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        diffuse_light.clone(),
    )));
    objects.add(Rc::new(Sphere::new(
        &Point3::new(0.0, 7.0, 0.0),
        2.0,
        diffuse_light,
    )));
    objects
}

pub fn cornell_box() -> HittableList {
    let mut objects = HittableList::default();
    let red = Rc::new(Lambertian::new(&Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(&Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(&Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(&Color::new(15.0, 15.0, 15.0)));
    objects.add(Rc::new(YZRect::new(0., 555., 0., 555., 555., green)));
    objects.add(Rc::new(YZRect::new(0., 555., 0., 555., 0., red)));
    objects.add(Rc::new(XZRect::new(213., 343., 227., 332., 554., light)));
    objects.add(Rc::new(XZRect::new(0., 555., 0., 555., 0., white.clone())));
    objects.add(Rc::new(XZRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    objects.add(Rc::new(XYRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    let box1 = Rc::new(MyBox::new(
        &Point3::new(0., 0., 0.),
        &Point3::new(165., 330., 165.),
        white.clone(),
    ));
    let box1 = Rc::new(RotateY::new(box1, 15.0));
    let box1 = Rc::new(Translate::new(box1, &Vec3::new(265., 0., 295.)));
    let box2 = Rc::new(MyBox::new(
        &Point3::new(0., 0., 0.),
        &Point3::new(165., 165., 165.),
        white,
    ));
    let box2 = Rc::new(RotateY::new(box2, -18.0));
    let box2 = Rc::new(Translate::new(box2, &Vec3::new(130., 0., 65.)));
    objects.add(box1);
    objects.add(box2);
    objects
}

pub fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::default();
    let red = Rc::new(Lambertian::new(&Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(&Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(&Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(&Color::new(7.0, 7.0, 7.0)));
    objects.add(Rc::new(YZRect::new(0., 555., 0., 555., 555., green)));
    objects.add(Rc::new(YZRect::new(0., 555., 0., 555., 0., red)));
    objects.add(Rc::new(XZRect::new(113., 343., 127., 432., 554., light)));
    objects.add(Rc::new(XZRect::new(0., 555., 0., 555., 0., white.clone())));
    objects.add(Rc::new(XZRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    objects.add(Rc::new(XYRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));

    let box1 = Rc::new(MyBox::new(
        &Point3::new(0., 0., 0.),
        &Point3::new(165., 330., 165.),
        white.clone(),
    ));
    let box1 = Rc::new(RotateY::new(box1, 15.0));
    let box1 = Rc::new(Translate::new(box1, &Vec3::new(265., 0., 295.)));
    let box2 = Rc::new(MyBox::new(
        &Point3::new(0., 0., 0.),
        &Point3::new(165., 165., 165.),
        white,
    ));
    let box2 = Rc::new(RotateY::new(box2, -18.0));
    let box2 = Rc::new(Translate::new(box2, &Vec3::new(130., 0., 65.)));
    objects.add(Rc::new(ConstantMedium::new(
        box1,
        0.01,
        &Color::new(0., 0., 0.),
    )));
    objects.add(Rc::new(ConstantMedium::new(
        box2,
        0.01,
        &Color::new(1., 1., 1.),
    )));
    objects
}

pub fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::default();
    let ground = Rc::new(Lambertian::new(&Color::new(0.48, 0.83, 0.53)));
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
            boxes1.add(Rc::new(MyBox::new(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    let mut objects = HittableList::default();
    objects.add(Rc::new(BVHNode::new(&boxes1, 0.0, 1.0)));

    let light = Rc::new(DiffuseLight::new(&Color::new(7.0, 7.0, 7.0)));
    objects.add(Rc::new(XZRect::new(123., 423., 147., 412., 554., light)));

    let center1 = Point3::new(400., 400., 200.);
    let center2 = Point3::new(430., 400., 200.);
    let moving_sphere_material = Rc::new(Lambertian::new(&Color::new(0.7, 0.3, 0.1)));
    objects.add(Rc::new(MovingSphere::new(
        &center1,
        &center2,
        50.,
        0.,
        1.,
        moving_sphere_material,
    )));

    objects.add(Rc::new(Sphere::new(
        &Point3::new(260., 150., 45.),
        50.,
        Rc::new(Dielectric::new(1.5)),
    )));
    objects.add(Rc::new(Sphere::new(
        &Point3::new(0., 150., 145.),
        50.,
        Rc::new(Metal::new(&Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Rc::new(Sphere::new(
        &Point3::new(360., 150., 145.),
        70.,
        Rc::new(Dielectric::new(1.5)),
    ));
    objects.add(boundary.clone());
    objects.add(Rc::new(ConstantMedium::new(
        boundary,
        0.2,
        &Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Rc::new(Sphere::new(
        &Point3::new(0., 0., 0.),
        5000.,
        Rc::new(Dielectric::new(1.5)),
    ));
    objects.add(Rc::new(ConstantMedium::new(
        boundary,
        0.0001,
        &Color::new(1., 1., 1.),
    )));

    let earth_text =
        Rc::new(ImageTexture::new("raytracer/sources/earthmap.jpg")) as Rc<dyn Texture>;
    let earth_material = Rc::new(Lambertian::new_from_ptr(&earth_text));
    let globe = Rc::new(Sphere::new(
        &Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_material,
    ));
    objects.add(globe);

    let perlin_text = Rc::new(NoiseTexture::new(0.1)) as Rc<dyn Texture>;
    let perlin_material = Rc::new(Lambertian::new_from_ptr(&perlin_text));
    objects.add(Rc::new(Sphere::new(
        &Point3::new(220.0, 280.0, 300.0),
        80.0,
        perlin_material,
    )));

    let mut boxes2 = HittableList::default();
    let white = Rc::new(Lambertian::new(&Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Rc::new(Sphere::new(
            &Point3::random_range(0., 165.),
            10.,
            white.clone(),
        )));
    }

    let bvh_ptr = Rc::new(BVHNode::new(&boxes2, 0.0, 1.0));
    objects.add(Rc::new(Translate::new(
        Rc::new(RotateY::new(bvh_ptr, 15.0)),
        &Vec3::new(-100., 270., 395.),
    )));

    objects
}
