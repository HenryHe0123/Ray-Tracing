pub mod aabb;
pub mod aarect;
pub mod bvh;
pub mod camera;
pub mod constant_medium;
pub mod hittable;
pub mod material;
pub mod mybox;
pub mod onb;
pub mod pdf;
pub mod perlin;
pub mod ray;
pub mod rt_weekend;
pub mod scene;
pub mod sphere;
pub mod texture;
pub mod vec3;

use crate::aarect::XZRect;
use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::material::EmptyMaterial;
use crate::ray::ray_color;
use crate::rt_weekend::random_double;
use crate::scene::*;
use crate::vec3::Vec3;
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{MultiProgress, ProgressBar};
use rand::seq::SliceRandom;
use std::sync::{mpsc, Arc};
use std::{fs::File, process::exit, thread};
use vec3::{Color, Point3};

fn main() {
    let path = std::path::Path::new("output/book3/image8.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    //Image
    let aspect_ratio = 1.0;
    let width = 600;
    let samples_per_pixel: u32 = 1000;
    let max_bounce_depth: i32 = 50;

    //World & Camera
    let world = cornell_box();
    let light = XZRect::new(
        213.,
        343.,
        227.,
        332.,
        554.,
        Arc::new(EmptyMaterial::default()),
    );

    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let aperture = 0.0;
    let vfov = 40.0;
    let background = Color::default();

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

    //Multi Threads
    let threads_number: usize = 10;
    let shuffle: bool = true;

    let multi_progress = MultiProgress::new();
    let (pixel_list, pixels_per_thread) = pixel_allocate(width, height, threads_number, shuffle);
    let mut threads = Vec::new();
    let mut recv = Vec::new();

    let world = Arc::new(world);

    for (k, pixels) in pixel_list.iter().enumerate() {
        let (tx, rx) = mpsc::channel();
        recv.push(rx);
        //let world = world.clone();
        let world = Arc::clone(&world);
        let camera = camera;
        let pixels = pixels.clone();
        let light = light.clone();
        let lights = Arc::new(light) as Arc<dyn Hittable>;
        let pb = multi_progress.add(ProgressBar::new(pixels_per_thread));
        pb.set_prefix(format!("Process {}", k));
        let handle = thread::spawn(move || {
            for pixel in pixels {
                let mut pixel_color = Color::default();
                for _s in 0..samples_per_pixel {
                    let u = ((pixel.0 as f64) + random_double()) / ((width - 1) as f64);
                    let v =
                        (((height - pixel.1 - 1) as f64) + random_double()) / ((height - 1) as f64);
                    let r = camera.get_ray(u, v, 0.0, 1.0);
                    pixel_color +=
                        ray_color(&r, &background, world.as_ref(), &lights, max_bounce_depth);
                }
                tx.send((pixel, pixel_color)).unwrap();
                pb.inc(1);
            }
            pb.finish();
        });
        threads.push(handle);
    }

    if option_env!("CI").unwrap_or_default() != "true" {
        multi_progress.join_and_clear().unwrap();
    }

    for _k in 0..pixels_per_thread {
        for receiver in &recv {
            if let Ok(((i, j), pixel_color)) = receiver.recv() {
                let pixel = img.get_pixel_mut(i, j);
                *pixel = image::Rgb(pixel_color.multi_samples_rgb(samples_per_pixel));
            } else {
                continue;
            }
        }
    }

    for thread in threads {
        thread.join().unwrap();
    }

    // Single Thread
    //
    // let progress = if option_env!("CI").unwrap_or_default() == "true" {
    //     ProgressBar::hidden()
    // } else {
    //     ProgressBar::new((height * width) as u64)
    // };
    //
    // for j in 0..height {
    //     for i in 0..width {
    //         let pixel = img.get_pixel_mut(i, j);
    //         let mut pixel_color = Color::default();
    //         for _s in 0..samples_per_pixel {
    //             let u = ((i as f64) + random_double()) / ((width - 1) as f64);
    //             let v = (((height - j - 1) as f64) + random_double()) / ((height - 1) as f64);
    //             let r = camera.get_ray(u, v, 0.0, 1.0);
    //             pixel_color += ray_color(&r, &background, &world, max_bounce_depth);
    //         }
    //         *pixel = image::Rgb(pixel_color.multi_samples_rgb(samples_per_pixel));
    //         progress.inc(1);
    //     }
    // }
    //
    // progress.finish();

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

//----------------------------------------------------------------------------------------------

fn pixel_allocate(
    w: u32,
    h: u32,
    threads_num: usize,
    shuffle: bool,
) -> (Vec<Vec<(u32, u32)>>, u64) {
    let mut pixels_per_thread = (w * h) as usize / threads_num;
    if (w * h) as usize % threads_num > 0 {
        pixels_per_thread += 1;
    }

    let mut pixel_set = vec![Vec::new(); threads_num];

    let mut all_pixels = Vec::new();
    for j in 0..h {
        for i in 0..w {
            all_pixels.push((i, j));
        }
    }

    if shuffle {
        all_pixels.shuffle(&mut rand::thread_rng());
    }

    for (index, pixel) in all_pixels.iter().enumerate() {
        let id = index / pixels_per_thread;
        pixel_set[id].push(*pixel);
    }

    (pixel_set, pixels_per_thread as u64)
}
