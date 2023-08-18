use crate::{
	matrix::matrix4d::Matrix4D,
	primitives::{point::Point, ray::Ray},
	visualisation::{canvas::Canvas, world::World},
};
use std::f64;

#[derive(Debug)]
pub struct Camera {
	pub hsize: f64,
	pub vsize: f64,
	pub field_of_view: f64,
	pub transform: Matrix4D,
	pub pixel_size: f64,
	pub half_width: f64,
	pub half_height: f64,
}

impl Camera {
	pub fn new(hsize: f64, vsize: f64, field_of_view: f64) -> Self {
		let half_view = (field_of_view / 2.0).tan();
		let aspect = hsize / vsize;

		let mut half_width = half_view * aspect;
		let mut half_height = half_view;

		if aspect >= 1.0 {
			half_width = half_view;
			half_height = half_view / aspect;
		}

		let pixel_size = (half_width * 2.0) / hsize;

		Camera {
			hsize,
			vsize,
			field_of_view,
			transform: Matrix4D::identity(),
			pixel_size,
			half_width,
			half_height,
		}
	}

	fn ray_for_pixel(&self, px: f64, py: f64) -> Ray {
		let x_offset = (px + 0.5) * self.pixel_size;
		let y_offset = (py + 0.5) * self.pixel_size;

		// camera looks toward -z, so +x is to the left
		// camera origin is top left angle
		let world_x = self.half_width - x_offset;
		let world_y = self.half_height - y_offset;

		let transform = self.transform.inverse().expect("Cannot inverse camera transform");

		// canvas is assumed to be 1.0 away
		let pixel = transform * Point::new(world_x, world_y, -1.0);
		let origin = transform * Point::new(0.0, 0.0, 0.0);
		let direction = (pixel - origin).normalise();
		Ray::new(origin, direction)
	}

	pub fn render(&self, world: &World) -> Result<Canvas, String> {
		let mut image = Canvas::new(self.hsize as usize, self.vsize as usize);

		for y in 0..self.vsize as usize {
			for x in 0..self.hsize as usize {
				let r = self.ray_for_pixel(x as f64, y as f64);
				let color = world.color_at(&r, None);
				match color {
					Ok(color_value) => image.write_pixel(x, y, color_value),
					Err(err) => return Err(err),
				}
			}
		}
		Ok(image)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::primitives::{color::Color, point::Point, transformations::*, vector::Vector};
	use std::f64;

	#[test]
	fn pixel_size_test() {
		let c = Camera::new(200.0, 125.0, f64::consts::PI / 2.0);
		approx::assert_relative_eq!(c.pixel_size, 0.01);
		let c = Camera::new(125.0, 200.0, f64::consts::PI / 2.0);
		approx::assert_relative_eq!(c.pixel_size, 0.01);
	}

	#[test]
	fn ray_for_pixel_test() {
		let c = Camera::new(201.0, 101.0, f64::consts::PI / 2.0);
		let r = c.ray_for_pixel(100.0, 50.0);
		approx::assert_relative_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
		approx::assert_relative_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));

		let c = Camera::new(201.0, 101.0, f64::consts::PI / 2.0);
		let r = c.ray_for_pixel(0.0, 0.0);
		approx::assert_relative_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
		approx::assert_relative_eq!(
			r.direction,
			Vector::new(0.66519, 0.33259, -0.66851),
			epsilon = 1e-5
		);

		let mut c = Camera::new(201.0, 101.0, f64::consts::PI / 2.0);
		c.transform = rotation_y(f64::consts::PI / 4.0) * translation(0.0, -2.0, 5.0);
		let r = c.ray_for_pixel(100.0, 50.0);
		approx::assert_relative_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
		approx::assert_relative_eq!(
			r.direction,
			Vector::new(2.0f64.sqrt() / 2.0, 0.0, -2.0f64.sqrt() / 2.0)
		);
	}

	#[test]
	fn render_test() {
		let w = World::default();
		let mut c = Camera::new(11.0, 11.0, f64::consts::PI / 2.0);

		let from = Point::new(0.0, 0.0, -5.0);
		let to = Point::new(0.0, 0.0, 0.0);
		let up = Vector::new(0.0, 1.0, 0.0);

		c.transform = view_transform(from, to, up);
		let image = c.render(&w).unwrap();
		approx::assert_relative_eq!(
			image.pixel_at(5, 5),
			Color::new(0.38066, 0.47583, 0.2855),
			epsilon = 1e-5
		);
	}
}
