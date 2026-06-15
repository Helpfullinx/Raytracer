use crate::hittable::{HitRecord, Hittable};
use crate::math::interval::Interval;
use crate::math::ray::Ray;

#[derive(Default)]
pub struct HittableList
{
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, interval: Interval, mut rec: &mut HitRecord) -> bool {
        let mut temp_record: HitRecord = Default::default();
        let mut hit_anything = false;
        let mut closest_so_far = interval.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(interval.min, closest_so_far), &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *rec = temp_record.clone();
            }
        }

        hit_anything
    }
}