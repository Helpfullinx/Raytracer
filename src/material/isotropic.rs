use crate::hittable::HitRecord;
use crate::material::Material;
use crate::math::ray::Ray;
use crate::math::vec3::{random_unit_vector, Color};

pub struct Isotropic {
    albedo: Color
}

impl Isotropic {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *scattered = Ray::new(rec.p, random_unit_vector(), ray_in.time());
        *attenuation = self.albedo;
        true
    }
}