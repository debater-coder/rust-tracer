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
            let r = (i as f64) / ((image_width - 1) as f64);
            let g = (j as f64) / ((image_height - 1) as f64);
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{ir} {ig} {ib}");
        }
    }

    eprintln!("\nDone");
}