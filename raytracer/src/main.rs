pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod rt_weekend;
pub mod sphere;
pub mod vec3;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::material::{Lambertian, Metal};
use crate::ray::ray_color;
use crate::rt_weekend::random_double;
use crate::sphere::Sphere;
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::rc::Rc;
use std::{fs::File, process::exit};
use vec3::{Color, Point3};

fn main() {
    let path = std::path::Path::new("output/book1/image12.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    //Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;
    let max_bounce_depth: i32 = 50;

    //World
    let mut world = HittableList::default();
    let material_ground = Rc::new(Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(&Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(&Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(&Color::new(0.8, 0.6, 0.2), 1.0));
    world.add(Rc::new(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    //Camera
    let camera = Camera::default();

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
                let r = camera.get_ray(u, v);
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
