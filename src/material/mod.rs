use crate::hittable::HitRecord;
use crate::math::ray::Ray;
use crate::math::vec3::{Color, Point3};

pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;

pub trait Material: Sync + Send {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord, _attenuation: &mut Color, _scattered: &mut Ray) -> bool { false }
    fn emitted(&self) -> Color { Color::new(0.0,0.0,0.0) }
}