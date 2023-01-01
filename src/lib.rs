use std::io::{stderr, Write};

pub mod math;

pub fn run() {
    // Image

    let (image_width, image_height) = (256, 256);

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j}");
        stderr().flush().unwrap_or_default();

        for i in 0..image_width {
            let pixel_color = math::Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );

            println!("{}", write_color(&pixel_color));
        }
    }

    eprintln!("\nDone");
}

fn write_color(pixel_color: &math::Color) -> String {
    let ir = (255.999 * pixel_color.x()) as i32;
    let ig = (255.999 * pixel_color.y()) as i32;
    let ib = (255.999 * pixel_color.z()) as i32;

    format!("{ir} {ig} {ib}")
}
