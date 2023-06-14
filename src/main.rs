mod vec3;
mod ray;
mod hittable;
mod sphere;

use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

fn main() {

    // Image

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i16 = 1920;
    const IMAGE_HEIGHT: i16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i16;

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
            let pixel_color= ray_color(&r);
            Vec3::write_color(pixel_color);
        }
    }
}

fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = Vec3::dot(&oc, &r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    return if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let mut t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = Vec3::unit_vector(&(r.at(t) - Vec3::new(0.0,0.0,-1.0)));
        return Color::new(n.x + 1.0,n.y + 1.0, n.z + 1.0) * 0.5;
    }
    let unit_direction = Vec3::unit_vector(&r.direction());
    t = 0.5 * (unit_direction.y + 1.0);
    return Color::new(1.0,1.0,1.0) * (1.0 - t) + Color::new(0.5,0.7,1.0) * t
}
