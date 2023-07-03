mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utility;

use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3, Vec3};

fn main() {

    // Image

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i16 = 1920;
    const IMAGE_HEIGHT: i16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i16;

    //World
    let mut world: HittableList = Default::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0)));

    // Camera

    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    let focal_length: f32 = 1.0;

    let origin = Point3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    // Render

    println!("P3\n{:?} {:?}\n255",IMAGE_WIDTH,IMAGE_HEIGHT);

    for y in (0..IMAGE_HEIGHT).rev() {
        for x in 0..IMAGE_WIDTH {
            let u = x as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = y as f32 / (IMAGE_HEIGHT - 1) as f32;
            let r = Ray::new(origin, (lower_left_corner + (horizontal * u) + (vertical * v)) - origin);
            let pixel_color= ray_color(&r, &world);
            Vec3::write_color(pixel_color);
        }
    }
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec: HitRecord = Default::default();
    if world.hit(r, 0.0, utility::INFINITY as f32, &mut rec) {
        return (rec.normal + Color::new(1.0,1.0,1.0)) * 0.5
    }
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    return Color::new(1.0,1.0,1.0) * (1.0 - t) + Color::new(0.5,0.7,1.0) * t
}
