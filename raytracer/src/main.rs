pub mod aabb;
pub mod aarect;
pub mod bvh;
pub mod camera;
pub mod constant_medium;
pub mod hittable;
pub mod material;
pub mod mybox;
pub mod onb;
pub mod perlin;
pub mod ray;
pub mod rt_weekend;
pub mod scene;
pub mod sphere;
pub mod texture;
pub mod vec3;

use crate::camera::Camera;
use crate::ray::ray_color;
use crate::rt_weekend::random_double;
use crate::scene::*;
use crate::vec3::Vec3;
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, process::exit};
use vec3::{Color, Point3};

fn main() {
    let path = std::path::Path::new("output/book3/image3.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    //Image
    let mut aspect_ratio = 16.0 / 9.0;
    let mut width = 400;
    let mut samples_per_pixel: u32 = 100;
    let max_bounce_depth: i32 = 50;

    //World & Camera
    let world;
    let lookfrom;
    let lookat;
    let mut aperture = 0.0;
    let vfov;
    //let mut vfov = 40.0;
    let mut background = Color::new(0.7, 0.8, 1.0);
    let choice = 6;

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
        3 => {
            world = two_perlin_spheres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        4 => {
            world = earth();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        5 => {
            world = simple_light();
            samples_per_pixel = 400;
            background = Color::default();
            lookfrom = Point3::new(26.0, 3.0, 6.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        6 => {
            world = cornell_box();
            aspect_ratio = 1.0;
            width = 600;
            samples_per_pixel = 200;
            background = Color::default();
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            world = cornell_smoke();
            aspect_ratio = 1.0;
            width = 600;
            samples_per_pixel = 200;
            background = Color::default();
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _other => {
            world = final_scene();
            aspect_ratio = 1.0;
            width = 800;
            samples_per_pixel = 2500;
            background = Color::default();
            lookfrom = Point3::new(478.0, 278.0, -600.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
    }

    let height = (width as f64 / aspect_ratio) as u32;
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
                pixel_color += ray_color(&r, &background, &world, max_bounce_depth);
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
