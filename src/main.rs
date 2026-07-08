mod ray;
mod vec3;
mod vec3color;
mod hittable;
mod sphere;

use crate::vec3::{Point3, Vec3, dot, unit_vector};
use ray::Ray;
use vec3color::Color;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = center - r.origin();
    let a = dot(r.direction(), r.direction());
    let b = -2.0 * dot(r.direction(), oc);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}
fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Point3::new_vec3(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = unit_vector(r.at(t) - Vec3::new_vec3(0.0, 0.0, -1.0));
        return 0.5 * Color::new_vec3(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Vec3::new_vec3(1.0, 1.0, 1.0) + a * Vec3::new_vec3(0.5, 0.7, 1.0)
}

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

fn main() {
    let path = std::path::Path::new("output/book1/image4.png");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3::new_vec3(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new_vec3(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new_vec3(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center
        - Vec3::new_vec3(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let width = image_width as u32;
    let height = image_height as u32;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    for j in 0..height {
        for i in 0..width {
            // 计算当前像素中心
            let pixel_center =
                pixel00_loc + pixel_delta_u * (i as f64) + pixel_delta_v * (j as f64);
            // 射线方向
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new_ray(camera_center, ray_direction);
            // 计算颜色
            let pixel_color = ray_color(&r);
            // 将 [0,1] 映射到 [0,255] 的 u8
            let rbyte = (255.999 * pixel_color.x()) as u8;
            let gbyte = (255.999 * pixel_color.y()) as u8;
            let bbyte = (255.999 * pixel_color.z()) as u8;
            // 写入图像缓冲区（注意 image 坐标 (x, y) 从左上角开始，与 C++ 一致）
            *img.get_pixel_mut(i, j) = image::Rgb([rbyte, gbyte, bbyte]);
        }
        progress.inc(width as u64);
    }
    progress.finish();

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    img.save(path).expect("Cannot save the image to the file");
}
