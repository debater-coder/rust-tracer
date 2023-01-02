use rand::rngs::ThreadRng;

use crate::math::{Color, Vector3};

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) -> String {
    let scale = 1.0 / samples_per_pixel as f64;
    let pixel_color = pixel_color * scale;

    let ir = (256.0 * clamp(pixel_color.x().sqrt(), 0.0, 0.999)) as i32;
    let ig = (256.0 * clamp(pixel_color.y().sqrt(), 0.0, 0.999)) as i32;
    let ib = (256.0 * clamp(pixel_color.z().sqrt(), 0.0, 0.999)) as i32;

    format!("{ir} {ig} {ib}")
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vector3 {
    use rand::prelude::*;

    loop {
        let p = Vector3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector(rng: &mut ThreadRng) -> Vector3 {
    random_in_unit_sphere(rng).unit_vector()
}

pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - 2.0 * v.dot(&n) * n
}

pub fn refract(uv: Vector3, n: Vector3, etai_over_etat: f64) -> Vector3 {
    let cos_theta = (-uv).dot(&n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}
