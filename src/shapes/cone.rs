use std::f64;

use crate::{
	intersection::Intersection,
	primitives::{point::Point, ray::Ray, vector::Vector},
	shapes::shape::{ConcreteShape, Shape},
};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Cone {
	shape: Shape,
	maximum: f64,
	minimum: f64,
	closed: bool,
}

impl Cone {
	pub fn new(origin: Point, maximum: f64, minimum: f64, closed: bool) -> Self {
		Self { shape: Shape::new(origin), maximum, minimum, closed }
	}

	fn check_cap(&self, ray: &Ray, t: f64, y: f64) -> bool {
		let x = ray.origin.tuple.x + t * ray.direction.tuple.x;
		let z = ray.origin.tuple.z + t * ray.direction.tuple.z;
		(x.powi(2) + z.powi(2)) <= y.abs()
	}
}

impl ConcreteShape for Cone {
	#[allow(unused_variables)]
	fn local_normal_at(&self, point: Point) -> Vector {
		let d = point.tuple.x.powi(2) + point.tuple.z.powi(2);

		if d < 1.0 && point.tuple.y >= self.maximum - f64::EPSILON {
			Vector::new(0.0, 1.0, 0.0)
		} else if d < 1.0 && point.tuple.y <= self.minimum + f64::EPSILON {
			Vector::new(0.0, -1.0, 0.0)
		} else {
			let mut y = d.sqrt();
			if point.tuple.y >= f64::EPSILON {
				y = -y;
			}
			Vector::new(point.tuple.x, y, point.tuple.z).normalise()
		}
	}

	fn local_intersect<'i>(&'i self, ray: Ray) -> Result<Vec<Intersection<'i>>, String> {
		let mut xs = vec![];

		let a = ray.direction.tuple.x.powi(2) - ray.direction.tuple.y.powi(2) +
			ray.direction.tuple.z.powi(2);
		let b = 2.0 * ray.origin.tuple.x * ray.direction.tuple.x -
			2.0 * ray.origin.tuple.y * ray.direction.tuple.y +
			2.0 * ray.origin.tuple.z * ray.direction.tuple.z;
		let c =
			ray.origin.tuple.x.powi(2) - ray.origin.tuple.y.powi(2) + ray.origin.tuple.z.powi(2);
		if a.abs() >= f64::EPSILON {
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
		} else if b.abs() >= f64::EPSILON {
			xs.push(Intersection::new(-c / (2.0 * b), self))
		}

		if self.closed && ray.direction.tuple.y.abs() >= f64::EPSILON {
			let t = (self.minimum - ray.origin.tuple.y) / ray.direction.tuple.y;
			if self.check_cap(&ray, t, self.minimum) {
				xs.push(Intersection::new(t, self));
			}
			let t = (self.maximum - ray.origin.tuple.y) / ray.direction.tuple.y;
			if self.check_cap(&ray, t, self.maximum) {
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

impl Default for Cone {
	fn default() -> Self {
		Self {
			shape: Shape::new(Point::new(0.0, 0.0, 0.0)),
			maximum: f64::MAX,
			minimum: f64::MIN,
			closed: true,
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
		let c = Cone::default();
		for (p, n) in vec![
			(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)),
			(Point::new(1.0, 1.0, 1.0), Vector::new(1.0, -2.0_f64.sqrt(), 1.0).normalise()),
			(Point::new(-1.0, -1.0, 0.0), Vector::new(-1.0, 1.0, 0.0).normalise()),
		] {
			assert_eq!(c.local_normal_at(p), n);
		}
	}

	#[test]
	fn test_intersections() {
		let c = Cone::default();
		// basic hits
		for (o, d, t1, t2) in vec![
			(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), 5.0, 5.0),
			(Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0), 8.66025, 8.66025),
			(Point::new(1.0, 1.0, -5.0), Vector::new(-0.5, -1.0, 1.0), 4.55006, 49.44994),
		] {
			let r = Ray::new(o, d.normalise());
			let xs = c.local_intersect(r).unwrap();
			assert_eq!(xs.len(), 2);
			approx::assert_relative_eq!(xs[0].t, t1, epsilon = 1e-4);
			approx::assert_relative_eq!(xs[1].t, t2, epsilon = 1e-4);
		}
		// parallel ray
		for (o, d, t) in vec![(Point::new(0.0, 0.0, -1.0), Vector::new(0.0, 1.0, 1.0), 0.35355)] {
			let r = Ray::new(o, d.normalise());
			let xs = c.local_intersect(r).unwrap();
			assert_eq!(xs.len(), 1);
			approx::assert_relative_eq!(xs[0].t, t, epsilon = 1e-4);
		}

		// cone's end caps
		let c = Cone::new(Point::new(0.0, 0.0, 0.0), 0.5, -0.5, true);
		for (o, d, count) in vec![
			(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0), 0),
			(Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 2),
			(Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 0.0), 4),
		] {
			let r = Ray::new(o, d.normalise());
			let xs = c.local_intersect(r).unwrap();
			assert_eq!(xs.len(), count);
		}
	}
}
