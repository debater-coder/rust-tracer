use crate::math::Color;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) -> String {
    let scale = 1.0 / samples_per_pixel as f64;
    let pixel_color = pixel_color * scale;

    let ir = (255.999 * pixel_color.x()) as i32;
    let ig = (255.999 * pixel_color.y()) as i32;
    let ib = (255.999 * pixel_color.z()) as i32;

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
