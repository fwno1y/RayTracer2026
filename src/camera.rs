use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::ScatterRecord;
use crate::pdf::{HittablePDF, MixturePDF, Pdf};
use crate::ray::Ray;
use crate::rtweekend::random_double;
use crate::rtweekend::{INFINITY, degrees_to_radians};
use crate::vec3::{Point3, Vec3, cross, random_in_unit_disk, unit_vector};
use crate::vec3color::{Color, linear_to_gemma};
use image::RgbImage;
use rayon::prelude::*;
use std::sync::Arc;

#[allow(dead_code)]
pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    background: Color,
    // albedo: f64, //反射率
    vfov: f64,
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
    image_height: u32,
    pixel_samples_scale: f64,
    sqrt_spp: u32,
    recip_sqrt_spp: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn render(&self, world: &dyn Hittable, lights: Arc<dyn Hittable>) -> RgbImage {
        let width = self.image_width;
        let height = self.image_height;
        let mut img = RgbImage::new(width, height);
        let intensity = Interval::new(0.000, 0.999);
        // 生成所有像素坐标
        let pixels: Vec<(u32, u32)> = (0..height)
            .flat_map(|j| (0..width).map(move |i| (i, j)))
            .collect();

        // 并行计算每个像素的颜色（借用 self 和 world）
        let colors: Vec<Color> = pixels
            .par_iter()
            .map(|&(i, j)| {
                let mut pixel_color = Color::new_vec3(0.0, 0.0, 0.0);
                for s_i in 0..self.sqrt_spp {
                    for s_j in 0..self.sqrt_spp {
                        let r = self.get_ray(i, j, s_i, s_j);
                        pixel_color += self.ray_color(&r, self.max_depth, world, lights.clone());
                    }
                }
                pixel_color * self.pixel_samples_scale
            })
            .collect();

        // 顺序写入图像
        for (&(i, j), &color) in pixels.iter().zip(colors.iter()) {
            let rbyte = (256.0 * intensity.clamp(linear_to_gemma(color.x()))) as u8;
            let gbyte = (256.0 * intensity.clamp(linear_to_gemma(color.y()))) as u8;
            let bbyte = (256.0 * intensity.clamp(linear_to_gemma(color.z()))) as u8;
            *img.get_pixel_mut(i, j) = image::Rgb([rbyte, gbyte, bbyte]);
        }

        img
    }
    #[allow(clippy::too_many_arguments)]
    pub fn initialize(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        background: Color,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let sqrt_spp = (samples_per_pixel as f64).sqrt() as u32;
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
        let recip_sqrt_spp = 1.0 / sqrt_spp as f64;

        let center = lookfrom;

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;
        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            background,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            image_height,
            pixel_samples_scale,
            sqrt_spp,
            recip_sqrt_spp,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
    fn get_ray(&self, i: u32, j: u32, s_i: u32, s_j: u32) -> Ray {
        let offset = self.sample_square_stratified(s_i, s_j);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();
        Ray::new_ray(ray_origin, ray_direction, ray_time)
    }
    #[allow(dead_code)]
    fn sample_square() -> Vec3 {
        Vec3::new_vec3(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }
    #[allow(clippy::only_used_in_recursion)]
    fn ray_color(
        &self,
        r: &Ray,
        depth: u32,
        world: &dyn Hittable,
        lights: Arc<dyn Hittable>,
    ) -> Color {
        if depth == 0 {
            return Color::new_vec3(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::default();
        if !world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            return self.background;
        }
        let mut srec = ScatterRecord::default();
        let mat = rec.mat.as_ref().unwrap();
        let color_from_emission = mat.emitted(r, &rec, rec.u, rec.v, &rec.p);
        if !mat.scatter(r, &rec, &mut srec) {
            return color_from_emission;
        }
        if srec.skip_pdf {
            return srec.attenuation * self.ray_color(&srec.skip_pdf_ray, depth - 1, world, lights.clone());
        }
        let light_ptr = Arc::new(HittablePDF::new(lights.clone(), rec.p));
        let p = MixturePDF::new(light_ptr, srec.pdf_ptr.clone());
        let scattered = Ray::new_ray(rec.p, p.generate(), r.time());
        let pdf_value = p.value(scattered.direction());
        let scattering_pdf = mat.scattering_pdf(r, &rec, &scattered);
        let sample_color = self.ray_color(&scattered, depth - 1, world, lights);
        let color_from_scatter = (srec.attenuation * scattering_pdf * sample_color) / pdf_value;
        color_from_emission + color_from_scatter
    }
    fn sample_square_stratified(&self, s_i: u32, s_j: u32) -> Vec3 {
        let px = ((s_i as f64 + random_double()) * self.recip_sqrt_spp) - 0.5;
        let py = ((s_j as f64 + random_double()) * self.recip_sqrt_spp) - 0.5;
        Vec3::new_vec3(px, py, 0.0)
    }
}
