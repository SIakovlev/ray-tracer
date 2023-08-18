use crate::primitives::color::Color;
use std::{fs::File, io::Write};

const MAX_PPM_LINE_WIDTH: usize = 70;

pub struct Canvas {
	pub pixels: Vec<Color>,
	pub width: usize,
	pub height: usize,
}

impl Canvas {
	pub fn new(width: usize, height: usize) -> Self {
		let pixels = vec![Color::new(0.0, 0.0, 0.0); width * height];

		Canvas { pixels, width, height }
	}

	pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
		assert!(y < self.height && x < self.width, "Provided values x: {}, y: {}", x, y);
		self.pixels[self.width * y + x] = color;
	}

	pub fn pixel_at(&self, x: usize, y: usize) -> Color {
		self.pixels[self.width * y + x]
	}

	pub fn to_ppm(&self, max_color_value: u32, path: &str) {
		// Construct header
		let mut ppm_data = format!("P3\n{} {}\n{}\n", self.width, self.height, max_color_value);

		// Construct data
		let mut line = String::with_capacity(MAX_PPM_LINE_WIDTH);

		for (idx, pixel) in self.pixels.iter().enumerate() {
			let (red, green, blue) = pixel.normalise(0.0, max_color_value as f64);

			for value in [red, green, blue] {
				let tmp = format!("{} ", value);
				if line.len() > MAX_PPM_LINE_WIDTH - &tmp.len() - 2 {
					line.push_str("\n");
					ppm_data.push_str(&line);
					line = String::with_capacity(MAX_PPM_LINE_WIDTH);
				}
				line.push_str(&tmp);
			}

			if (idx + 1) % self.width == 0 && line.len() != 0 {
				line.push_str("\n");
				ppm_data.push_str(&line);
				line = String::with_capacity(MAX_PPM_LINE_WIDTH);
			}
		}

		#[cfg(debug_assertions)]
		println!("{}", &ppm_data);

		let mut output = match File::create(&path) {
			Err(e) => panic!("Couldn't open {}. Error msg: {}", &path, &e),
			Ok(file) => file,
		};
		write!(output, "{}", &ppm_data).expect("Couldn't write to the file");
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn sanity() {
		let c = Canvas::new(10, 20);
		let black = Color::new(0.0, 0.0, 0.0);
		assert_eq!(&c.pixels[0], &black);
		assert_eq!(&c.pixels[10], &black);
		assert_eq!(&c.pixels[199], &black);
	}

	#[test]
	fn writing_pixel() {
		let mut c = Canvas::new(10, 20);
		let red = Color::new(1.0, 0.0, 0.0);

		c.write_pixel(2, 3, red);
		assert_eq!(&c.pixel_at(2, 3), &red);
	}

	#[test]
	fn constucting_ppm_small() {
		let mut c = Canvas::new(5, 3);
		let c1 = Color::new(1.5, 0.0, 0.0);
		let c2 = Color::new(0.0, 0.5, 0.0);
		let c3 = Color::new(-0.5, 0.0, 1.0);

		c.write_pixel(0, 0, c1);
		c.write_pixel(2, 1, c2);
		c.write_pixel(4, 2, c3);

		c.to_ppm(255, "test.ppm");
	}

	#[test]
	fn constucting_ppm_large() {
		let mut c = Canvas::new(10, 2);
		let c1 = Color::new(1.0, 0.8, 0.6);

		for pixel in &mut c.pixels {
			*pixel = c1;
		}

		c.to_ppm(255, "test.ppm");
	}
}
