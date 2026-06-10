use std::fs::File;
use std::io;
use std::io::{BufWriter, Error, Write};
use std::path::Path;
use png::Encoder;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::*;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_data: Vec<u8>
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Camera {
        let mut image_height = (image_width as f32 / aspect_ratio) as u32;
        image_height = if image_height < 1 { 1 } else { image_height };

        let center = Point3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left = center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            ..Default::default()
        }
    }

    pub fn render(self: &mut Self, world: &dyn Hittable) {
        println!("----------------------Starting Render----------------------");

        for y in 0..self.image_height {
            print!("\rProgress: {:?}%", ((y as f32 / self.image_height as f32) * 100.0) as u32);
            io::stdout().flush().unwrap();

            for x in 0..self.image_width {
                let pixel_center = self.pixel00_loc + (self.pixel_delta_u * x as f32) + (self.pixel_delta_v * y as f32);
                let ray_direction = pixel_center - self.center;
                let r= Ray::new(self.center, ray_direction);

                let pixel_color = ray_color(&r, world);
                let mut con_color = Vec3::convert_color(pixel_color);
                self.pixel_data.append(&mut con_color);
            }
        }

        match self.create_image() {
            Ok(_) => {println!("\n----------------------Render Finished----------------------")}
            Err(e) => {println!("\n----------------------Render Failed----------------------\n{:?}", e)}
        }
    }

    fn create_image(self: &Self) -> Result<(),Error> {
        let path = Path::new(r"./image.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = Encoder::new(w, self.image_width, self.image_height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
        let source_chromaticities = png::SourceChromaticities::new(
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000)
        );
        encoder.set_source_chromaticities(source_chromaticities);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.pixel_data)?;

        Ok(())
    }

    pub fn debug_info(self: &Self) {
        println!("Aspect Ratio: {:?}", self.aspect_ratio);
        println!("Image Dimenstions: {:?} x {:?}", self.image_width, self.image_height);
        println!()
    }
}

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut rec: HitRecord = HitRecord::default();

    if world.hit(ray, 0.0, f32::INFINITY, &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5
    }

    let unit_direction = Vec3::unit_vector(&ray.direction());
    let a = (unit_direction.y + 1.0) * 0.5;
    (Color::new(0.5, 0.7, 1.0) * a) + Color::new(1.0,1.0,1.0) * (1.0-a)
}