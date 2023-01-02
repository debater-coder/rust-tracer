use rand::rngs::ThreadRng;

use crate::{
    math::{Point3, Ray, Vector3},
    utils::random_in_unit_disk,
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    lens_radius: f64,
    pub aspect_ratio: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vector3,
        vfov: f64,
        aspect_ratio: f64,
        apeture: f64,
        focus_dist: f64,
    ) -> Self {
        let viewport_height = 2.0 * (vfov.to_radians() / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = Vector3::cross(&vup, &w).unit_vector();
        let v = Vector3::cross(&w, &u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        let lens_radius = apeture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            aspect_ratio,
            u,
            v,
            lens_radius,
        }
    }

    pub fn ray(&self, s: f64, t: f64, rng: &mut ThreadRng) -> Ray {
        let focus_disk = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * focus_disk.x() + self.v * focus_disk.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
