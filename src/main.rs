mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utility;
mod camera;
mod interval;
mod material;
mod lambertian;
mod metal;

use std::rc::Rc;
use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::lambertian::Lambertian;
use crate::material::Material;
use crate::metal::Metal;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3};

fn main() {
    //World
    let mut world: HittableList = Default::default();

    //Materials
    let lambertian: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.3,0.8,0.0)));
    let metal: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.5,0.5,0.5)));

    //Objects
    world.add(
        Box::new(
            Sphere::new(
                Point3::new(0.0,-100.5,-1.0),
                100.0,
                lambertian.clone()
            )
        )
    );
    world.add(
        Box::new(
            Sphere::new(
                Point3::new(0.0,0.0,-1.2),
                0.5,
                lambertian.clone()
            )
        )
    );
    world.add(
        Box::new(
            Sphere::new(
                Point3::new(-1.0, 0.0,-1.0),
                0.5,
                metal.clone()
            )
        )
    );
    world.add(
        Box::new(
            Sphere::new(
                Point3::new(1.0, 0.0,-1.0),
                0.5,
                metal.clone()
            )
        )
    );


    //Camera
    let aspect_ratio: f64 = 16.0/9.0;
    let image_width = 1920;

    let mut camera = Camera::new(aspect_ratio, image_width, 75, 50);

    camera.debug_info();

    camera.render(&world)
}
