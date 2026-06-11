use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;

pub trait Material{
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool { false }
}