use rand::{thread_rng, Rng};
use rust_tracer::{
    camera::Camera,
    hittables::{HittableList, Sphere},
    materials::{Dielectric, Lambertian, Material, Metal},
    math::{Color, Point3, Vector3},
};

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let mut rng = thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Box<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::new(rng.gen(), rng.gen(), rng.gen())
                        * Color::new(rng.gen(), rng.gen(), rng.gen());
                    Lambertian::new(albedo)
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::new(
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                    );
                    let fuzz = rng.gen_range(0.0..0.5);
                    Metal::new(albedo, fuzz)
                } else {
                    // glass
                    Dielectric::new(1.5)
                };

                world.add(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    world
}

fn main() {
    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vector3(0.0, 1.0, 0.0),
        20.0,
        16.0 / 9.0,
        0.1,
        10.0,
    );

    // Render
    rust_tracer::render(400, 10, 5, &world, &camera);
}
