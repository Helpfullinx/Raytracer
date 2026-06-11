use std::fs::File;
use std::io;
use std::io::{BufWriter, Error, Write};
use std::path::Path;
use png::Encoder;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utility::random_float_range;
use crate::vec3::*;

#[derive(Default)]
pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_data: Vec<u8>,
    sample_per_pixel: u32,
    pixel_samples_scale: f64,
    max_depth: u32
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, spp: u32, max_depth: u32) -> Camera {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        image_height = if image_height < 1 { 1 } else { image_height };

        let center = Point3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

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
            sample_per_pixel: spp,
            pixel_samples_scale: 1.0 / spp as f64,
            max_depth,
            ..Default::default()
        }
    }

    pub fn render(self: &mut Self, world: &dyn Hittable) {
        println!("----------------------Starting Render----------------------");

        for y in 0..self.image_height {
            print!("\rProgress: {:?}%", ((y as f64 / self.image_height as f64) * 100.0) as u32);
            io::stdout().flush().unwrap();

            for x in 0..self.image_width {
                let mut pixel_color = Color::new(0.0,0.0,0.0);
                
                for _ in 0..self.sample_per_pixel{
                    let r = self.get_ray(x,y);
                    pixel_color = pixel_color + ray_color(&r, self.max_depth, world);
                }
                
                let mut con_color = convert_color(pixel_color * self.pixel_samples_scale);
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

    fn get_ray(self: &Self, x: u32, y: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (x as f64 + offset.x))
            + (self.pixel_delta_v* (y as f64 + offset.y));
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        
        Ray::new(ray_origin, ray_direction)
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(
        random_float_range(0.0, 1.0) - 0.5,
        random_float_range(0.0, 1.0) - 0.5,
        0.0)
}

fn ray_color(ray: &Ray, depth: u32, world: &dyn Hittable) -> Color {
    if depth == 0 { return Color::new(0.0,0.0,0.0) }
    
    let mut rec: HitRecord = HitRecord::default();

    //Objects
    if world.hit(ray, Interval::new (0.001, f64::INFINITY), &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec.material.scatter(ray, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, depth-1, world);
        }
        return Color::default();
    }

    // Sky
    let unit_direction = unit_vector(&ray.direction());
    let a = (unit_direction.y + 1.0) * 0.5;
    (Color::new(0.5, 0.7, 1.0) * a) + Color::new(1.0,1.0,1.0) * (1.0-a)
}