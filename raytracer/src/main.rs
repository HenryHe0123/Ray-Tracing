pub mod bvh;
pub mod camera;
pub mod hittable;
pub mod material;
pub mod pdf;
pub mod ray;
pub mod scene;
pub mod texture;
pub mod utility;

use crate::camera::Camera;
use crate::hittable::aarect::XZRect;
use crate::hittable::Hittable;
use crate::material::EmptyMaterial;
use crate::ray::ray_color;
use crate::scene::*;
use crate::utility::random_double;
use crate::utility::vec3::*;
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;
use std::sync::{mpsc, Arc};
use std::{fs::File, process::exit, thread};

fn main() {
    let path = std::path::Path::new("output/book3/image22(b2)-6000.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    //Image
    let aspect_ratio = 1.0;
    let width = 800;
    let samples_per_pixel: u32 = 6000;
    let max_bounce_depth: i32 = 50;

    //World
    let world = final_scene();
    let background = Color::default();

    //Lights
    //let mut lights = HittableList::default();
    let lights = XZRect::new(
        123.,
        423.,
        147.,
        412.,
        554.,
        Arc::new(EmptyMaterial::default()),
    );

    //Camera
    let lookfrom = Point3::new(478.0, 278.0, -600.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vfov = 40.0;
    let aperture = 0.0;

    let height = (width as f64 / aspect_ratio) as u32;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let time0 = 0.0;
    let time1 = 1.0;

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

    let multi_progress_bar = MultiProgress::new();
    let (pixel_list, pixels_per_thread) = pixel_allocate(width, height, threads_number, shuffle);
    let mut threads = Vec::new();
    let mut recv = Vec::new();

    let world = Arc::new(world);

    for (_k, pixels) in pixel_list.iter().enumerate() {
        let (tx, rx) = mpsc::channel();
        recv.push(rx);
        let mut pixel_color_list = Vec::new(); //for channel sending
        let world = world.clone();
        let camera = camera;
        let pixels = pixels.clone();
        let lights = Arc::new(lights.clone()) as Arc<dyn Hittable>;
        let pb = multi_progress_bar.add(ProgressBar::new(pixels_per_thread));
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
            .progress_chars("#>-"));
        let handle = thread::spawn(move || {
            for pixel in pixels {
                let mut pixel_color = Color::default();
                for _s in 0..samples_per_pixel {
                    let u = ((pixel.0 as f64) + random_double()) / ((width - 1) as f64);
                    let v =
                        (((height - pixel.1 - 1) as f64) + random_double()) / ((height - 1) as f64);
                    let r = camera.get_ray(u, v, time0, time1);
                    pixel_color +=
                        ray_color(&r, &background, world.as_ref(), &lights, max_bounce_depth);
                }
                pixel_color_list.push((pixel, pixel_color));
                pb.inc(1);
            }
            tx.send(pixel_color_list).unwrap();
            pb.finish();
        });
        threads.push(handle);
    }

    if option_env!("CI").unwrap_or_default() != "true" {
        multi_progress_bar.join_and_clear().unwrap();
    }

    for receiver in &recv {
        let pixel_color_list = receiver.recv().unwrap();
        for ((i, j), pixel_color) in pixel_color_list {
            let pixel = img.get_pixel_mut(i, j);
            *pixel = image::Rgb(pixel_color.multi_samples_rgb(samples_per_pixel));
        }
    }

    for thread in threads {
        thread.join().unwrap();
    }

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
