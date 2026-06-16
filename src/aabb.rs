use std::sync::Arc;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::interval::Interval;
use crate::math::ray::Ray;
use crate::math::vec3::{Point3, Vec3};

pub struct AABB {
    material: Arc<dyn Material>,
    x_region: Interval,
    y_region: Interval,
    z_region: Interval,
}

impl AABB {
    pub fn new(center: Point3, dimensions: Vec3, material: Arc<dyn Material>) -> Self {
       Self {
           material,
           x_region: Interval::new(center.x - dimensions.x / 2.0, center.x + dimensions.x / 2.0),
           y_region: Interval::new(center.y - dimensions.y / 2.0, center.y + dimensions.y / 2.0),
           z_region: Interval::new(center.z - dimensions.z / 2.0, center.z + dimensions.z / 2.0),
       }
    }
}

impl Hittable for AABB {
    fn hit(&self, r: &Ray, interval: Interval, rec: &mut HitRecord) -> bool {
        let mut t_min = interval.min;
        let mut t_max = interval.max;

        let mut normal = Vec3::default();

        for (min, max, origin, dir, neg_normal, pos_normal) in [
            (
                self.x_region.min,
                self.x_region.max,
                r.origin().x,
                r.direction().x,
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new( 1.0, 0.0, 0.0),
            ),
            (
                self.y_region.min,
                self.y_region.max,
                r.origin().y,
                r.direction().y,
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(0.0,  1.0, 0.0),
            ),
            (
                self.z_region.min,
                self.z_region.max,
                r.origin().z,
                r.direction().z,
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 0.0,  1.0),
            ),
        ] {
            let inv_d = 1.0 / dir;

            let mut t0 = (min - origin) * inv_d;
            let mut t1 = (max - origin) * inv_d;

            let axis_normal;

            if inv_d >= 0.0 {
                axis_normal = neg_normal;
            } else {
                std::mem::swap(&mut t0, &mut t1);
                axis_normal = pos_normal;
            }

            if t0 > t_min {
                t_min = t0;
                normal = axis_normal;
            }

            t_max = t_max.min(t1);

            if t_max <= t_min {
                return false;
            }
        }

        rec.t = t_min;
        rec.p = r.at(t_min);
        rec.material = self.material.clone();
        rec.set_face_normal(r, &normal);

        true
    }
}