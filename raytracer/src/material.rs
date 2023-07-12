use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rt_weekend::random_double;
use crate::texture::{SolidColor, Texture};
use crate::vec3::{dot, reflect, refract, Color, Point3, Vec3};
use std::rc::Rc;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::default()
    }
}

#[derive(Clone, Default)]
pub struct Lambertian {
    pub albedo: Option<Rc<dyn Texture>>,
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        //new a solid color
        Lambertian {
            albedo: Some(Rc::new(SolidColor::new(a))),
        }
    }

    pub fn new_from_ptr(a: &Rc<dyn Texture>) -> Self {
        Lambertian {
            albedo: Some(a.clone()),
        }
    }

    pub fn new_from_opt(a: &Option<Rc<dyn Texture>>) -> Self {
        Lambertian { albedo: a.clone() }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_dir = rec.normal + Vec3::random_unit_vector();
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }
        *attenuation = self.albedo.as_ref().unwrap().value(rec.u, rec.v, &rec.p);
        *scattered = Ray::new(&rec.p, &scatter_dir, r_in.time());
        true
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
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&r_in.direction().unit(), &rec.normal)
            + self.fuzz * Vec3::random_in_unit_sphere();
        *attenuation = self.albedo;
        *scattered = Ray::new(&rec.p, &reflected, r_in.time());
        dot(&scattered.direction(), &rec.normal) > 0.0
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
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
        *scattered = Ray::new(&rec.p, &direction, r_in.time());
        true
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
}

#[derive(Clone, Default)]
pub struct DiffuseLight {
    emit: Option<Rc<dyn Texture>>,
}

impl DiffuseLight {
    pub fn new(c: &Color) -> Self {
        Self {
            emit: Some(Rc::new(SolidColor::new(c))),
        }
    }

    pub fn new_from_ptr(a: &Rc<dyn Texture>) -> Self {
        Self {
            emit: Some(a.clone()),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.as_ref().unwrap().value(u, v, p)
    }
}

#[derive(Clone, Default)]
pub struct Isotropic {
    pub albedo: Option<Rc<dyn Texture>>,
}

impl Isotropic {
    pub fn new(c: &Color) -> Self {
        Self {
            albedo: Some(Rc::new(SolidColor::new(c))),
        }
    }

    pub fn new_from_ptr(a: &Rc<dyn Texture>) -> Self {
        Self {
            albedo: Some(a.clone()),
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(&rec.p, &Vec3::random_in_unit_sphere(), r_in.time());
        *attenuation = self.albedo.as_ref().unwrap().value(rec.u, rec.v, &rec.p);
        true
    }
}
