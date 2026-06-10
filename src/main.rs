mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utility;
mod camera;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Point3;

fn main() {
    //World
    let mut world: HittableList = Default::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0)));

    let aspect_ratio: f32 = 16.0/9.0;
    let image_width = 1920;

    let mut camera = Camera::new(aspect_ratio, image_width);

    camera.debug_info();

    camera.render(&world)
}
