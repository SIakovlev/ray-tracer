use crate::primitives::{tuple::Tuple, vector::Vector};
use approx::{AbsDiffEq, RelativeEq};
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Point {
	pub tuple: Tuple,
}

impl Point {
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Self { tuple: Tuple::new(x, y, z, 1.0) }
	}

	pub fn from_tuple(tuple: Tuple) -> Self {
		Self { tuple }
	}
}

// Adding vector to a point
impl Add<Vector> for Point {
	type Output = Self;

	fn add(self, rhs: Vector) -> Self::Output {
		let t = self.tuple + rhs.tuple;
		Point { tuple: t }
	}
}

// Subtracting two points
impl Sub<Point> for Point {
	type Output = Vector;

	fn sub(self, rhs: Point) -> Self::Output {
		let t = self.tuple - rhs.tuple;
		Vector::from_tuple(t)
	}
}

// Subtracting vector from a point
impl Sub<Vector> for Point {
	type Output = Point;

	fn sub(self, rhs: Vector) -> Self::Output {
		let t = self.tuple - rhs.tuple;
		Point { tuple: t }
	}
}

impl AbsDiffEq for Point {
	type Epsilon = f64;

	fn default_epsilon() -> Self::Epsilon {
		f64::default_epsilon()
	}

	fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
		Tuple::abs_diff_eq(&self.tuple, &other.tuple, epsilon)
	}
}

impl RelativeEq for Point {
	fn default_max_relative() -> f64 {
		f64::default_max_relative()
	}

	fn relative_eq(&self, other: &Self, epsilon: f64, max_relative: f64) -> bool {
		Tuple::relative_eq(&self.tuple, &other.tuple, epsilon, max_relative)
	}
}

#[cfg(test)]
mod tests {
	use crate::primitives::{point::Point, tuple::Tuple, vector::Vector};

	#[test]
	fn is_tuple() {
		let p = Point::new(1.0, 2.0, 3.0);
		let t = Tuple::new(1.0, 2.0, 3.0, 1.0);

		assert_eq!(&p.tuple, &t);
	}

	#[test]
	fn subtracting_two_points() {
		let p1 = Point::new(3.0, 2.0, 1.0);
		let p2 = Point::new(5.0, 6.0, 7.0);
		let v = Vector::new(-2.0, -4.0, -6.0);

		assert_eq!(&(p1 - p2), &v)
	}

	#[test]
	fn subtracting_vector_from_point() {
		let p1 = Point::new(3.0, 2.0, 1.0);
		let v = Vector::new(5.0, 6.0, 7.0);
		let p2 = Point::new(-2.0, -4.0, -6.0);

		assert_eq!(&(p1 - v), &p2)
	}
}
