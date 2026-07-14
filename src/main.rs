mod aabb;
mod bvh;
mod camera;
mod constant_medium;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod quad;
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
use crate::constant_medium::ConstantMedium;
use crate::hittable::{Hittable, RotateY, Translate};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::quad::{Quad, make_box};
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
    let pertext = Rc::new(NoiseTexture::new(4.0));
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
    let left_red = Rc::new(Lambertian::from_color(Color::new_vec3(1.0, 0.2, 0.2)));
    let back_green = Rc::new(Lambertian::from_color(Color::new_vec3(0.2, 1.0, 0.2)));
    let right_blue = Rc::new(Lambertian::from_color(Color::new_vec3(0.2, 0.2, 1.0)));
    let upper_orange = Rc::new(Lambertian::from_color(Color::new_vec3(1.0, 0.5, 0.0)));
    let lower_teal = Rc::new(Lambertian::from_color(Color::new_vec3(0.2, 0.8, 0.8)));
    world.add(Rc::new(Quad::new(
        Point3::new_vec3(-3.0, -2.0, 5.0),
        Vec3::new_vec3(0.0, 0.0, -4.0),
        Vec3::new_vec3(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new_vec3(-2.0, -2.0, 0.0),
        Vec3::new_vec3(4.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new_vec3(3.0, -2.0, 1.0),
        Vec3::new_vec3(0.0, 0.0, 4.0),
        Vec3::new_vec3(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new_vec3(-2.0, 3.0, 1.0),
        Vec3::new_vec3(4.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Rc::new(Quad::new(
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
    let pertext = Rc::new(NoiseTexture::new(4.0));
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
    let difflight = Rc::new(DiffuseLight::from_color(Color::new_vec3(4.0, 4.0, 4.0)));
    world.add(Rc::new(Sphere::new(
        Point3::new_vec3(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Rc::new(Quad::new(
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
    let red = Rc::new(Lambertian::from_color(Color::new_vec3(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::from_color(Color::new_vec3(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::from_color(Color::new_vec3(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::from_color(Color::new_vec3(15.0, 15.0, 15.0)));

    world.add(Rc::new(Quad::new(
        Point3::new_vec3(555.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 555.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new_vec3(0.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 555.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new_vec3(343.0, 554.0, 332.0),
        Vec3::new_vec3(-130.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new_vec3(0.0, 0.0, 0.0),
        Vec3::new_vec3(555.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new_vec3(555.0, 555.0, 555.0),
        Vec3::new_vec3(-555.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new_vec3(0.0, 0.0, 555.0),
        Vec3::new_vec3(555.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 555.0, 0.0),
        white.clone(),
    )));
    let mut box1: Rc<dyn Hittable> = make_box(
        Point3::new_vec3(0.0, 0.0, 0.0),
        Point3::new_vec3(165.0, 330.0, 165.0),
        white.clone(),
    );
    box1 = Rc::new(RotateY::new(box1, 15.0));
    box1 = Rc::new(Translate::new(box1, Vec3::new_vec3(265.0, 0.0, 295.0)));
    world.add(box1);
    let mut box2: Rc<dyn Hittable> = make_box(
        Point3::new_vec3(0.0, 0.0, 0.0),
        Point3::new_vec3(165.0, 165.0, 165.0),
        white.clone(),
    );
    box2 = Rc::new(RotateY::new(box2, -18.0));
    box2 = Rc::new(Translate::new(box2, Vec3::new_vec3(130.0, 0.0, 65.0)));
    world.add(box2);

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
    let path = std::path::Path::new("output/book2/image21.png");
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
    let red = Rc::new(Lambertian::from_color(Color::new_vec3(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::from_color(Color::new_vec3(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::from_color(Color::new_vec3(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::from_color(Color::new_vec3(7.0, 7.0, 7.0)));

    world.add(Rc::new(Quad::new(
        Point3::new_vec3(555.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 555.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new_vec3(0.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 555.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new_vec3(113.0, 554.0, 127.0),
        Vec3::new_vec3(330.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new_vec3(0.0, 555.0, 0.0),
        Vec3::new_vec3(555.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new_vec3(0.0, 0.0, 0.0),
        Vec3::new_vec3(555.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new_vec3(0.0, 0.0, 555.0),
        Vec3::new_vec3(555.0, 0.0, 0.0),
        Vec3::new_vec3(0.0, 555.0, 0.0),
        white.clone(),
    )));
    let mut box1: Rc<dyn Hittable> = make_box(
        Point3::new_vec3(0.0, 0.0, 0.0),
        Point3::new_vec3(165.0, 330.0, 165.0),
        white.clone(),
    );
    box1 = Rc::new(RotateY::new(box1, 15.0));
    box1 = Rc::new(Translate::new(box1, Vec3::new_vec3(265.0, 0.0, 295.0)));
    let mut box2: Rc<dyn Hittable> = make_box(
        Point3::new_vec3(0.0, 0.0, 0.0),
        Point3::new_vec3(165.0, 165.0, 165.0),
        white.clone(),
    );
    box2 = Rc::new(RotateY::new(box2, -18.0));
    box2 = Rc::new(Translate::new(box2, Vec3::new_vec3(130.0, 0.0, 65.0)));
    world.add(Rc::new(ConstantMedium::from_color(
        box1,
        0.01,
        Color::new_vec3(0.0, 0.0, 0.0),
    )));
    world.add(Rc::new(ConstantMedium::from_color(
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
fn main() {
    match 8 {
        1 => bouncing_spheres().unwrap(),
        2 => checkered_spheres().unwrap(),
        3 => earth().unwrap(),
        4 => perlin_spheres().unwrap(),
        5 => quads().unwrap(),
        6 => simple_light().unwrap(),
        7 => cornell_box().unwrap(),
        8 => cornell_smoke().unwrap(),
        _ => {}
    }
}
