use std::f64::consts::PI;

use rust_tracer::{
    hittables::{self, Sphere},
    materials::Lambertian,
    math::{Color, Point3},
};

fn main() {
    // World
    let mut world = hittables::HittableList::new();
    let r = (PI / 4.0).cos();

    let left_material = Box::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let right_material = Box::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    world.add(Box::new(Sphere::new(
        Point3::new(-r, 0.0, -1.0),
        r,
        left_material,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(r, 0.0, -1.0),
        r,
        right_material,
    )));

    rust_tracer::run(400, 100, 50, &world);
}
