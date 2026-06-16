use std::fs::File;
use std::io;
use std::io::{BufWriter, Error, Write};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use png::Encoder;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use crate::hittable::{HitRecord, Hittable};
use crate::math::interval::Interval;
use crate::math::ray::Ray;
use crate::math::utility::{degrees_to_radians, random_f64, random_float_range};
use crate::math::vec3::{convert_color, cross, unit_vector, Color, Point3, Vec3};
use crate::postprocessing::denoise_bilateral;

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
    max_depth: u32,
    fov: f32,
    environment: (Color, Color)
}

impl Camera {
    pub fn new(origin: Vec3, look_at: Vec3, fov: f32, aspect_ratio: f64, image_width: u32, spp: u32, max_depth: u32, environment: (Color, Color)) -> Camera {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        image_height = if image_height < 1 { 1 } else { image_height };

        let center = origin;

        let focal_length = (origin - look_at).length();
        let theta = degrees_to_radians(fov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * focal_length as f32;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let w = unit_vector(&(origin - look_at));
        let u = unit_vector(&cross(&Vec3::new(0.0,1.0,0.0), &w));
        let v = cross(&w, &u);

        let viewport_u = u * viewport_width as f64;
        let viewport_v = -v * viewport_height as f64;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = center - (w * focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
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
            fov,
            environment,
            ..Default::default()
        }
    }

    pub fn render(self: &mut Self, world: &dyn Hittable) {
        println!("----------------------Starting Render----------------------");

        let width = self.image_width;
        let height = self.image_height;
        let spp = self.sample_per_pixel;
        let scale = self.pixel_samples_scale;
        let max_depth = self.max_depth;
        let completed = AtomicUsize::new(0);

        let mut pixels: Vec<u8> = (0..height)
            .into_par_iter()
            .flat_map_iter(|y| {
                let mut row = Vec::with_capacity(width as usize * 3);

                for x in 0..width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                    for _ in 0..spp {
                        let r = self.get_ray(x, y);
                        pixel_color += self.ray_color(&r, max_depth, world);
                    }

                    row.extend(convert_color(pixel_color * scale));
                }

                let done = completed.fetch_add(1, Ordering::Relaxed) + 1;

                if done % 10 == 0 {
                    print!("\rProgress: {}%", done * 100 / height as usize);
                    io::stdout().flush().unwrap();
                }

                row
            })
            .collect();

        // pixels = denoise_bilateral(&pixels, width as usize, height as usize);
        self.pixel_data = pixels;

        match self.create_image() {
            Ok(_) => println!("\n----------------------Render Finished----------------------"),
            Err(e) => println!("\n----------------------Render Failed----------------------\n{:?}", e),
        }
    }

    fn create_image(self: &Self) -> Result<(),Error> {
        let path = Path::new(r"../resources/image.png");
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
        let ray_time = random_f64();
        
        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn ray_color(self: &Self, ray: &Ray, depth: u32, world: &dyn Hittable) -> Color {
        if depth == 0 { return Color::new(0.0,0.0,0.0) }

        let mut rec: HitRecord = HitRecord::default();

        //Objects
        if world.hit(ray, Interval::new (0.001, f64::INFINITY), &mut rec) {
            let emitted = rec.material.emitted();
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if rec.material.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                return emitted + attenuation * self.ray_color(&scattered, depth-1, world);
            }

            return emitted;
        }

        // Sky
        let unit_direction = unit_vector(&ray.direction());
        let a = (unit_direction.y + 1.0) * 0.5;
        (self.environment.0 * a) + self.environment.1 * (1.0-a)
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(
        random_float_range(0.0, 1.0) - 0.5,
        random_float_range(0.0, 1.0) - 0.5,
        0.0)
}