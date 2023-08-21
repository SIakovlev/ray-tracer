use std::f64;

use crate::{
	intersection::Intersection,
	primitives::{point::Point, ray::Ray, vector::Vector},
	shapes::shape::{ConcreteShape, Shape},
};

use approx::RelativeEq;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Cylinder {
	shape: Shape,
	maximum: f64,
	minimum: f64,
	closed: bool,
}

impl Cylinder {
	pub fn new(origin: Point, maximum: f64, minimum: f64, closed: bool) -> Self {
		Self { shape: Shape::new(origin), maximum, minimum, closed }
	}

	fn check_cap(&self, ray: &Ray, t: f64) -> bool {
		let x = ray.origin.tuple.x + t * ray.direction.tuple.x;
		let z = ray.origin.tuple.z + t * ray.direction.tuple.z;
		(x.powi(2) + z.powi(2)) <= 1.0
	}
}

impl ConcreteShape for Cylinder {
	#[allow(unused_variables)]
	fn local_normal_at(&self, point: Point) -> Vector {
		let d = point.tuple.x.powi(2) + point.tuple.z.powi(2);

		if d < 1.0 && point.tuple.y >= self.maximum - f64::EPSILON {
			Vector::new(0.0, 1.0, 0.0)
		} else if d < 1.0 && point.tuple.y <= self.minimum + f64::EPSILON {
			Vector::new(0.0, -1.0, 0.0)
		} else {
			Vector::new(point.tuple.x, 0.0, point.tuple.z).normalise()
		}
	}

	fn local_intersect<'i>(&'i self, ray: Ray) -> Result<Vec<Intersection<'i>>, String> {
		let mut xs = vec![];

		let a = ray.direction.tuple.x.powi(2) + ray.direction.tuple.z.powi(2);
		if a.abs() >= f64::EPSILON {
			let b = 2.0 * ray.origin.tuple.x * ray.direction.tuple.x +
				2.0 * ray.origin.tuple.z * ray.direction.tuple.z;
			let c = ray.origin.tuple.x.powi(2) + ray.origin.tuple.z.powi(2) - 1.0;
			let discriminant = b.powi(2) - 4.0 * a * c;
			if discriminant < 0.0 {
				return Ok(vec![])
			}
			let mut t0 = (-b - discriminant.sqrt()) / (2.0 * a);
			let mut t1 = (-b + discriminant.sqrt()) / (2.0 * a);
			if t0 > t1 {
				(t0, t1) = (t1, t0);
			}

			let y0 = ray.origin.tuple.y + t0 * ray.direction.tuple.y;
			if self.minimum < y0 && y0 < self.maximum {
				xs.push(Intersection::new(t0, self))
			}

			let y1 = ray.origin.tuple.y + t1 * ray.direction.tuple.y;
			if self.minimum < y1 && y1 < self.maximum {
				xs.push(Intersection::new(t1, self))
			}
		}

		if self.closed && ray.direction.tuple.y.abs() >= f64::EPSILON {
			let t = (self.minimum - ray.origin.tuple.y) / ray.direction.tuple.y;
			if self.check_cap(&ray, t) {
				xs.push(Intersection::new(t, self));
			}
			let t = (self.maximum - ray.origin.tuple.y) / ray.direction.tuple.y;
			if self.check_cap(&ray, t) {
				xs.push(Intersection::new(t, self));
			}
		}

		Ok(xs)
	}

	fn shape(&self) -> &Shape {
		&self.shape
	}

	fn get_shape(&mut self) -> &mut Shape {
		&mut self.shape
	}
}

impl Default for Cylinder {
	fn default() -> Self {
		Self {
			shape: Shape::new(Point::new(0.0, 0.0, 0.0)),
			maximum: f64::MAX,
			minimum: f64::MIN,
			closed: false,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::shapes::shape::ConcreteShape;

	#[test]
	fn test_normal_at() {
		// Basic normal
		let c = Cylinder::default();
		for (p, n) in vec![
			(Point::new(1.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0)),
			(Point::new(0.0, 5.0, -1.0), Vector::new(0.0, 0.0, -1.0)),
			(Point::new(0.0, -2.0, 1.0), Vector::new(0.0, 0.0, 1.0)),
			(Point::new(-1.0, 1.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
		] {
			assert_eq!(c.local_normal_at(p), n);
		}

		// Normal at the end caps
		let c = Cylinder::new(Point::new(0.0, 0.0, 0.0), 2.0, 1.0, true);
		for (p, n) in vec![
			(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
			(Point::new(0.5, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
			(Point::new(0.0, 1.0, 0.5), Vector::new(0.0, -1.0, 0.0)),
			(Point::new(0.0, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
			(Point::new(0.5, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
			(Point::new(0.0, 2.0, 0.5), Vector::new(0.0, 1.0, 0.0)),
		] {
			assert_eq!(c.local_normal_at(p), n);
		}
	}

	#[test]
	fn test_intersections() {
		// miss
		let c = Cylinder::default();
		for (o, d) in vec![
			(Point::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
			(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
			(Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0)),
		] {
			let r = Ray::new(o, d.normalise());
			let xs = c.local_intersect(r).unwrap();
			assert_eq!(xs.len(), 0);
		}

		// basic hits
		for (o, d, t1, t2) in vec![
			(Point::new(1.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), 5.0, 5.0),
			(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), 4.0, 6.0),
			(Point::new(0.5, 0.0, -5.0), Vector::new(0.1, 1.0, 1.0), 6.80798, 7.08872),
		] {
			let r = Ray::new(o, d.normalise());
			let xs = c.local_intersect(r).unwrap();
			assert_eq!(xs.len(), 2);
			approx::assert_relative_eq!(xs[0].t, t1, epsilon = 1e-4);
			approx::assert_relative_eq!(xs[1].t, t2, epsilon = 1e-4);
		}

		// truncated cylinder
		let c = Cylinder::new(Point::new(0.0, 0.0, 0.0), 2.0, 1.0, false);
		for (o, d, count) in vec![
			(Point::new(0.0, 1.5, 0.0), Vector::new(0.1, 1.0, 0.0), 0),
			(Point::new(0.0, 3.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
			(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
			(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
			(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
			(Point::new(0.0, 1.5, -2.0), Vector::new(0.0, 0.0, 1.0), 2),
		] {
			let r = Ray::new(o, d.normalise());
			let xs = c.local_intersect(r).unwrap();
			assert_eq!(xs.len(), count);
		}

		// truncated cylinder
		let c = Cylinder::new(Point::new(0.0, 0.0, 0.0), 2.0, 1.0, false);
		for (o, d, count) in vec![
			(Point::new(0.0, 1.5, 0.0), Vector::new(0.1, 1.0, 0.0), 0),
			(Point::new(0.0, 3.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
			(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
			(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
			(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
			(Point::new(0.0, 1.5, -2.0), Vector::new(0.0, 0.0, 1.0), 2),
		] {
			let r = Ray::new(o, d.normalise());
			let xs = c.local_intersect(r).unwrap();
			assert_eq!(xs.len(), count);
		}

		// closed cylinder
		let c = Cylinder::new(Point::new(0.0, 0.0, 0.0), 2.0, 1.0, true);
		for (o, d, count) in vec![
			(Point::new(0.0, 3.0, 0.0), Vector::new(0.0, -1.0, 0.0), 2),
			(Point::new(0.0, 3.0, -2.0), Vector::new(0.0, -1.0, 2.0), 2),
			(Point::new(0.0, 4.0, -2.0), Vector::new(0.0, -1.0, 1.0), 2),
			(Point::new(0.0, 0.0, -2.0), Vector::new(0.0, 1.0, 2.0), 2),
			(Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 1.0), 2),
		] {
			let r = Ray::new(o, d.normalise());
			let xs = c.local_intersect(r).unwrap();
			assert_eq!(xs.len(), count);
		}
	}
}
