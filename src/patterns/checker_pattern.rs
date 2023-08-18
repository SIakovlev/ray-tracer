use crate::primitives::{color::Color, matrix::matrix4d::Matrix4D, point::Point};

use super::color_pattern::Pattern;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CheckerPattern {
	pub a: Color,
	pub b: Color,
	pub transform: Matrix4D,
}

impl CheckerPattern {
	pub fn new(a: Color, b: Color) -> Self {
		Self { a, b, transform: Matrix4D::identity() }
	}
}

impl Pattern for CheckerPattern {
	fn transform(&self) -> &Matrix4D {
		&self.transform
	}

	fn get_transform(&mut self) -> &mut Matrix4D {
		&mut self.transform
	}

	fn set_transform(&mut self, transform: Matrix4D) {
		self.transform = transform
	}

	fn pattern_at(&self, point: &Point) -> Color {
		let condition =
			(point.tuple.x.floor() + point.tuple.y.floor() + point.tuple.z.floor()) as i64 % 2;
		match condition {
			0 => self.a,
			_ => self.b,
		}
	}
}
