pub mod perlin;

use crate::texture::perlin::Perlin;
use crate::utility::clamp;
use crate::utility::vec3::*;
use image::GenericImageView;
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color: &Color) -> Self {
        Self {
            color_value: *color,
        }
    }

    pub fn new_from_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}

#[derive(Clone, Default)]
pub struct CheckerTexture<T1: Texture, T2: Texture> {
    pub even: T1,
    pub odd: T2,
}

impl<T1: Texture, T2: Texture> CheckerTexture<T1, T2> {
    pub fn new(even: T1, odd: T2) -> Self {
        Self { even, odd }
    }
}

impl CheckerTexture<SolidColor, SolidColor> {
    pub fn new_from_color(color1: &Color, color2: &Color) -> Self {
        Self {
            even: SolidColor::new(color1),
            odd: SolidColor::new(color2),
        }
    }
}

impl<T1: Texture, T2: Texture> Texture for CheckerTexture<T1, T2> {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Clone, Default)]
pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1., 1., 1.)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turbulence(p)).sin())
    }
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        NoiseTexture {
            noise: Perlin::default(),
            scale,
        }
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    data: Arc<Vec<u8>>,
    width: u32,
    height: u32,
    bytes_per_scanline: u32,
}

impl ImageTexture {
    pub const BYTES_PER_PIXEL: u32 = 3;

    pub fn new(pathname: &str) -> Self {
        let img = image::open(pathname).expect("Fail to load image file.");
        let data = img.to_rgb8().into_vec();
        let (width, height) = img.dimensions();
        Self {
            data: Arc::new(data),
            width,
            height,
            bytes_per_scanline: ImageTexture::BYTES_PER_PIXEL * width,
        }
    }

    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for ImageTexture {
    fn default() -> Self {
        Self {
            data: Arc::new(Vec::new()),
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);
        let mut i = (u * self.width as f64) as u32;
        let mut j = (v * self.height as f64) as u32;
        // Clamp integer mapping, since actual coordinates should be less than 1.0
        i = i.min(self.width - 1);
        j = j.min(self.height - 1);
        let color_scale = 1.0 / 255.0;
        let index = (j * self.bytes_per_scanline + i * ImageTexture::BYTES_PER_PIXEL) as usize;
        color_scale
            * Color::new(
                self.data[index] as f64,
                self.data[index + 1] as f64,
                self.data[index + 2] as f64,
            )
    }
}
