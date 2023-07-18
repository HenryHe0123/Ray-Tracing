use crate::bvh::{bvh_build_static, Object};
use crate::utility::random_double_range;
use crate::utility::vec3::Vec3;
use quote::quote;

pub fn define_static_final_scene() -> proc_macro::TokenStream {
    let mut boxes1 = Vec::new();
    let ground = quote!(Lambertian::new_from_color(&Color::new(0.48, 0.83, 0.53)));
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
            boxes1.push(Object {
                bounding_box_min: Vec3::new(x0, y0, z0),
                code: quote!(Box::new(MyBox::new(&Point3::new(#x0, #y0, #z0),&Point3::new(#x1, #y1, #z1),#ground,))),
            });
        }
    }
    let len1 = boxes1.len();
    let bvh1_code = bvh_build_static(&mut boxes1, 0, len1);

    let light = quote!(DiffuseLight::new_from_color(&Color::new(7.0, 7.0, 7.0)));
    let light_code = quote!(Box::new(FlipFace::new(XZRect::new(
        123., 423., 147., 412., 554., #light,
    ))));

    let moving_sphere_material = quote!(Lambertian::new_from_color(&Color::new(0.7, 0.3, 0.1)));
    let moving_sphere_code = quote!(Box::new(MovingSphere::new(
        &Point3::new(400., 400., 200.),
        &Point3::new(430., 400., 200.),
        50.,
        0.,
        1.,
        #moving_sphere_material,
    )));

    let ball1_code = quote!(Box::new(Sphere::new(
        &Point3::new(260., 150., 45.),
        50.,
        Dielectric::new(1.5),
    )));

    let ball2_code = quote!(Box::new(Sphere::new(
        &Point3::new(0., 150., 145.),
        50.,
        Metal::new(&Color::new(0.8, 0.8, 0.9), 1.0),
    )));

    let boundary_obj_code = quote!(Sphere::new(
        &Point3::new(360., 150., 145.),
        70.,
        Dielectric::new(1.5)
    ));

    let medium_code = quote!(Box::new(ConstantMedium::new_from_color(
        #boundary_obj_code,
        0.2,
        &Color::new(0.2, 0.4, 0.9),
    )));

    let boundary_obj2_code = quote!(Sphere::new(
        &Point3::new(0., 0., 0.),
        5000.,
        Dielectric::new(1.5)
    ));

    let medium2_code = quote!(Box::new(ConstantMedium::new_from_color(
        #boundary_obj2_code,
        0.0001,
        &Color::new(1., 1., 1.),
    )));

    let earth_material_code = quote!(Lambertian::new(ImageTexture::new(
        "raytracer/sources/earthmap.jpg"
    )));
    let globe_code = quote!(Box::new(Sphere::new(
        &Point3::new(400.0, 200.0, 400.0),
        100.0,
        #earth_material_code,
    )));

    let perlin_material_code = quote!(Lambertian::new(NoiseTexture::new(0.1)));
    let perlin_ball_code = quote!(Box::new(Sphere::new(
        &Point3::new(220.0, 280.0, 300.0),
        80.0,
        #perlin_material_code,
    )));

    let mut boxes2 = Vec::new();
    let white = quote!(Lambertian::new_from_color(&Color::new(0.73, 0.73, 0.73)));
    let radius = 10.0;
    let ns = 1000;
    for _j in 0..ns {
        let center = Vec3::random_range(0., 165.);
        let x = center.x();
        let y = center.y();
        let z = center.z();
        let center_code = quote!(&Point3::new(#x,#y,#z));
        boxes2.push(Object {
            bounding_box_min: Vec3::new(x - radius, y - radius, z - radius),
            code: quote!(Box::new(Sphere::new(
            #center_code,
            #radius,
            #white,))),
        });
    }
    let len2 = boxes2.len();
    let bvh2_code = bvh_build_static(&mut boxes2, 0, len2);
    let moved_bvh2_code = quote!(Box::new(Translate::new(
        RotateY::new(*#bvh2_code, 15.0),
        &Vec3::new(-100., 270., 395.),
    )));

    //------------------------------------------------------------------------

    let code = quote! (
        pub fn static_final_scene() -> HittableList {
        let mut objects = HittableList::default();
        objects.add(#bvh1_code);
        objects.add(#light_code);
        objects.add(#moving_sphere_code);
        objects.add(#ball1_code);
        objects.add(#ball2_code);
        objects.add(Box::new(#boundary_obj_code));
        objects.add(#medium_code);
        objects.add(Box::new(#boundary_obj2_code));
        objects.add(#medium2_code);
        objects.add(#globe_code);
        objects.add(#perlin_ball_code);
        objects.add(#moved_bvh2_code);
        objects
        }
    );

    //println!("{}\n", code.clone());
    code.into()
}
