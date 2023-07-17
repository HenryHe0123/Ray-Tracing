use crate::hittable::Hittable;
use crate::material::ScatterRecord;
use crate::pdf::{HittablePDF, MixturePDF, PDF};
use crate::utility::vec3::*;
use std::f64::INFINITY;
use std::sync::Arc;

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

pub fn ray_color(
    r: &Ray,
    background: &Color,
    world: &impl Hittable,
    lights: &Arc<dyn Hittable>,
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
    let emitted = rec
        .mat_ptr
        .as_ref()
        .unwrap()
        .emitted(r, &rec, rec.u, rec.v, &rec.p);
    if !rec.mat_ptr.as_ref().unwrap().scatter(r, &rec, &mut srec) {
        return emitted;
    }

    if srec.is_specular {
        return srec.attenuation
            * ray_color(&srec.specular_ray, background, world, lights, depth - 1);
    }

    let light_pdf = HittablePDF::new(lights.clone(), &rec.p);
    let cos_pdf_ptr = srec.pdf_ptr.clone().unwrap();
    let mixed_pdf = MixturePDF::new(Arc::new(light_pdf), cos_pdf_ptr);

    let scattered = Ray::new(&rec.p, &mixed_pdf.generate(), r.time());
    let pdf_val = mixed_pdf.value(&scattered.direction());

    emitted
        + srec.attenuation
            * rec
                .mat_ptr
                .as_ref()
                .unwrap()
                .scattering_pdf(r, &rec, &scattered)
            * ray_color(&scattered, background, world, lights, depth - 1)
            / pdf_val
}
