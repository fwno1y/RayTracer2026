use image::RgbImage;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::INFINITY;
use crate::rtweekend::random_double;
use crate::vec3::{unit_vector, Point3, Vec3};
use crate::vec3color::Color;

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    samples_per_pixel: u32,
    image_height: u32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(&self, world: &dyn Hittable) -> RgbImage {
        let width = self.image_width;
        let height = self.image_height;
        let mut img = RgbImage::new(width, height);
        let intensity = Interval::new(0.000, 0.999);

        for j in 0..height {
            for i in 0..width {
                let mut color = Color::new_vec3(0.0, 0.0, 0.0);

                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    color += Self::ray_color(&r, world);
                }
                // 平均颜色
                color *= self.pixel_samples_scale;
                // 写入像素
                let rbyte = (256.0 * intensity.clamp(color.x())) as u8;
                let gbyte = (256.0 * intensity.clamp(color.y())) as u8;
                let bbyte = (256.0 * intensity.clamp(color.z())) as u8;
                *img.get_pixel_mut(i, j) = image::Rgb([rbyte, gbyte, bbyte]);
            }
        }
        img
    }
    pub fn initialize(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let pixel_samples_scale = 1.0 / image_height as f64;

        let center = Point3::new_vec3(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new_vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new_vec3(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = center
            - Vec3::new_vec3(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            image_height,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc + ((i as f64 + offset.x()) * self.pixel_delta_u) + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new_ray(ray_origin, ray_direction)
    }
    fn sample_square() -> Vec3 {
        Vec3::new_vec3(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.0, INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new_vec3(1.0, 1.0, 1.0));
        }
        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new_vec3(1.0, 1.0, 1.0) + a * Color::new_vec3(0.5, 0.7, 1.0)
    }
}