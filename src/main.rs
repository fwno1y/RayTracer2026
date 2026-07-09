mod camera;
mod hittable;
mod hittable_list;
mod interval;
mod material;
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
use std::rc::Rc;

use crate::material::{Lambertian, Metal};
use crate::vec3color::Color;
use console::style;
use image::RgbImage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian {
        albedo: Color::new_vec3(0.8, 0.8, 0.0),
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color::new_vec3(0.1, 0.2, 0.5),
    });
    let material_left = Rc::new(Metal {
        albedo: Color::new_vec3(0.8, 0.8, 0.8),
    });
    let material_right = Rc::new(Metal {
        albedo: Color::new_vec3(0.8, 0.6, 0.2),
    });

    world.add(Box::new(Sphere::new(
        Point3::new_vec3(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new_vec3(0.0, 0.0, -1.2),
        0.5,
        material_center.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new_vec3(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new_vec3(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
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

    let path = std::path::Path::new("output/book1/image13.png");
    std::fs::create_dir_all(path.parent().unwrap())?;
    final_img.save(path)?;

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    Ok(())
}
