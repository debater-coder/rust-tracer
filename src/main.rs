use rust_tracer::{
    hittables::{self, Sphere},
    materials::{Dielectric, Lambertian, Metal},
    math::{Color, Point3},
};

fn main() {
    // World
    let mut world = hittables::HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Color::new(0.5, 0.8, 0.3))),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        Box::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0)),
    )));

    rust_tracer::run(400, 100, 50, &world);
}
