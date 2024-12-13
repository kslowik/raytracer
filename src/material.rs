use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use serde::{Deserialize, Serialize};

serde_with::serde_conv!(
    ColorAsArray,
    Color,
    |color: &Color| [color.x() as f32, color.y() as f32, color.z() as f32],
    |value: [f32; 3]| -> Result<_, std::convert::Infallible> {
        Ok(Color::new(
            value[0] as f64,
            value[1] as f64,
            value[2] as f64,
        ))
    }
);

pub trait Scatterable {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Glass(Glass),
}

impl Scatterable for Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec, attenuation, scattered),
            Material::Metal(m) => m.scatter(r_in, rec, attenuation, scattered),
            Material::Glass(g) => g.scatter(r_in, rec, attenuation, scattered),
        }
    }
}

#[serde_with::serde_as]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Lambertian {
    #[serde_as(as = "ColorAsArray")]
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();

        let scatter_direction = if scatter_direction.near_zero() {
            rec.normal
        } else {
            scatter_direction
        };

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[serde_with::serde_as]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Metal {
    #[serde_as(as = "ColorAsArray")]
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Scatterable for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&r_in.direction().unit_vector(), &rec.normal);
        let scattered_direction = reflected + self.fuzz * Vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, scattered_direction);
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.0
    }
}

#[serde_with::serde_as]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Glass {
    pub refraction_index: f64,
}

impl Glass {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatterable for Glass {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = f64::min((-unit_direction).dot(&rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Glass::reflectance(cos_theta, refraction_ratio) > rand::random::<f64>()
        {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction);
        true
    }
}
