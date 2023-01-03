use std::{
    io,
    io::Write,
    ops::{AddAssign, Div},
};

use crate::{math::Color, utils::write_color};

#[derive(Clone)]
pub struct Image {
    pub pixels: Vec<Color>,
    pub width: u32,
    pub height: u32,
}

impl AddAssign for Image {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..self.pixels.len() {
            self.pixels[i] = self.pixels[i] + rhs.pixels[i];
        }
    }
}

impl Div<f64> for Image {
    type Output = Image;

    fn div(self, rhs: f64) -> Self::Output {
        let mut pixels = Vec::with_capacity((self.width * self.height) as usize);

        for i in 0..self.pixels.len() {
            pixels.push(self.pixels[i] / rhs);
        }

        Image {
            pixels,
            width: self.width,
            height: self.height,
        }
    }
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            pixels: Vec::with_capacity((width * height) as usize),
            width,
            height,
        }
    }

    pub fn write_as_ppm(&self, lock: &mut dyn Write) -> io::Result<()> {
        writeln!(lock, "P3\n{0} {1}\n255", self.width, self.height)?;

        for pixel in &self.pixels {
            writeln!(lock, "{}", write_color(*pixel, 1))?;
        }

        Ok(())
    }

    pub fn average(images: Vec<Image>, width: u32, height: u32) -> Image {
        let mut output_image = Image {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); (width * height) as usize],
        };

        for image in images.clone() {
            output_image += image;
        }

        output_image / images.len() as f64
    }
}
