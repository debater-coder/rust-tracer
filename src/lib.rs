use std::{
    io::{self, stderr, stdout, Write},
    sync::mpsc,
    thread,
    time::Instant,
};

use hittables::HittableList;
use math::{Color, Ray};
use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::{camera::Camera, image::Image};

pub mod camera;
pub mod hittables;
pub mod image;
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

fn render(
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: usize,
    world: &HittableList,
    camera: Camera,
) -> Image {
    // Image
    let aspect_ratio = camera.aspect_ratio;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // Render

    let mut rng = thread_rng();
    let mut image = Image::new(image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j}  ");
        stderr().flush().unwrap_or_default();

        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                pixel_color += ray_color(camera.ray(u, v, &mut rng), &world, &mut rng, max_depth);
            }

            image
                .pixels
                .push(pixel_color * (1.0 / samples_per_pixel as f64));
        }
    }

    image
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
///     0.1,
///     10.0
/// );
///
/// // Render
/// rust_tracer::render_to_stdout(400, 100, 50, &world, &camera);
/// ```
pub fn render_to_stdout<F>(
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: usize,
    build_world: F,
    camera: Camera,
    threads: usize,
) -> io::Result<()>
where
    F: Fn() -> HittableList + Send + 'static + Copy,
{
    let now = Instant::now();

    let (tx, rx) = mpsc::channel();
    let image_height = (image_width as f64 / camera.aspect_ratio) as u32;

    for _ in 0..threads {
        let tx = tx.clone();

        thread::spawn(move || {
            tx.send(render(
                image_width,
                samples_per_pixel / threads as u32,
                max_depth,
                &build_world(),
                camera,
            ))
            .unwrap();
        });
    }

    let mut results = Vec::new();

    for _ in 0..threads {
        results.push(rx.recv().unwrap());
    }

    Image::average(results, image_width, image_height).write_as_ppm(&mut stdout().lock())?;

    eprintln!("\nCompleted in {} seconds.", now.elapsed().as_secs());

    Ok(())
}
