use crate::primitives::{color::Color, matrix::matrix4d::Matrix4D, point::Point};

use super::color_pattern::Pattern;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TestPattern {
	pub transform: Matrix4D,
}

impl TestPattern {
	pub fn new() -> Self {
		Self { transform: Matrix4D::identity() }
	}
}

impl Pattern for TestPattern {
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
		Color::new(point.tuple.x, point.tuple.y, point.tuple.z)
	}
}
