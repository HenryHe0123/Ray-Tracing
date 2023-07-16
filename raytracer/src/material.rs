use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::onb::ONB;
use crate::ray::Ray;
use crate::rt_weekend::random_double;
use crate::texture::{SolidColor, Texture};
use crate::vec3::{dot, reflect, refract, Color, Point3, Vec3};
use std::f64::consts::PI;
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        false
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0
    }

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::default()
    }
}

#[derive(Copy, Clone, Default)]
pub struct EmptyMaterial {}
impl Material for EmptyMaterial {}

#[derive(Clone, Default)]
pub struct Lambertian {
    pub albedo: Option<Arc<dyn Texture + Send + Sync>>,
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        //new a solid color
        Lambertian {
            albedo: Some(Arc::new(SolidColor::new(a))),
        }
    }

    pub fn new_from_ptr(a: &Arc<dyn Texture + Send + Sync>) -> Self {
        Lambertian {
            albedo: Some(a.clone()),
        }
    }

    pub fn new_from_opt(a: &Option<Arc<dyn Texture + Send + Sync>>) -> Self {
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
        pdf: &mut f64,
    ) -> bool {
        // let mut scatter_dir = rec.normal + Vec3::random_unit_vector();
        // if scatter_dir.near_zero() {
        //     scatter_dir = rec.normal; // Catch degenerate scatter direction
        // }
        // *scattered = Ray::new(&rec.p, &scatter_dir.unit(), r_in.time());
        // *attenuation = self.albedo.as_ref().unwrap().value(rec.u, rec.v, &rec.p);
        // *pdf = dot(&rec.normal, &scattered.direction()) / PI;
        //
        let uvw = ONB::build_from_w(&rec.normal);
        let direction = uvw.local_vec(&Vec3::random_cosine_direction());
        *scattered = Ray::new(&rec.p, &direction.unit(), r_in.time());
        *attenuation = self.albedo.as_ref().unwrap().value(rec.u, rec.v, &rec.p);
        *pdf = dot(&uvw.w(), &scattered.direction()) / PI;
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
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _pdf: &mut f64,
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
        _pdf: &mut f64,
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
    emit: Option<Arc<dyn Texture + Send + Sync>>,
}

impl DiffuseLight {
    pub fn new(c: &Color) -> Self {
        Self {
            emit: Some(Arc::new(SolidColor::new(c))),
        }
    }

    pub fn new_from_ptr(a: &Arc<dyn Texture + Send + Sync>) -> Self {
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
        _pdf: &mut f64,
    ) -> bool {
        false
    }

    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if rec.front_face {
            self.emit.as_ref().unwrap().value(u, v, p)
        } else {
            Color::default()
        }
    }
}

#[derive(Clone, Default)]
pub struct Isotropic {
    pub albedo: Option<Arc<dyn Texture + Send + Sync>>,
}

impl Isotropic {
    pub fn new(c: &Color) -> Self {
        Self {
            albedo: Some(Arc::new(SolidColor::new(c))),
        }
    }

    pub fn new_from_ptr(a: &Arc<dyn Texture + Send + Sync>) -> Self {
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
        _pdf: &mut f64,
    ) -> bool {
        *scattered = Ray::new(&rec.p, &Vec3::random_in_unit_sphere(), r_in.time());
        *attenuation = self.albedo.as_ref().unwrap().value(rec.u, rec.v, &rec.p);
        true
    }
}

#[derive(Clone, Default)]
pub struct FlipFace {
    pub ptr: Option<Arc<dyn Hittable + Send + Sync>>,
}

impl FlipFace {
    pub fn new(p_clone: Arc<dyn Hittable + Send + Sync>) -> Self {
        Self { ptr: Some(p_clone) }
    }
}

impl Hittable for FlipFace {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.ptr.as_ref().unwrap().hit(r, t_min, t_max, rec) {
            return false;
        }
        rec.front_face = !rec.front_face;
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.ptr
            .as_ref()
            .unwrap()
            .bounding_box(time0, time1, output_box)
    }
}
