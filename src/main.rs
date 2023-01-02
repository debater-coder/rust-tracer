use rust_tracer::{
    camera::Camera,
    hittables::{self, Sphere},
    materials::{Dielectric, Lambertian, Metal},
    math::{Color, Point3, Vector3},
};

fn main() {
    // World
    let mut world = hittables::HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let center_material = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let left_material = Dielectric::new(1.5);
    let right_material = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        ground_material,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        center_material,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        left_material.clone(),
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        left_material,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        right_material,
    ));

    // Camera
    let camera = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vector3(0.0, 1.0, 0.0),
        90.0,
        16.0 / 9.0,
    );

    // Render
    rust_tracer::render(400, 100, 50, &world, &camera);
}
