use std::io::{stderr, Write};

use hittables::HittableList;
use math::{Color, Ray};
use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::{camera::Camera, utils::write_color};

pub mod camera;
pub mod hittables;
pub mod materials;
pub mod math;
pub mod utils;

fn ray_color(ray: Ray, world: &HittableList, rng: &mut ThreadRng, depth: usize) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        return if let Some((attenuation, scattered)) = rec.material.scatter(ray, rec, rng) {
            attenuation * ray_color(scattered, world, rng, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        };
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + Color::new(0.5, 0.7, 1.0) * t
}

pub fn render(
    image_width: i32,
    samples_per_pixel: i32,
    max_depth: usize,
    world: &HittableList,
    camera: &Camera,
) {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

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
