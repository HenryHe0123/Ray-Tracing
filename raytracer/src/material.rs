use crate::hittable::HitRecord;
use crate::pdf::CosPDF;
use crate::texture::{SolidColor, Texture};
use crate::utility::random_double;
use crate::utility::ray::Ray;
use crate::utility::vec3::*;
use std::f64::consts::PI;

#[derive(Default)]
pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Color,
    pub pdf_ptr: Option<Box<CosPDF>>,
}

pub trait Material: Send + Sync {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool {
        false
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::default()
    }
}

#[derive(Copy, Clone, Default)]
pub struct EmptyMaterial {}
impl Material for EmptyMaterial {}
pub const DEFAULT_MATERIAL: EmptyMaterial = EmptyMaterial {};

#[derive(Clone, Default)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(a: T) -> Self {
        Lambertian { albedo: a }
    }
}

impl Lambertian<SolidColor> {
    pub fn new_from_color(color: &Color) -> Self {
        Lambertian {
            albedo: SolidColor::new(color),
        }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.is_specular = false;
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Some(Box::new(CosPDF::new(&rec.normal)));
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = dot(&rec.normal, &scattered.direction().unit());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: &Color, f: f64) -> Self {
        Metal {
            albedo: *a,
            fuzz: f.min(1.0),
        }
    }

    pub fn gold() -> Self {
        Self {
            albedo: Color::gold(),
            fuzz: 0.2,
        }
    }

    pub fn silver() -> Self {
        Self {
            albedo: Color::silver(),
            fuzz: 0.2,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let reflected = reflect(&r_in.direction().unit(), &rec.normal)
            + self.fuzz * Vec3::random_in_unit_sphere();
        srec.specular_ray = Ray::new(&rec.p, &reflected, r_in.time());
        srec.attenuation = self.albedo;
        srec.is_specular = true;
        srec.pdf_ptr = None;
        true
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Dielectric {
    pub ir: f64, //index of refraction
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Dielectric {
            ir: index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.is_specular = true;
        srec.pdf_ptr = None;
        srec.attenuation = Color::new(1.0, 1.0, 1.0);

        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_dir = r_in.direction().unit();
        let cos_theta = dot(&(-unit_dir), &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
                reflect(&unit_dir, &rec.normal)
            } else {
                refract(&unit_dir, &rec.normal, refraction_ratio)
            };

        srec.specular_ray = Ray::new(&rec.p, &direction, r_in.time());
        true
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
}

#[derive(Clone, Default)]
pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(a: T) -> Self {
        Self { emit: a }
    }
}

impl DiffuseLight<SolidColor> {
    pub fn new_from_color(color: &Color) -> Self {
        Self {
            emit: SolidColor::new(color),
        }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            Color::default()
        }
    }
}

#[derive(Clone, Default)]
pub struct Isotropic<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(a: T) -> Self {
        Self { albedo: a }
    }
}

impl Isotropic<SolidColor> {
    pub fn new_from_color(color: &Color) -> Self {
        Self {
            albedo: SolidColor::new(color),
        }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.specular_ray = Ray::new(&rec.p, &Vec3::random_in_unit_sphere(), r_in.time());
        srec.is_specular = true;
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = None;
        true
    }
}
