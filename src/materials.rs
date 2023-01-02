use rand::{rngs::ThreadRng, Rng};

use crate::{
    hittables::HitRecord,
    math::{Color, Ray, Vector3},
    utils::{random_in_unit_sphere, random_unit_vector, reflect, refract},
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

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
        let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, rec: HitRecord, rng: &mut ThreadRng) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.unit_vector();

        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen()
        {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };

        Some((Color::new(1.0, 1.0, 1.0), Ray::new(rec.point, direction)))
    }
}
