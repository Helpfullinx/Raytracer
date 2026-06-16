use crate::hittable::HitRecord;
use crate::material::Material;
use crate::math::ray::Ray;
use crate::math::utility::random_f64;
use crate::math::vec3::{dot, reflect, refract, unit_vector, Color};

pub struct Dielectric {
    refractive_index: f64,
    albedo: Color
}

impl Dielectric {
    pub fn new(refractive_index: f64, albedo: Color) -> Self {
        Self {
            refractive_index,
            albedo
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = self.albedo;
        let ri = if rec.front_face { 1.0 / self.refractive_index } else { self.refractive_index };

        let unit_direction = unit_vector(&ray_in.direction());
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, ri) > random_f64() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, ri)
        };

        *scattered = Ray::new(rec.p, direction, ray_in.time());
        true
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 * r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}