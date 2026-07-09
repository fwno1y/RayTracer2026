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

use crate::material::{Dielectric, Lambertian, Metal};
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
    let material_left = Rc::new(Dielectric {
        refractive_index: 1.00 / 1.33,
    });
    let material_right = Rc::new(Metal {
        albedo: Color::new_vec3(0.8, 0.6, 0.2),
        fuzz: 1.0,
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
    let camera = Camera::initialize(aspect_ratio, image_width, samples_per_pixel, max_depth);
    let img: RgbImage = camera.render(&world);

    let path = std::path::Path::new("output/book1/image17.png");
    std::fs::create_dir_all(path.parent().unwrap())?;
    img.save(path)?;

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    Ok(())
}
