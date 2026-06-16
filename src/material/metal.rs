use crate::hittable::HitRecord;
use crate::material::Material;
use crate::math::ray::Ray;
use crate::math::vec3::{dot, random_unit_vector, reflect, unit_vector, Color};

#[derive(Default, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    roughness: f64
}

impl Metal {
    pub fn new(albedo: Color, roughness: f64) -> Self{
        Self{
            albedo,
            roughness: if roughness < 1.0 { roughness } else { 1.0 }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut reflected = reflect(&ray_in.direction(), &rec.normal);
        if self.roughness == 0.0 {
            reflected = unit_vector(&reflected);
        } else {
            reflected = unit_vector(&reflected) + (random_unit_vector() * self.roughness);
        }
        *scattered = Ray::new(rec.p, reflected, ray_in.time());
        *attenuation = self.albedo;

        dot(&scattered.direction(), &rec.normal) > 0.0
    }
}