use std::rc::Rc;
use crate::math::interval::Interval;
use crate::material::Material;
use crate::material::lambertian::Lambertian;
use crate::math::ray::Ray;
use crate::math::vec3::{dot, Point3, Vec3};
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -(*outward_normal) };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, interval: Interval, rec: &mut HitRecord) -> bool;
}

impl Default for HitRecord {
    fn default() -> Self {
        Self{
            material: Rc::new(Lambertian::default()),
            p: Default::default(),
            normal: Default::default(),
            
            t: 0.0,
            front_face: false,
        }
    }
}