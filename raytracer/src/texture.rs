use crate::perlin::Perlin;
use crate::vec3::{Color, Point3};
use std::rc::Rc;

pub trait Texture {
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
pub struct CheckerTexture {
    pub even: Option<Rc<dyn Texture>>,
    pub odd: Option<Rc<dyn Texture>>,
}

impl CheckerTexture {
    pub fn new(color1: &Color, color2: &Color) -> Self {
        Self {
            even: Some(Rc::new(SolidColor::new(color1))),
            odd: Some(Rc::new(SolidColor::new(color2))),
        }
    }

    pub fn new_from_opt(even: &Option<Rc<dyn Texture>>, odd: &Option<Rc<dyn Texture>>) -> Self {
        Self {
            even: even.clone(),
            odd: odd.clone(),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.as_ref().unwrap().value(u, v, p)
        } else {
            self.even.as_ref().unwrap().value(u, v, p)
        }
    }
}

#[derive(Clone, Default)]
pub struct NoiseTexture {
    pub noise: Perlin,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}
