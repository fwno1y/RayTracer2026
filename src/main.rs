mod aabb;
mod bvh;
mod camera;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod ray;
mod rtw_stb_image;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;
mod vec3color;

use crate::hittable_list::HittableList;
use crate::rtweekend::{INFINITY, random_double, random_double_in_range};
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};
use camera::Camera;
use std::rc::Rc;

use crate::bvh::BvhNode;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::vec3color::Color;
use console::style;
use image::RgbImage;

fn bouncing_spheres() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::from_color(
        0.32,
        Color::new_vec3(0.2, 0.3, 0.1),
        Color::new_vec3(0.9, 0.9, 0.9),
    ));
    world.add(Rc::new(Sphere::new(
        Point3::new_vec3(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(checker)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new_vec3(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            if (center - Point3::new_vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Rc::new(Lambertian::from_color(albedo));
                    let center2 =
                        center + Vec3::new_vec3(0.0, random_double_in_range(0.0, 0.5), 0.0);
                    world.add(Rc::new(Sphere::new_move(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = random_double_in_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal { albedo, fuzz });
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Rc::new(Dielectric {
                        refractive_index: 1.5,
                    });
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric {
        refractive_index: 1.50,
    });
    world.add(Rc::new(Sphere::new(
        Point3::new_vec3(0.0, 1.0, 0.0),
        1.0,
        material1.clone(),
    )));
    let material2 = Rc::new(Lambertian::from_color(Color::new_vec3(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new_vec3(-4.0, 1.0, 0.0),
        1.0,
        material2.clone(),
    )));
    let material3 = Rc::new(Metal {
        albedo: Color::new_vec3(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Rc::new(Sphere::new(
        Point3::new_vec3(4.0, 1.0, 0.0),
        1.0,
        material3.clone(),
    )));

    let bvh_root = BvhNode::from_list(world);
    let mut new_world = HittableList::new();
    new_world.add(bvh_root);
    world = new_world;

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 20.0;
    let lookfrom = Point3::new_vec3(13.0, 2.0, 3.0);
    let lookat = Point3::new_vec3(0.0, 0.0, 0.0);
    let vup = Vec3::new_vec3(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;
    let camera = Camera::initialize(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );
    let img: RgbImage = camera.render(&world);

    let path = std::path::Path::new("output/book2/image2.png");
    std::fs::create_dir_all(path.parent().unwrap())?;
    img.save(path)?;

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    Ok(())
}

fn checkered_spheres() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();
    let checker = Rc::new(CheckerTexture::from_color(
        0.32,
        Color::new_vec3(0.2, 0.3, 0.1),
        Color::new_vec3(0.9, 0.9, 0.9),
    ));
    world.add(Rc::new(Sphere::new(
        Point3::new_vec3(0.0, -10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new(checker)),
    )));
    let checker = Rc::new(CheckerTexture::from_color(
        0.32,
        Color::new_vec3(0.2, 0.3, 0.1),
        Color::new_vec3(0.9, 0.9, 0.9),
    ));
    world.add(Rc::new(Sphere::new(
        Point3::new_vec3(0.0, 10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new(checker)),
    )));
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 20.0;
    let lookfrom = Point3::new_vec3(13.0, 2.0, 3.0);
    let lookat = Point3::new_vec3(0.0, 0.0, 0.0);
    let vup = Vec3::new_vec3(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;
    let camera = Camera::initialize(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );
    let img: RgbImage = camera.render(&world);
    let path = std::path::Path::new("output/book2/image3.png");
    std::fs::create_dir_all(path.parent().unwrap())?;
    img.save(path)?;

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    Ok(())
}
fn earth() -> Result<(), Box<dyn std::error::Error>> {
    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Rc::new(Lambertian::new(earth_texture.clone()));
    let globe = Rc::new(Sphere::new(
        Point3::new_vec3(0.0, 0.0, 0.0),
        2.0,
        earth_surface.clone(),
    ));
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 20.0;
    let lookfrom = Point3::new_vec3(0.0, 0.0, 12.0);
    let lookat = Point3::new_vec3(0.0, 0.0, 0.0);
    let vup = Vec3::new_vec3(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let camera = Camera::initialize(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );
    let mut world = HittableList::new();
    world.add(globe);
    let img: RgbImage = camera.render(&world);
    let path = std::path::Path::new("output/book2/image5.png");
    std::fs::create_dir_all(path.parent().unwrap())?;
    img.save(path)?;

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    Ok(())
}
fn perlin_spheres() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();
    let pertext = Rc::new(NoiseTexture::new());
    world.add(Rc::new(Sphere::new(
        Point3::new_vec3(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new_vec3(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(pertext.clone())),
    )));
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 20.0;
    let lookfrom = Point3::new_vec3(13.0, 2.0, 3.0);
    let lookat = Point3::new_vec3(0.0, 0.0, 0.0);
    let vup = Vec3::new_vec3(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let camera = Camera::initialize(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );
    let img: RgbImage = camera.render(&world);
    let path = std::path::Path::new("output/book2/image10.png");
    std::fs::create_dir_all(path.parent().unwrap())?;
    img.save(path)?;

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    Ok(())
}
fn main() {
    match 4 {
        1 => bouncing_spheres().unwrap(),
        2 => checkered_spheres().unwrap(),
        3 => earth().unwrap(),
        4 => perlin_spheres().unwrap(),
        _ => {}
    }
}
