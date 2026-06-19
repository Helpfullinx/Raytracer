mod hittable;
mod sphere;
mod aabb;
mod hittable_list;
mod camera;
mod material;
mod math;
mod postprocessing;
mod volume;

use std::sync::Arc;
use crate::camera::Camera;
use crate::aabb::AABB;
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::material::dielectric::Dielectric;
use crate::material::diffuse_light::DiffuseLight;
use crate::sphere::Sphere;
use crate::math::vec3::{Color, Point3, Vec3};
use crate::volume::Volume;

#[tokio::main]
async fn main() {
    //World
    let mut world: HittableList = Default::default();

    //Materials
    let grey: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.8,0.8,0.8)));
    let red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(1.0,0.0,0.0)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.0,1.0,0.0)));
    let metal: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.5,0.5,0.5), 0.05));
    let glass: Arc<dyn Material> = Arc::new(Dielectric::new(1.5, Color::new(1.0,1.0,1.0)));
    let emissive: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(1.0,1.0,1.0), 2.0));

    // Objects
    world.add(
        Box::new(
            Volume::new(
                Point3::new(1.5,0.5,-0.5),
                Vec3::new(0.75,2.0,0.75),
                0.9
            )
        )
    );

    world.add(
        Box::new(
            AABB::new(
                Point3::new(-1.0,0.5,-1.5),
                Vec3::new(0.75,2.0,0.75),
                grey.clone()
            )
        )
    );

    world.add(
        Box::new(
            Sphere::new(
                Point3::new(0.0, 0.0,0.0),
                0.5,
                glass.clone()
            )
        )
    );

    world.add(
        Box::new(
            AABB::new(
                Point3::new(0.0, 2.0,0.0),
                Vec3::new(2.0,0.1,2.0),
                emissive.clone()
            )
        )
    );

    world.add(
        Box::new(
            AABB::new(
                Point3::new(-2.0,0.0,0.0),
                Point3::new(0.1,10.0,5.0),
                green.clone()
            )
        )
    );

    world.add(
        Box::new(
            AABB::new(
                Point3::new(0.0,0.0,-2.5),
                Point3::new(10.0,10.0,0.1),
                grey.clone()
            )
        )
    );

    world.add(
        Box::new(
            AABB::new(
                Point3::new(2.0,0.0,0.0),
                Point3::new(0.1,10.0,5.0),
                red.clone()
            )
        )
    );

    world.add(
        Box::new(
            AABB::new(
                Point3::new(0.0,2.1,0.0),
                Point3::new(10.0,0.1,10.0),
                grey.clone()
            )
        )
    );

    world.add(
        Box::new(
            AABB::new(
                Point3::new(0.0,-0.6,0.0),
                Point3::new(10.0,0.1,10.0),
                grey.clone()
            )
        )
    );

    //Camera
    let aspect_ratio: f64 = 1.0;
    let image_width = 256;
    let sky_color = Color::new(0.0,0.0,0.0);
    let ground_color = Color::new(0.0,0.0,0.0);

    let mut camera = Camera::new(
        Vec3::new(0.0,1.0,6.0),
        Vec3::new(0.0,0.5,0.0),
        40.0,
        aspect_ratio,
        image_width,
        600,
        100,
        (sky_color, ground_color)
    );

    camera.debug_info();

    camera.render_gpu(&world).await
}
