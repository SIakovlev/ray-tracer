use std::f64;

use crate::{
	intersection::Intersection,
	primitives::{point::Point, ray::Ray, vector::Vector},
	shapes::shape::{ConcreteShape, Shape},
};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Plane {
	shape: Shape,
}

impl Plane {
	pub fn new(origin: Point) -> Self {
		Self { shape: Shape::new(origin) }
	}
}

impl ConcreteShape for Plane {
	#[allow(unused_variables)]
	fn local_normal_at(&self, point: Point) -> Vector {
		Vector::new(0.0, 1.0, 0.0)
	}

	fn local_intersect<'i>(&'i self, ray: Ray) -> Result<Vec<Intersection<'i>>, String> {
		let mut is = Vec::new();
		if ray.direction.tuple.y.abs() < f64::EPSILON {
			Ok(is)
		} else {
			let t = -ray.origin.tuple.y / ray.direction.tuple.y;
			is.push(Intersection { t, object: self });
			Ok(is)
		}
	}

	fn shape(&self) -> &Shape {
		&self.shape
	}

	fn get_shape(&mut self) -> &mut Shape {
		&mut self.shape
	}
}

impl Default for Plane {
	fn default() -> Self {
		Self { shape: Shape::new(Point::new(0.0, 0.0, 0.0)) }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::shapes::shape::ConcreteShape;

	#[test]
	fn test_normal_at() {
		let p = Plane::default();

		let n1 = p.normal_at(Point::new(0.0, 0.0, 0.0));
		let n2 = p.normal_at(Point::new(10.0, 0.0, -10.0));
		let n3 = p.normal_at(Point::new(-5.0, 0.0, 150.0));

		let n = Vector::new(0.0, 1.0, 0.0);
		assert_eq!(n1, n);
		assert_eq!(n2, n);
		assert_eq!(n3, n);
	}

	#[test]
	fn test_intersections() {
		// ray is parallel to the plane
		let p = Plane::default();
		let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
		let xs = p.local_intersect(r).unwrap();

		assert_eq!(xs.len(), 0);

		// ray is coplanar to the plane
		let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
		let xs = p.local_intersect(r).unwrap();
		assert_eq!(xs.len(), 0);

		// ray intersecting plane from above and below
		let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
		let xs = p.local_intersect(r).unwrap();
		assert_eq!(xs.len(), 1);
		assert_eq!(xs[0].t, 1.0);
		assert_eq!(xs[0].object, &p as &dyn ConcreteShape);

		let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
		let xs = p.local_intersect(r).unwrap();
		assert_eq!(xs.len(), 1);
		assert_eq!(xs[0].t, 1.0);
		assert_eq!(xs[0].object, &p as &dyn ConcreteShape);
	}
}
