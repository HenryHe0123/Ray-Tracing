pub mod camera;
pub mod hittable;
pub mod material;
pub mod obj_loader;
pub mod pdf;
pub mod scene;
pub mod texture;
pub mod utility;

use crate::hittable::*;
use crate::material::*;
use crate::pdf::{HittablePDF, MixturePDF, PDF};
use crate::scene::my_scene::*;
use crate::utility::random_double;
use crate::utility::ray::Ray;
use crate::utility::vec3::*;
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;
use std::f64::INFINITY;
use std::sync::{mpsc, Arc};
use std::{fs::File, process::exit, thread};

const MAX_LEN: usize = 4000;
const TIME0: f64 = 0.0;
const TIME1: f64 = 1.0;

fn main() {
    let path = std::path::Path::new("output/works/final-work-edge-detect.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let threads_number: usize = 10;
    let shuffle: bool = true;

    //Image
    let aspect_ratio = 16.0 / 9.0;
    let width: usize = 3840;
    let samples_per_pixel: u32 = 100;
    let max_bounce_depth: i32 = 50;
    let height = (width as f64 / aspect_ratio) as usize;

    let edge_detect: bool = true;
    let edge_detect_level = 72.0; //high: 64, low: 128

    //World
    let (world, camera) = final_work();
    let background = Color::new(0.5, 0.7, 1.0) * 0.8;

    //Lights
    let lights = HittableList::default();

    //Render
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);

    //for edge detection
    let mut rgb_table = vec![[[0u8; 3]; MAX_LEN]; MAX_LEN];
    let mut gray_table = vec![[0u8; MAX_LEN]; MAX_LEN];

    //Multi Threads
    let multi_progress_bar = MultiProgress::new();
    let (pixel_list, pixels_per_thread) = pixel_allocate(width, height, threads_number, shuffle);
    let mut threads = Vec::new();
    let mut recv = Vec::new();

    let world = Arc::new(world);
    let lights = Arc::new(lights);

    for (_k, pixels) in pixel_list.iter().enumerate() {
        let (tx, rx) = mpsc::channel();
        recv.push(rx);
        let mut pixel_color_list = Vec::new(); //for channel sending
        let world = world.clone();
        let camera = camera;
        let pixels = pixels.clone();
        //let lights = Arc::new(lights.clone()) as Arc<dyn Hittable>;
        let lights = lights.clone();
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
                    let r = camera.get_ray(u, v, TIME0, TIME1);
                    pixel_color += ray_color(
                        &r,
                        &background,
                        world.as_ref(),
                        lights.as_ref(),
                        max_bounce_depth,
                    );
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

    for thread in threads {
        thread.join().unwrap();
    }

    for receiver in &recv {
        let pixel_color_list = receiver.recv().unwrap();
        for ((i, j), pixel_color) in pixel_color_list {
            if edge_detect {
                rgb_table[i][j] = pixel_color.multi_samples_rgb(samples_per_pixel);
                gray_table[i][j] = gray_color(&rgb_table[i][j]);
            } else {
                let pixel = img.get_pixel_mut(i as u32, j as u32);
                *pixel = image::Rgb(pixel_color.multi_samples_rgb(samples_per_pixel));
            }
        }
    }

    if edge_detect {
        let wx: [i32; 9] = [-1, 0, 1, -2, -0, 2, -1, 0, 1];
        let wy: [i32; 9] = [-1, -2, -1, 0, 0, 0, 1, 2, 1];

        for j in 1..height - 1 {
            for i in 1..width - 1 {
                let rgb = vec![
                    gray_table[i - 1][j - 1],
                    gray_table[i - 1][j],
                    gray_table[i - 1][j + 1],
                    gray_table[i][j - 1],
                    gray_table[i][j],
                    gray_table[i][j + 1],
                    gray_table[i + 1][j - 1],
                    gray_table[i + 1][j],
                    gray_table[i + 1][j + 1],
                ];
                let mut gx = 0;
                let mut gy = 0;
                for k in 0..9 {
                    gx += rgb[k] as i32 * wx[k];
                    gy += rgb[k] as i32 * wy[k];
                }
                let g = ((gx * gx + gy * gy) as f64).sqrt();
                let pixel = img.get_pixel_mut(i as u32, j as u32);
                *pixel = image::Rgb(if g > edge_detect_level {
                    [0, 0, 0]
                } else {
                    rgb_table[i][j]
                });
            }
        }
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

fn ray_color(
    r: &Ray,
    background: &Color,
    world: &impl Hittable,
    lights: &impl Hittable,
    depth: i32,
) -> Color {
    if depth <= 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        return Color::default();
    }
    let rec_op = world.hit(r, 0.001, INFINITY);
    if rec_op.is_none() {
        // If the ray hits nothing, return the background color.
        return *background;
    }
    let rec = rec_op.unwrap();

    let mut srec = ScatterRecord::default();
    let emitted = rec.mat_ptr.emitted(r, &rec, rec.u, rec.v, &rec.p);
    if !rec.mat_ptr.scatter(r, &rec, &mut srec) {
        return emitted;
    }

    if srec.is_specular {
        return srec.attenuation
            * ray_color(&srec.specular_ray, background, world, lights, depth - 1);
    }

    let light_pdf = HittablePDF::new(lights, &rec.p);
    let cos_pdf_box = srec.pdf_ptr.unwrap();
    let cos_pdf_ptr = cos_pdf_box.as_ref();
    let mixed_pdf = MixturePDF::new(&light_pdf, cos_pdf_ptr);

    let pdf_ptr = if lights.empty() {
        cos_pdf_ptr as &dyn PDF
    } else {
        &mixed_pdf as &dyn PDF
    };

    let scattered = Ray::new(&rec.p, &pdf_ptr.generate(), r.time());
    let pdf_val = pdf_ptr.value(&scattered.direction());

    emitted
        + srec.attenuation
            * rec.mat_ptr.scattering_pdf(r, &rec, &scattered)
            * ray_color(&scattered, background, world, lights, depth - 1)
            / pdf_val
}

fn pixel_allocate(
    w: usize,
    h: usize,
    threads_num: usize,
    shuffle: bool,
) -> (Vec<Vec<(usize, usize)>>, u64) {
    let mut pixels_per_thread = (w * h) / threads_num;
    if (w * h) % threads_num > 0 {
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

fn gray_color(rgb: &[u8; 3]) -> u8 {
    rgb[0].max(rgb[1].max(rgb[2]))
}
