use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{reflect, Color};

#[derive(Default, Clone, Copy)]
pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(albedo: Color) -> Self{
        Self{ albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(&ray_in.direction(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}