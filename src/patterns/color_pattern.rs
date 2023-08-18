use crate::{
	patterns::{
		checker_pattern::CheckerPattern, gradient_pattern::GradientPattern,
		ring_pattern::RingPattern, stripe_pattern::StripePattern, test_pattern::TestPattern,
	},
	primitives::{color::Color, matrix::matrix4d::Matrix4D, point::Point},
	shapes::shape::ConcreteShape,
};
use core::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ColorPattern {
	TestPattern(TestPattern),
	StripePattern(StripePattern),
	GradientPattern(GradientPattern),
	RingPattern(RingPattern),
	CheckerPattern(CheckerPattern),
}

impl ColorPattern {
	pub fn new_test() -> Self {
		Self::TestPattern(TestPattern::new())
	}

	pub fn new_stripe(a: Color, b: Color) -> Self {
		Self::StripePattern(StripePattern::new(a, b))
	}

	pub fn new_gradient(a: Color, b: Color) -> Self {
		Self::GradientPattern(GradientPattern::new(a, b))
	}

	pub fn new_ring(a: Color, b: Color) -> Self {
		Self::RingPattern(RingPattern::new(a, b))
	}

	pub fn new_checker(a: Color, b: Color) -> Self {
		Self::CheckerPattern(CheckerPattern::new(a, b))
	}

	pub fn pattern_at_object<'a>(&self, object: &'a dyn ConcreteShape, point: &Point) -> Color {
		let obj_point =
			object.transform().inverse().expect("Could not invert object transform") * (*point);
		let pattern_point =
			self.transform().inverse().expect("Could not invert pattern transform") * obj_point;

		self.pattern_at(&pattern_point)
	}
}

impl Pattern for ColorPattern {
	fn transform(&self) -> &Matrix4D {
		match self {
			Self::TestPattern(p) => p.transform(),
			Self::StripePattern(p) => p.transform(),
			Self::GradientPattern(p) => p.transform(),
			Self::RingPattern(p) => p.transform(),
			Self::CheckerPattern(p) => p.transform(),
		}
	}

	fn get_transform(&mut self) -> &mut Matrix4D {
		match self {
			Self::TestPattern(p) => p.get_transform(),
			Self::StripePattern(p) => p.get_transform(),
			Self::GradientPattern(p) => p.get_transform(),
			Self::RingPattern(p) => p.get_transform(),
			Self::CheckerPattern(p) => p.get_transform(),
		}
	}

	fn set_transform(&mut self, transform: Matrix4D) {
		match self {
			Self::TestPattern(p) => p.set_transform(transform),
			Self::StripePattern(p) => p.set_transform(transform),
			Self::GradientPattern(p) => p.set_transform(transform),
			Self::RingPattern(p) => p.set_transform(transform),
			Self::CheckerPattern(p) => p.set_transform(transform),
		}
	}

	fn pattern_at(&self, point: &Point) -> Color {
		match self {
			Self::TestPattern(p) => p.pattern_at(point),
			Self::StripePattern(p) => p.pattern_at(point),
			Self::GradientPattern(p) => p.pattern_at(point),
			Self::RingPattern(p) => p.pattern_at(point),
			Self::CheckerPattern(p) => p.pattern_at(point),
		}
	}
}

pub trait Pattern {
	fn set_transform(&mut self, transform: Matrix4D);
	fn transform(&self) -> &Matrix4D;
	fn get_transform(&mut self) -> &mut Matrix4D;
	fn pattern_at(&self, point: &Point) -> Color;
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::shapes::{shape::ConcreteShape, spheres::Sphere};

	#[test]
	fn test_stripe_pattern() {
		let white = Color::new(0.0, 0.0, 0.0);
		let black = Color::new(1.0, 1.0, 1.0);

		let pattern = ColorPattern::new_stripe(white, black);

		// constant in y
		assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), white);
		assert_eq!(pattern.pattern_at(&Point::new(0.0, 1.0, 0.0)), white);
		assert_eq!(pattern.pattern_at(&Point::new(0.0, 2.0, 0.0)), white);

		// constant in z
		assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), white);
		assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 1.0)), white);
		assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 2.0)), white);

		// alternates in x
		assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), white);
		assert_eq!(pattern.pattern_at(&Point::new(0.9, 0.0, 0.0)), white);
		assert_eq!(pattern.pattern_at(&Point::new(1.0, 0.0, 0.0)), black);
		assert_eq!(pattern.pattern_at(&Point::new(-0.1, 0.0, 0.0)), black);
		assert_eq!(pattern.pattern_at(&Point::new(-1.0, 0.0, 0.0)), black);
		assert_eq!(pattern.pattern_at(&Point::new(-1.1, 0.0, 0.0)), white);
	}

	#[test]
	fn test_object_transformation() {
		use crate::primitives::transformations::*;

		let mut s = Sphere::default();
		s.set_transform(scaling(2.0, 2.0, 2.0));
		let p = ColorPattern::new_test();
		let c = p.pattern_at_object(&s, &Point::new(2.0, 3.0, 4.0));
		assert_eq!(c, Color::new(1.0, 1.5, 2.0));

		let s = Sphere::default();
		let mut p = ColorPattern::new_test();
		p.set_transform(scaling(2.0, 2.0, 2.0));
		let c = p.pattern_at_object(&s, &Point::new(2.0, 3.0, 4.0));
		assert_eq!(c, Color::new(1.0, 1.5, 2.0));

		let mut s = Sphere::default();
		s.set_transform(scaling(2.0, 2.0, 2.0));
		let mut p = ColorPattern::new_test();
		p.set_transform(translation(0.5, 1.0, 1.5));
		let c = p.pattern_at_object(&s, &Point::new(2.5, 3.0, 3.5));
		assert_eq!(c, Color::new(0.75, 0.5, 0.25));
	}

	#[test]
	fn test_gradient_pattern() {
		let white = Color::new(0.0, 0.0, 0.0);
		let black = Color::new(1.0, 1.0, 1.0);

		let p = ColorPattern::new_gradient(white, black);
		assert_eq!(p.pattern_at(&Point::new(0.0, 0.0, 0.0)), white);
		assert_eq!(p.pattern_at(&Point::new(0.25, 0.0, 0.0)), Color::new(0.25, 0.25, 0.25));
		assert_eq!(p.pattern_at(&Point::new(0.5, 0.0, 0.0)), Color::new(0.5, 0.5, 0.5));
		assert_eq!(p.pattern_at(&Point::new(0.75, 0.0, 0.0)), Color::new(0.75, 0.75, 0.75));
		// in the book:
		// assert_eq!(p.pattern_at(&Point::new(0.25, 0.0, 0.0)), Color::new(0.75, 0.75, 0.75));
		// assert_eq!(p.pattern_at(&Point::new(0.5, 0.0, 0.0)), Color::new(0.5, 0.5, 0.5));
		// assert_eq!(p.pattern_at(&Point::new(0.75, 0.0, 0.0)), Color::new(0.25, 0.25, 0.25));
	}

	#[test]
	fn test_ring_pattern() {
		let white = Color::new(0.0, 0.0, 0.0);
		let black = Color::new(1.0, 1.0, 1.0);

		let p = ColorPattern::new_ring(white, black);
		assert_eq!(p.pattern_at(&Point::new(0.0, 0.0, 0.0)), white);
		assert_eq!(p.pattern_at(&Point::new(1.0, 0.0, 0.0)), black);
		assert_eq!(p.pattern_at(&Point::new(0.0, 0.0, 1.0)), black);
		assert_eq!(p.pattern_at(&Point::new(0.708, 0.0, 0.708)), black);
	}

	#[test]
	fn test_checker_pattern() {
		let white = Color::new(0.0, 0.0, 0.0);
		let black = Color::new(1.0, 1.0, 1.0);

		let p = ColorPattern::new_checker(white, black);
		// repeat in x
		assert_eq!(p.pattern_at(&Point::new(0.0, 0.0, 0.0)), white);
		assert_eq!(p.pattern_at(&Point::new(0.99, 0.0, 0.0)), white);
		assert_eq!(p.pattern_at(&Point::new(1.01, 0.0, 0.0)), black);

		// repeat in y
		assert_eq!(p.pattern_at(&Point::new(0.0, 0.0, 0.0)), white);
		assert_eq!(p.pattern_at(&Point::new(0.0, 0.99, 0.0)), white);
		assert_eq!(p.pattern_at(&Point::new(0.0, 1.01, 0.0)), black);

		// repeat in z
		assert_eq!(p.pattern_at(&Point::new(0.0, 0.0, 0.0)), white);
		assert_eq!(p.pattern_at(&Point::new(0.0, 0.0, 0.99)), white);
		assert_eq!(p.pattern_at(&Point::new(0.0, 0.0, 1.01)), black);
	}
}
