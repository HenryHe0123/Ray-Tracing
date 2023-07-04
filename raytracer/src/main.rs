pub mod vec3;
pub mod ray;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, process::exit};
use vec3::{Vec3, Point3, Color};
use crate::ray::{Ray, ray_color};

fn main() {
    let path = std::path::Path::new("output/book1/image2.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    //Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as u32;

    //Camera
    let viewport_h = 2.0;
    let viewport_w = aspect_ratio * viewport_h;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_w, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_h, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    for j in (0..height).rev() {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, j);
            let u = (i as f64) / ((width - 1) as f64);
            let v = (j as f64) / ((height - 1) as f64);
            let dir = lower_left_corner + u * horizontal + v * vertical - origin;
            let r = Ray::new(&origin, &dir);
            let pixel_color: Color = ray_color(&r);
            *pixel = image::Rgb(pixel_color.rgb());
        }
        progress.inc(1);
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
