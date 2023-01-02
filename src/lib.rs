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

/// Renders a scene given an image width, number of samples, max recursion depth, a world, and a camera
///
/// # Examples
/// ```
/// use rust_tracer::{
///     camera::Camera,
///     hittables::{self, Sphere},
///     materials::{Dielectric, Lambertian, Metal},
///     math::{Color, Point3, Vector3},
/// };
///
/// // World
/// let mut world = hittables::HittableList::new();
///
/// let ground_material = Lambertian::new(Color::new(0.8, 0.8, 0.0));
/// let center_material = Lambertian::new(Color::new(0.1, 0.2, 0.5));
/// let left_material = Dielectric::new(1.5);
/// let right_material = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);
///
/// world.add(Sphere::new(
///     Point3::new(0.0, -100.5, -1.0),
///     100.0,
///     ground_material,
/// ));
/// world.add(Sphere::new(
///     Point3::new(0.0, 0.0, -1.0),
///     0.5,
///     center_material,
/// ));
/// world.add(Sphere::new(
///     Point3::new(-1.0, 0.0, -1.0),
///     0.5,
///     left_material.clone(),
/// ));
/// world.add(Sphere::new(
///     Point3::new(-1.0, 0.0, -1.0),
///     -0.45,
///     left_material,
/// ));
/// world.add(Sphere::new(
///     Point3::new(1.0, 0.0, -1.0),
///     0.5,
///     right_material,
/// ));
///
/// // Camera
/// let camera = Camera::new(
///     Point3::new(-2.0, 2.0, 1.0),
///     Point3::new(0.0, 0.0, -1.0),
///     Vector3(0.0, 1.0, 0.0),
///     20.0,
///     16.0 / 9.0,
/// );
///
/// // Render
/// ```
pub fn render(
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: usize,
    world: &HittableList,
    camera: &Camera,
) {
    // Image
    let aspect_ratio = camera.aspect_ratio;
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
