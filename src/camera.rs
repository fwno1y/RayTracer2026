use image::RgbImage;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::INFINITY;
use crate::vec3::{unit_vector, Point3, Vec3};
use crate::vec3color::Color;

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
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

        for j in 0..height {
            for i in 0..width {
                let pixel_center = self.pixel00_loc
                    + self.pixel_delta_u * (i as f64)
                    + self.pixel_delta_v * (j as f64);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new_ray(self.center, ray_direction);

                let pixel_color = Self::ray_color(&r, world);

                let rbyte = (255.999 * pixel_color.x()) as u8;
                let gbyte = (255.999 * pixel_color.y()) as u8;
                let bbyte = (255.999 * pixel_color.z()) as u8;

                *img.get_pixel_mut(i, j) = image::Rgb([rbyte, gbyte, bbyte]);
            }
        }
        img
    }
    fn initialize(aspect_ratio: f64, image_width: u32) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

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
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
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