use crate::{color::Color, matrix::matrix4d::Matrix4D, point::Point};

use super::color_pattern::Pattern;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GradientPattern {
	pub a: Color,
	pub b: Color,
	pub transform: Matrix4D,
}

impl GradientPattern {
	pub fn new(a: Color, b: Color) -> Self {
		Self { a, b, transform: Matrix4D::identity() }
	}
}

impl Pattern for GradientPattern {
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
		let distance = self.b - self.a;
		let fraction = point.tuple.x - point.tuple.x.floor();
		self.a + distance * fraction
	}
}
