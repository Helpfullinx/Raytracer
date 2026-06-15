use crate::hittable::HitRecord;
use crate::math::ray::Ray;
use crate::math::vec3::Color;

pub mod lambertian;
pub mod metal;

pub trait Material{
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool { false }
}