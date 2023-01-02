use rand::rngs::ThreadRng;

use crate::{
    hittables::HitRecord,
    math::{Color, Ray},
    utils::random_unit_vector,
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
