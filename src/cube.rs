use std::rc::Rc;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::interval::Interval;
use crate::math::ray::Ray;
use crate::math::vec3::{Point3, Vec3};

pub struct Cube {
    center: Point3,
    dimensions: Vec3,
    material: Rc<dyn Material>
}

impl Cube {
    pub fn new(center: Point3, dimensions: Vec3, material: Rc<dyn Material>) -> Self {
       Self {
           center,
           dimensions,
           material
       }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, interval: Interval, rec: &mut HitRecord) -> bool {
    }
}