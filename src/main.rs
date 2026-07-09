mod camera;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
mod vec3color;

use crate::hittable_list::HittableList;
use crate::rtweekend::INFINITY;
use crate::sphere::Sphere;
use crate::vec3::Point3;
use camera::Camera;

use console::style;
use image::RgbImage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new_vec3(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new_vec3(0.0, -100.5, -1.0),
        100.0,
    )));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let albedos = vec![0.1, 0.3, 0.5, 0.7, 0.9];
    let n = albedos.len();
    let strip_width = image_width / n as u32;
    let mut images = Vec::new();
    for &albedo in &albedos {
        let camera = Camera::initialize(
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            albedo,
        );
        let img: RgbImage = camera.render(&world);
        images.push(img);
    }

    let height = images[0].height();
    let mut final_img = RgbImage::new(image_width, height);

    for (idx, img) in images.iter().enumerate() {
        let x_start = idx as u32 * strip_width;
        let x_end = x_start + strip_width;
        for y in 0..height {
            for x in x_start..x_end {
                let pixel = img.get_pixel(x, y);
                final_img.put_pixel(x, y, *pixel);
            }
        }
    }

    let path = std::path::Path::new("output/book1/image12.png");
    std::fs::create_dir_all(path.parent().unwrap())?;
    final_img.save(path)?;

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    Ok(())
}
