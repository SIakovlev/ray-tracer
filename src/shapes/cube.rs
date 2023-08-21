use std::f64;

use crate::{
	intersection::Intersection,
	primitives::{point::Point, ray::Ray, vector::Vector},
	shapes::shape::{ConcreteShape, Shape},
};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Cube {
	shape: Shape,
}

impl Cube {
	pub fn new(origin: Point) -> Self {
		Self { shape: Shape::new(origin) }
	}

	fn check_axis(&self, origin: f64, direction: f64) -> (f64, f64) {
		let t_min_num = -1.0 - origin;
		let t_max_num = 1.0 - origin;
		let (mut t_min, mut t_max) = if direction.abs() >= f64::EPSILON {
			(t_min_num / direction, t_max_num / direction)
		} else {
			(t_min_num.signum() * f64::MAX, t_max_num.signum() * f64::MAX)
		};
		if t_min > t_max {
			(t_min, t_max) = (t_max, t_min);
		}
		(t_min, t_max)
	}
}

impl ConcreteShape for Cube {
	#[allow(unused_variables)]
	fn local_normal_at(&self, point: Point) -> Vector {
		Vector::new(0.0, 1.0, 0.0)
	}

	fn local_intersect<'i>(&'i self, ray: Ray) -> Result<Vec<Intersection<'i>>, String> {
		let (xtmin, xtmax) = self.check_axis(ray.origin.tuple.x, ray.direction.tuple.x);
		let (ytmin, ytmax) = self.check_axis(ray.origin.tuple.y, ray.direction.tuple.y);
		let (ztmin, ztmax) = self.check_axis(ray.origin.tuple.z, ray.direction.tuple.z);

		let tmin = xtmin.max(ytmin).max(ztmin);
		let tmax = xtmax.min(ytmax).min(ztmax);

		if tmin > tmax {
			// ray misses the cube
			Ok(vec![])
		} else {
			Ok(vec![Intersection::new(tmin, self), Intersection::new(tmax, self)])
		}
	}

	fn shape(&self) -> &Shape {
		&self.shape
	}

	fn get_shape(&mut self) -> &mut Shape {
		&mut self.shape
	}
}

impl Default for Cube {
	fn default() -> Self {
		Self { shape: Shape::new(Point::new(0.0, 0.0, 0.0)) }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::shapes::shape::ConcreteShape;

	#[test]
	fn test_normal_at() {}

	#[test]
	fn test_intersections() {
		let c = Cube::default();

		// +x
		let r = Ray::new(Point::new(5.0, 0.5, 0.0), Vector::new(-1.0, 0.0, 0.0));
		let xs = c.local_intersect(r).unwrap();
		let (t1, t2) = (4.0, 6.0);
		assert_eq!(xs.len(), 2);
		assert_eq!(xs[0].t, t1);
		assert_eq!(xs[1].t, t2);

		// -x
		let r = Ray::new(Point::new(-5.0, 0.5, 0.0), Vector::new(1.0, 0.0, 0.0));
		let xs = c.local_intersect(r).unwrap();
		let (t1, t2) = (4.0, 6.0);
		assert_eq!(xs.len(), 2);
		assert_eq!(xs[0].t, t1);
		assert_eq!(xs[1].t, t2);

		// +y
		let r = Ray::new(Point::new(0.5, 5.0, 0.0), Vector::new(0.0, -1.0, 0.0));
		let xs = c.local_intersect(r).unwrap();
		let (t1, t2) = (4.0, 6.0);
		assert_eq!(xs.len(), 2);
		assert_eq!(xs[0].t, t1);
		assert_eq!(xs[1].t, t2);

		// -y
		let r = Ray::new(Point::new(0.5, -5.0, 0.0), Vector::new(0.0, 1.0, 0.0));
		let xs = c.local_intersect(r).unwrap();
		let (t1, t2) = (4.0, 6.0);
		assert_eq!(xs.len(), 2);
		assert_eq!(xs[0].t, t1);
		assert_eq!(xs[1].t, t2);

		// +z
		let r = Ray::new(Point::new(0.5, 0.0, 5.0), Vector::new(0.0, 0.0, -1.0));
		let xs = c.local_intersect(r).unwrap();
		let (t1, t2) = (4.0, 6.0);
		assert_eq!(xs.len(), 2);
		assert_eq!(xs[0].t, t1);
		assert_eq!(xs[1].t, t2);

		// -z
		let r = Ray::new(Point::new(0.5, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
		let xs = c.local_intersect(r).unwrap();
		let (t1, t2) = (4.0, 6.0);
		assert_eq!(xs.len(), 2);
		assert_eq!(xs[0].t, t1);
		assert_eq!(xs[1].t, t2);

		// inside
		let r = Ray::new(Point::new(0.0, 0.5, 0.0), Vector::new(0.0, 0.0, 1.0));
		let xs = c.local_intersect(r).unwrap();
		let (t1, t2) = (-1.0, 1.0);
		assert_eq!(xs.len(), 2);
		assert_eq!(xs[0].t, t1);
		assert_eq!(xs[1].t, t2);
	}
}
