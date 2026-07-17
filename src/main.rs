mod aabb;
mod bvh;
mod camera;
mod constant_medium;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod model;
mod onb;
mod perlin;
mod quad;
mod ray;
mod rtw_stb_image;
mod rtweekend;
mod sphere;
mod texture;
mod triangle;
mod vec3;
mod vec3color;

use crate::hittable_list::HittableList;
use crate::rtweekend::{INFINITY, random_double, random_double_in_range};
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};
use camera::Camera;
use std::sync::Arc;

use crate::bvh::BvhNode;
use crate::constant_medium::ConstantMedium;
use crate::hittable::{Hittable, RotateY, Translate};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::model::load_obj_;
use crate::quad::{Quad, make_box};
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::vec3color::Color;
use console::style;
use image::RgbImage;

fn bouncing_spheres() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::from_color(
        0.32,
        Color::new_vec3(0.2, 0.3, 0.1),
        Color::new_vec3(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new_vec3(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(checker)),
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
                    let sphere_material = Arc::new(Lambertian::from_color(albedo));
                    let center2 =
                        center + Vec3::new_vec3(0.0, random_double_in_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::new_move(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = random_double_in_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal { albedo, fuzz });
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric {
                        refractive_index: 1.5,
                    });
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric {
        refractive_index: 1.50,
    });
    world.add(Arc::new(Sphere::new(
        Point3::new_vec3(0.0, 1.0, 0.0),
        1.0,
        material1.clone(),
    )));
    let material2 = Arc::new(Lambertian::from_color(Color::new_vec3(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new_vec3(-4.0, 1.0, 0.0),
        1.0,
        material2.clone(),
    )));
    let material3 = Arc::new(Metal {
        albedo: Color::new_vec3(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Arc::new(Sphere::new(
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
    let background = Color::new_vec3(0.70, 0.80, 1.00);
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
        background,
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
    let checker = Arc::new(CheckerTexture::from_color(
        0.32,
        Color::new_vec3(0.2, 0.3, 0.1),
        Color::new_vec3(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new_vec3(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker)),
    )));
    let checker = Arc::new(CheckerTexture::from_color(
        0.32,
        Color::new_vec3(0.2, 0.3, 0.1),
        Color::new_vec3(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new_vec3(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker)),
    )));
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let background = Color::new_vec3(0.70, 0.80, 1.00);
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
        background,
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
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new(earth_texture.clone()));
    let globe = Arc::new(Sphere::new(
        Point3::new_vec3(0.0, 0.0, 0.0),
        2.0,
        earth_surface.clone(),
    ));
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let background = Color::new_vec3(0.70, 0.80, 1.00);
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
        background,
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
    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(
        Point3::new_vec3(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new_vec3(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let background = Color::new_vec3(0.70, 0.80, 1.00);
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
        background,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );
    let img: RgbImage = camera.render(&world);
    let path = std::path::Path::new("output/book2/image15.png");
    std::fs::create_dir_all(path.parent().unwrap())?;
    img.save(path)?;

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    Ok(())
}
fn quads() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();
    let left_red = Arc::new(Lambertian::from_color(Color::new_vec3(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::from_color(Color::new_vec3(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::from_color(Color::new_vec3(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::from_color(Color::new_vec3(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::from_color(Color::new_vec3(0.2, 0.8, 0.8)));
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(-3.0, -2.0, 5.0),
        Vec3::new_vec3(0.0, 0.0, -4.0),
        Vec3::new_vec3(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(-2.0, -2.0, 0.0),
        Vec3::new_vec3(4.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(3.0, -2.0, 1.0),
        Vec3::new_vec3(0.0, 0.0, 4.0),
        Vec3::new_vec3(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(-2.0, 3.0, 1.0),
        Vec3::new_vec3(4.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(-2.0, -3.0, 5.0),
        Vec3::new_vec3(4.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let aspect_ratio = 1.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let background = Color::new_vec3(0.70, 0.80, 1.00);
    let vfov = 80.0;
    let lookfrom = Point3::new_vec3(0.0, 0.0, 9.0);
    let lookat = Point3::new_vec3(0.0, 0.0, 0.0);
    let vup = Vec3::new_vec3(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let camera = Camera::initialize(
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
    );
    let img: RgbImage = camera.render(&world);
    let path = std::path::Path::new("output/book2/image16.png");
    std::fs::create_dir_all(path.parent().unwrap())?;
    img.save(path)?;

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    Ok(())
}
fn simple_light() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(
        Point3::new_vec3(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new_vec3(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    let difflight = Arc::new(DiffuseLight::from_color(Color::new_vec3(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::new(
        Point3::new_vec3(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(3.0, 1.0, -2.0),
        Vec3::new_vec3(2.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 2.0, 0.0),
        difflight.clone(),
    )));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let background = Color::new_vec3(0.0, 0.0, 0.0);
    let vfov = 20.0;
    let lookfrom = Point3::new_vec3(26.0, 3.0, 6.0);
    let lookat = Point3::new_vec3(0.0, 2.0, 0.0);
    let vup = Vec3::new_vec3(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let camera = Camera::initialize(
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
    );
    let img: RgbImage = camera.render(&world);
    let path = std::path::Path::new("output/book2/image18.png");
    std::fs::create_dir_all(path.parent().unwrap())?;
    img.save(path)?;

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    Ok(())
}
fn cornell_box() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();
    let red = Arc::new(Lambertian::from_color(Color::new_vec3(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(Color::new_vec3(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(Color::new_vec3(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new_vec3(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new_vec3(555.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 555.0),
        Vec3::new_vec3(0.0, 555.0, 0.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(0.0, 0.0, 555.0),
        Vec3::new_vec3(0.0, 0.0, -555.0),
        Vec3::new_vec3(0.0, 555.0, 0.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(0.0, 555.0, 0.0),
        Vec3::new_vec3(555.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(0.0, 0.0, 555.0),
        Vec3::new_vec3(555.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(555.0, 0.0, 555.0),
        Vec3::new_vec3(-555.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 555.0, 0.0),
        white.clone(),
    )));
    let mut box1: Arc<dyn Hittable> = make_box(
        Point3::new_vec3(0.0, 0.0, 0.0),
        Point3::new_vec3(165.0, 330.0, 165.0),
        white.clone(),
    );
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(213.0, 554.0, 227.0),
        Vec3::new_vec3(130.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 105.0),
        light,
    )));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new_vec3(265.0, 0.0, 295.0)));
    world.add(box1);
    let mut box2: Arc<dyn Hittable> = make_box(
        Point3::new_vec3(0.0, 0.0, 0.0),
        Point3::new_vec3(165.0, 165.0, 165.0),
        white.clone(),
    );
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new_vec3(130.0, 0.0, 65.0)));
    world.add(box2);

    let aspect_ratio = 1.0;
    let image_width = 600;
    let samples_per_pixel = 1000;
    let max_depth = 50;
    let background = Color::new_vec3(0.0, 0.0, 0.0);
    let vfov = 40.0;
    let lookfrom = Point3::new_vec3(278.0, 278.0, -800.0);
    let lookat = Point3::new_vec3(278.0, 278.0, 0.0);
    let vup = Vec3::new_vec3(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let camera = Camera::initialize(
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
    );
    let img: RgbImage = camera.render(&world);
    let path = std::path::Path::new("output/book3/image5.png");
    std::fs::create_dir_all(path.parent().unwrap())?;
    img.save(path)?;

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    Ok(())
}
fn cornell_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();
    let red = Arc::new(Lambertian::from_color(Color::new_vec3(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(Color::new_vec3(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(Color::new_vec3(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new_vec3(7.0, 7.0, 7.0)));

    world.add(Arc::new(Quad::new(
        Point3::new_vec3(555.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 555.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(0.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 555.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(113.0, 554.0, 127.0),
        Vec3::new_vec3(330.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(0.0, 555.0, 0.0),
        Vec3::new_vec3(555.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(0.0, 0.0, 0.0),
        Vec3::new_vec3(555.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(0.0, 0.0, 555.0),
        Vec3::new_vec3(555.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 555.0, 0.0),
        white.clone(),
    )));
    let mut box1: Arc<dyn Hittable> = make_box(
        Point3::new_vec3(0.0, 0.0, 0.0),
        Point3::new_vec3(165.0, 330.0, 165.0),
        white.clone(),
    );
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new_vec3(265.0, 0.0, 295.0)));
    let mut box2: Arc<dyn Hittable> = make_box(
        Point3::new_vec3(0.0, 0.0, 0.0),
        Point3::new_vec3(165.0, 165.0, 165.0),
        white.clone(),
    );
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new_vec3(130.0, 0.0, 65.0)));
    world.add(Arc::new(ConstantMedium::from_color(
        box1,
        0.01,
        Color::new_vec3(0.0, 0.0, 0.0),
    )));
    world.add(Arc::new(ConstantMedium::from_color(
        box2,
        0.01,
        Color::new_vec3(1.0, 1.0, 1.0),
    )));

    let aspect_ratio = 1.0;
    let image_width = 600;
    let samples_per_pixel = 200;
    let max_depth = 50;
    let background = Color::new_vec3(0.0, 0.0, 0.0);
    let vfov = 40.0;
    let lookfrom = Point3::new_vec3(278.0, 278.0, -800.0);
    let lookat = Point3::new_vec3(278.0, 278.0, 0.0);
    let vup = Vec3::new_vec3(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let camera = Camera::initialize(
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
    );
    let img: RgbImage = camera.render(&world);
    let path = std::path::Path::new("output/book2/image22.png");
    std::fs::create_dir_all(path.parent().unwrap())?;
    img.save(path)?;

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    Ok(())
}
fn final_scene(
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::from_color(Color::new_vec3(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_in_range(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(make_box(
                Point3::new_vec3(x0, y0, z0),
                Point3::new_vec3(x1, y1, z1),
                ground.clone(),
            ));
        }
    }
    let mut world = HittableList::new();
    world.add(BvhNode::from_list(boxes1));
    let light = Arc::new(DiffuseLight::from_color(Color::new_vec3(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        Point3::new_vec3(123.0, 554.0, 147.0),
        Vec3::new_vec3(300.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = Point3::new_vec3(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new_vec3(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::from_color(Color::new_vec3(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_move(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new_vec3(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric {
            refractive_index: 1.5,
        }),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new_vec3(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal {
            albedo: Color::new_vec3(0.8, 0.8, 0.9),
            fuzz: 1.0,
        }),
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new_vec3(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric {
            refractive_index: 1.5,
        }),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::from_color(
        boundary,
        0.2,
        Color::new_vec3(0.2, 0.4, 0.9),
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new_vec3(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric {
            refractive_index: 1.5,
        }),
    ));
    world.add(Arc::new(ConstantMedium::from_color(
        boundary,
        0.0001,
        Color::new_vec3(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian::new(Arc::new(ImageTexture::new("earthmap.jpg"))));
    world.add(Arc::new(Sphere::new(
        Point3::new_vec3(400.0, 200.0, 400.0),
        100.0,
        emat.clone(),
    )));

    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.add(Arc::new(Sphere::new(
        Point3::new_vec3(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::from_color(Color::new_vec3(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Point3::random_in_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(BvhNode::from_list(boxes2), 15.0)),
        Vec3::new_vec3(-100.0, 270.0, 395.0),
    )));

    let aspect_ratio = 1.0;
    let background = Color::new_vec3(0.0, 0.0, 0.0);
    let vfov = 40.0;
    let lookfrom = Point3::new_vec3(478.0, 278.0, -600.0);
    let lookat = Point3::new_vec3(278.0, 278.0, 0.0);
    let vup = Vec3::new_vec3(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let camera = Camera::initialize(
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
    );
    let img: RgbImage = camera.render(&world);
    let path = std::path::Path::new("output/book2/image23.png");
    std::fs::create_dir_all(path.parent().unwrap())?;
    img.save(path)?;

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    Ok(())
}
fn test_obj() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Current working directory: {:?}",
        std::env::current_dir().unwrap()
    );
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let obj_path = std::path::PathBuf::from(manifest_dir)
        .join("doc")
        .join("assets")
        .join("Mauser_98K.obj");

    let model_mat = Arc::new(Lambertian::from_color(Color::new_vec3(0.8, 0.6, 0.2)));
    let model_world = load_obj_(&obj_path, model_mat);

    let bvh_model = BvhNode::from_list(model_world);
    let bbox = bvh_model.bounding_box();

    // 计算包围盒中心和对角线长度
    let min_p = Point3::new_vec3(bbox.x.min, bbox.y.min, bbox.z.min);
    let max_p = Point3::new_vec3(bbox.x.max, bbox.y.max, bbox.z.max);
    let center = (min_p + max_p) * 0.5;
    let size = (max_p - min_p).length();
    let size = if size < 1e-6 { 10.0 } else { size };
    let lookfrom = center + Vec3::new_vec3(1.0, 0.5, 1.0) * size * 1.5;
    let lookat = center;

    let mut world = HittableList::new();
    world.add(bvh_model);

    // 相机参数
    let aspect_ratio = 1.0;
    let image_width = 600;
    let samples_per_pixel = 200;
    let max_depth = 50;
    let background = Color::new_vec3(0.7, 0.8, 1.0);
    let vfov = 40.0;
    let vup = Vec3::new_vec3(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let camera = Camera::initialize(
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
    );
    let img = camera.render(&world);
    let path = std::path::Path::new("output/test_obj.png");
    std::fs::create_dir_all(path.parent().unwrap())?;
    img.save(path)?;
    println!("Test OBJ image saved to {}", path.display());
    Ok(())
}
fn main() {
    match 7 {
        1 => bouncing_spheres().unwrap(),
        2 => checkered_spheres().unwrap(),
        3 => earth().unwrap(),
        4 => perlin_spheres().unwrap(),
        5 => quads().unwrap(),
        6 => simple_light().unwrap(),
        7 => cornell_box().unwrap(),
        8 => cornell_smoke().unwrap(),
        9 => final_scene(800, 10000, 40).unwrap(),
        10 => test_obj().unwrap(),
        _ => {}
    }
}
