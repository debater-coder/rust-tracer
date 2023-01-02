use std::io::{stderr, Write};

use hittable::HittableList;
use math::{Color, Ray};
use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::{camera::Camera, hittable::Sphere, math::Point3, utils::write_color};

pub mod camera;
pub mod hittable;
pub mod math;
pub mod utils;

fn ray_color(ray: Ray, world: &HittableList, rng: &mut ThreadRng, depth: usize) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        let target = rec.point + rec.normal + utils::random_in_unit_sphere(rng);
        return 0.5
            * ray_color(
                Ray::new(rec.point, target - rec.point),
                world,
                rng,
                depth - 1,
            );
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + Color::new(0.5, 0.7, 1.0) * t
}

pub fn run() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = hittable::HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3\n{image_width} {image_height}\n255");

    let mut rng = thread_rng();

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j}  ");
        stderr().flush().unwrap_or_default();

        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                pixel_color += ray_color(camera.ray(u, v), &world, &mut rng, max_depth);
            }

            println!("{}", write_color(pixel_color, samples_per_pixel));
        }
    }

    eprintln!("\nDone");
}
