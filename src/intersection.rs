use crate::{point::Point, shapes::shape::ConcreteShape, vector::Vector};

#[derive(Debug)]
pub struct IntersectionComputations<'a> {
	pub t: f64,
	pub object: &'a dyn ConcreteShape,
	pub point: Point,
	pub over_point: Point,
	pub eye: Vector,
	pub normal: Vector,
	pub reflection_vector: Vector,
	pub inside: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Intersection<'a> {
	pub t: f64,
	pub object: &'a dyn ConcreteShape,
}

impl<'a> Intersection<'a> {
	pub fn new(t: f64, obj: &'a dyn ConcreteShape) -> Self {
		Intersection { t, object: obj }
	}
}

pub fn hit<'a>(intersections: &'a mut Vec<Intersection<'a>>) -> Option<&Intersection<'a>> {
	intersections.sort_by(|i1, i2| (i1.t).partial_cmp(&i2.t).unwrap());
	intersections.iter().skip_while(|x| x.t < 0.0).next()
}

#[cfg(test)]
mod tests {
	use std::vec;

	use super::{hit, Intersection};
	use crate::{
		matrix::matrix4d::Matrix4D,
		point::Point,
		shapes::{shape::ConcreteShape, spheres::Sphere},
		transformations::*,
	};

	#[test]
	fn hit_test() {
		// basic intersection
		let s = Sphere::new(Point::new(0.0, 0.0, 0.0));
		let i1 = Intersection::new(1.0, &s);
		let i2 = Intersection::new(2.0, &s);
		let mut xs = vec![i1, i2];
		let i = hit(&mut xs);
		assert_eq!(i.unwrap(), &i1);

		// intersection where there is one point behind a ray
		let i1 = Intersection::new(-1.0, &s);
		let i2 = Intersection::new(1.0, &s);
		let mut xs = vec![i1, i2];
		let i = hit(&mut xs);
		assert_eq!(i.unwrap(), &i2);

		// no intersections
		let i1 = Intersection::new(-2.0, &s);
		let i2 = Intersection::new(-1.0, &s);
		let mut xs = vec![i1, i2];
		let i = hit(&mut xs);
		assert!(i.is_none());

		// more complex example
		let i1 = Intersection::new(5.0, &s);
		let i2 = Intersection::new(7.0, &s);
		let i3 = Intersection::new(-3.0, &s);
		let i4 = Intersection::new(2.0, &s);
		let mut xs = vec![i1, i2, i3, i4];
		let i = hit(&mut xs);
		assert_eq!(i.unwrap(), &i4);
	}

	#[test]
	fn obj_transformations() {
		let s = Sphere::new(Point::new(0.0, 0.0, 0.0));
		let t = Matrix4D::identity();
		assert_eq!(s.transform(), &t);

		let mut s = Sphere::new(Point::new(0.0, 0.0, 0.0));
		let t = translation(2.0, 3.0, 4.0);
		s.set_transform(t);
		assert_eq!(s.transform(), &t);
	}
}
