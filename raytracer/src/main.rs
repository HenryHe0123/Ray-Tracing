pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod hittable;
pub mod material;
pub mod perlin;
pub mod ray;
pub mod rt_weekend;
pub mod sphere;
pub mod texture;
pub mod vec3;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::ray_color;
use crate::rt_weekend::{random_double, random_double_range};
use crate::sphere::{MovingSphere, Sphere};
use crate::texture::{CheckerTexture, NoiseTexture, Texture};
use crate::vec3::Vec3;
use bvh::BVHNode;
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::rc::Rc;
use std::{fs::File, process::exit};
use vec3::{Color, Point3};

fn main() {
    let path = std::path::Path::new("output/book2/image11.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    //Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let samples_per_pixel: u32 = 100;
    let max_bounce_depth: i32 = 50;
    let height = (width as f64 / aspect_ratio) as u32;

    //World & Camera
    let world;
    let lookfrom;
    let lookat;
    let mut aperture = 0.0;
    let vfov;
    //let mut vfov = 40.0;
    let choice = 0;

    match choice {
        1 => {
            world = random_scene();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world = two_spheres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        _other => {
            world = two_perlin_spheres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
    }

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let camera = Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    //Render
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    for j in 0..height {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, j);
            let mut pixel_color = Color::default();
            for _s in 0..samples_per_pixel {
                let u = ((i as f64) + random_double()) / ((width - 1) as f64);
                let v = (((height - j - 1) as f64) + random_double()) / ((height - 1) as f64);
                let r = camera.get_ray(u, v, 0.0, 1.0);
                pixel_color += ray_color(&r, &world, max_bounce_depth);
            }
            *pixel = image::Rgb(pixel_color.multi_samples_rgb(samples_per_pixel));
            progress.inc(1);
        }
    }
    progress.finish();

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}

//--------------------------------------------------------------------------------

fn random_scene() -> HittableList {
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

fn two_spheres() -> HittableList {
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

fn two_perlin_spheres() -> HittableList {
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
