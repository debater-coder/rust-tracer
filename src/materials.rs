use rand::rngs::ThreadRng;

use crate::{
    hittables::HitRecord,
    math::{Color, Ray, Vector3},
    utils::{random_in_unit_sphere, random_unit_vector, reflect},
};

pub trait Material {
    fn scatter(&self, ray: Ray, rec: HitRecord, rng: &mut ThreadRng) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, rec: HitRecord, rng: &mut ThreadRng) -> Option<(Color, Ray)> {
        let scatter_direction = rec.normal + random_unit_vector(rng);

        // Catch degenerate scatter direction
        let scatter_direction = if scatter_direction.near_zero() {
            rec.normal
        } else {
            scatter_direction
        };

        Some((self.albedo, Ray::new(rec.point, scatter_direction)))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, rec: HitRecord, rng: &mut ThreadRng) -> Option<(Color, Ray)> {
        let reflected = reflect(ray.direction.unit_vector(), rec.normal);
        let scattered = Ray::new(
            rec.point,
            reflected + self.fuzz * random_in_unit_sphere(rng),
        );
        if Vector3::dot(&scattered.direction, &rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
