use std::sync::Arc;
use crate::hittable::{HitRecord, Hittable};
use crate::material::isotropic::Isotropic;
use crate::material::Material;
use crate::math::interval::Interval;
use crate::math::ray::Ray;
use crate::math::utility::random_f64;
use crate::math::vec3::{Color, Point3, Vec3};

pub struct Volume {
    x_region: Interval,
    y_region: Interval,
    z_region: Interval,
    neg_inv_density: f64,
    material: Arc<dyn Material>
}

impl Volume {
    pub fn new(center: Point3, dimensions: Vec3, density: f64) -> Self {
        Self {
            x_region: Interval::new(center.x - dimensions.x / 2.0, center.x + dimensions.x / 2.0),
            y_region: Interval::new(center.y - dimensions.y / 2.0, center.y + dimensions.y / 2.0),
            z_region: Interval::new(center.z - dimensions.z / 2.0, center.z + dimensions.z / 2.0),
            neg_inv_density: -1.0 / density,
            material: Arc::new(Isotropic::new(Color::new(0.8, 0.8, 0.8)))
        }
    }
}

impl Hittable for Volume {
    fn hit(&self, r: &Ray, interval: Interval, rec: &mut HitRecord) -> bool {
        let mut t_min = interval.min;
        let mut t_max = interval.max;

        for (min, max, origin, direction) in [
            (
                self.x_region.min,
                self.x_region.max,
                r.origin().x,
                r.direction().x,
            ),
            (
                self.y_region.min,
                self.y_region.max,
                r.origin().y,
                r.direction().y,
            ),
            (
                self.z_region.min,
                self.z_region.max,
                r.origin().z,
                r.direction().z,
            ),
        ] {
            let inv_d = 1.0 / direction;

            let mut t0 = (min - origin) * inv_d;
            let mut t1 = (max - origin) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max <= t_min {
                return false;
            }
        }

        if t_min < 0.0 {
            t_min = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (t_max - t_min) * ray_length;

        // random_f64() should return (0,1]
        let hit_distance = self.neg_inv_density * random_f64().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = t_min + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        // Medium hits don't have a meaningful surface normal.
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.material = self.material.clone();

        true
    }
}