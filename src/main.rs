mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
mod vec3color;
mod interval;
mod camera;

use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::rtweekend::INFINITY;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3, unit_vector};
use ray::Ray;
use vec3color::Color;
use camera::Camera;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use crate::interval::Interval;

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
    let camera = Camera::initialize(aspect_ratio, image_width, samples_per_pixel);
    let img: RgbImage = camera.render(&world);

    let path = std::path::Path::new("output/book1/image5.png");
    std::fs::create_dir_all(path.parent().unwrap())?;
    img.save(path)?;

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    Ok(())

}
