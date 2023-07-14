use crate::hittable::{HitRecord, Hittable};
use crate::pdf::{CosPDF, PDF};
use crate::vec3::{Color, Point3, Vec3};
use std::f64::INFINITY;

#[derive(Debug, Copy, Clone, Default)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub tm: f64,
}

impl Ray {
    pub fn new(orig: &Point3, dir: &Vec3, time: f64) -> Self {
        Ray {
            orig: *orig,
            dir: *dir,
            tm: time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }
}

pub fn ray_color(r: &Ray, background: &Color, world: &impl Hittable, depth: i32) -> Color {
    if depth <= 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        return Color::default();
    }
    let mut rec = HitRecord::default();
    if !world.hit(r, 0.001, INFINITY, &mut rec) {
        return *background; // If the ray hits nothing, return the background color.
    }

    let mut scattered = Ray::default();
    let mut albedo = Color::default();
    let emitted = rec
        .mat_ptr
        .as_ref()
        .unwrap()
        .emitted(r, &rec, rec.u, rec.v, &rec.p);
    let mut pdf_val = 1.0;
    if !rec
        .mat_ptr
        .as_ref()
        .unwrap()
        .scatter(r, &rec, &mut albedo, &mut scattered, &mut pdf_val)
    {
        return emitted;
    }

    let p = CosPDF::new(&rec.normal);
    scattered = Ray::new(&rec.p, &p.generate(), r.time());
    pdf_val = p.value(&scattered.direction());

    // hard code
    // let on_light = Point3::new(
    //     random_double_range(230., 343.),
    //     554.,
    //     random_double_range(227., 332.),
    // );
    // let mut to_light = on_light - rec.p;
    // let distance_squared = to_light.length_squared();
    // to_light = to_light.unit();
    //
    // if dot(&to_light, &rec.normal) < 0.0 {
    //     return emitted;
    // }
    //
    // let light_area = ((343 - 213) * (332 - 227)) as f64;
    // let light_cosine = to_light.y().abs();
    // if light_cosine < 0.000001 {
    //     return emitted;
    // }
    //
    // pdf = distance_squared / (light_cosine * light_area);
    // scattered = Ray::new(&rec.p, &to_light, r.time());
    //

    emitted
        + albedo
            * rec
                .mat_ptr
                .as_ref()
                .unwrap()
                .scattering_pdf(r, &rec, &scattered)
            * ray_color(&scattered, background, world, depth - 1)
            / pdf_val
}
