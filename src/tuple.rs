use approx::{AbsDiffEq, RelativeEq};
use std::{
	cmp::PartialEq,
	f64,
	ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Tuple {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub w: f64,
}

impl Tuple {
	pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
		Tuple { x, y, z, w }
	}

	pub fn from_array(arr: [f64; 4]) -> Self {
		Tuple { x: arr[0], y: arr[1], z: arr[2], w: arr[3] }
	}

	pub fn dot(&self, rhs: Tuple) -> f64 {
		self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
	}

	#[allow(non_snake_case)]
	pub fn cross_3D(&self, rhs: Tuple) -> Self {
		Self {
			x: self.y * rhs.z - self.z * rhs.y,
			y: self.z * rhs.x - self.x * rhs.z,
			z: self.x * rhs.y - self.y * rhs.x,
			w: self.w,
		}
	}

	pub fn abs(&self) -> f64 {
		(self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
	}
}

impl Add for Tuple {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
	}
}

impl Sub for Tuple {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
	}
}

impl Neg for Tuple {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
	}
}

impl Mul<f64> for Tuple {
	type Output = Self;

	fn mul(self, rhs: f64) -> Self::Output {
		Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs }
	}
}

impl Div<f64> for Tuple {
	type Output = Self;

	fn div(self, rhs: f64) -> Self::Output {
		Self { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs, w: self.w / rhs }
	}
}

impl AbsDiffEq for Tuple {
	type Epsilon = f64;

	fn default_epsilon() -> Self::Epsilon {
		f64::default_epsilon()
	}

	fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
		f64::abs_diff_eq(&self.x, &other.x, epsilon) &&
			f64::abs_diff_eq(&self.y, &other.z, epsilon) &&
			f64::abs_diff_eq(&self.z, &other.z, epsilon) &&
			f64::abs_diff_eq(&self.w, &other.w, epsilon)
	}
}

impl RelativeEq for Tuple {
	fn default_max_relative() -> f64 {
		f64::default_max_relative()
	}

	fn relative_eq(&self, other: &Self, epsilon: f64, max_relative: f64) -> bool {
		f64::relative_eq(&self.x, &other.x, epsilon, max_relative) &&
			f64::relative_eq(&self.y, &other.y, epsilon, max_relative) &&
			f64::relative_eq(&self.z, &other.z, epsilon, max_relative) &&
			f64::relative_eq(&self.w, &other.w, epsilon, max_relative)
	}
}

pub struct TupleIntoIterator {
	tuple: Tuple,
	index: usize,
}

impl IntoIterator for Tuple {
	type Item = f64;
	type IntoIter = TupleIntoIterator;

	fn into_iter(self) -> Self::IntoIter {
		TupleIntoIterator { tuple: self, index: 0 }
	}
}

impl Iterator for TupleIntoIterator {
	type Item = f64;
	fn next(&mut self) -> Option<f64> {
		let result = match self.index {
			0 => self.tuple.x,
			1 => self.tuple.y,
			2 => self.tuple.z,
			3 => self.tuple.w,
			_ => return None,
		};
		self.index += 1;
		Some(result)
	}
}

#[cfg(test)]
mod tests {
	use crate::tuple::Tuple;

	#[test]
	fn addition() {
		let t1 = Tuple { x: 3.0, y: -2.0, z: 5.2, w: 1.0 };
		let t2 = Tuple { x: -2.0, y: 3.0, z: 1.1, w: 0.0 };

		let t = Tuple { x: 1.0, y: 1.0, z: 6.3, w: 1.0 };

		approx::assert_relative_eq!(&(t2 + t1), &t);
	}

	#[test]
	fn subtraction() {
		let t1 = Tuple { x: 1.0, y: 2.0, z: 3.0, w: 1.0 };
		let t2 = Tuple { x: 8.0, y: 3.0, z: 5.0, w: 4.0 };

		let t = Tuple { x: 7.0, y: 1.0, z: 2.0, w: 3.0 };

		approx::assert_relative_eq!(&(t2 - t1), &t)
	}

	#[test]
	fn negation() {
		let t1 = Tuple { x: 1.0, y: 2.0, z: 3.0, w: 1.0 };
		let t2 = Tuple { x: -1.0, y: -2.0, z: -3.0, w: -1.0 };

		approx::assert_relative_eq!(&-t1, &t2)
	}

	#[test]
	fn iteration() {
		let t1 = Tuple { x: 1.0, y: 2.0, z: 3.0, w: 1.0 };
		for elem in t1.into_iter() {
			println!("{}", &elem)
		}
	}
}
