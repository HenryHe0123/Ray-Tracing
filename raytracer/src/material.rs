use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{dot, reflect, Color, Vec3};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Lambertian { albedo: *a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_dir = rec.normal + Vec3::random_unit_vector();
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }
        *attenuation = self.albedo;
        *scattered = Ray::new(&rec.p, &scatter_dir);
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
        let scatter_dir = reflect(&r_in.direction().unit(), &rec.normal)
            + self.fuzz * Vec3::random_in_unit_sphere();
        *attenuation = self.albedo;
        *scattered = Ray::new(&rec.p, &scatter_dir);
        dot(&scattered.direction(), &rec.normal) > 0.
    }
}
