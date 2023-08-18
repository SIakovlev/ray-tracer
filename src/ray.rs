use crate::{
	intersection::{Intersection, IntersectionComputations},
	matrix::matrix4d::Matrix4D,
	point::Point,
	vector::Vector,
	world::World,
};
#[derive(Debug)]
pub struct Ray {
	pub origin: Point,
	pub direction: Vector,
}

impl<'a, 'b> Ray {
	pub fn new(origin: Point, direction: Vector) -> Self {
		Ray { origin, direction }
	}

	pub fn position(&self, t: f64) -> Point {
		self.origin + self.direction * t
	}

	pub fn intersect_world(&'a self, world: &'b World) -> Result<Vec<Intersection<'b>>, String> {
		// gather all intersections into vector
		let mut result = Vec::<Intersection>::new();
		for obj in &world.objects {
			result.append(&mut obj.intersects(self)?);
		}
		// sort intersections based on t value
		result.sort_by(|i1, i2| (i1.t).partial_cmp(&i2.t).unwrap());
		Ok(result)
	}

	pub fn prepare_computations(
		&self,
		intersection: &'a Intersection,
	) -> IntersectionComputations<'a> {
		let point = self.position(intersection.t);
		let mut normal = intersection.object.normal_at(point);
		let eye = -self.direction;
		let mut inside = false;

		if normal.dot(&eye) < 0.0 {
			inside = true;
			normal = -normal;
		}

		let reflection_vector = self.direction.reflect(normal);

		IntersectionComputations {
			t: intersection.t,
			object: intersection.object,
			point,
			over_point: point + normal * 1e-6,
			eye,
			normal,
			reflection_vector,
			inside,
		}
	}

	pub fn transform(&self, transformation: Matrix4D) -> Self {
		Ray { origin: transformation * self.origin, direction: transformation * self.direction }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		shapes::{shape::ConcreteShape, spheres::Sphere},
		transformations::translation,
	};

	#[test]
	fn position_test() {
		let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

		approx::assert_relative_eq!(r.position(0.0), Point::new(2.0, 3.0, 4.0));
		approx::assert_relative_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
		approx::assert_relative_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
		approx::assert_relative_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
	}

	#[test]
	fn transform_test() {
		use crate::transformations::*;
		// translating a ray
		let r1 = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
		let t = translation(3.0, 4.0, 5.0);

		let r2 = r1.transform(t);

		assert_eq!(r2.origin, Point::new(4.0, 6.0, 8.0));
		assert_eq!(r2.direction, Vector::new(0.0, 1.0, 0.0));

		// scaling a ray
		let r1 = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
		let t = scaling(2.0, 3.0, 4.0);

		let r2 = r1.transform(t);

		assert_eq!(r2.origin, Point::new(2.0, 6.0, 12.0));
		assert_eq!(r2.direction, Vector::new(0.0, 3.0, 0.0));
	}

	#[test]
	fn prepare_computations_test() {
		// the hit, when an intersection occurs on the outside
		let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
		let s = Sphere::default();
		let i = Intersection::new(4.0, &s);
		let comps = r.prepare_computations(&i);

		assert_eq!(comps.inside, false);
		assert_eq!(comps.t, i.t);
		assert_eq!(comps.object, i.object);
		assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
		assert_eq!(comps.eye, Vector::new(0.0, 0.0, -1.0));
		assert_eq!(comps.normal, Vector::new(0.0, 0.0, -1.0));

		// the hit, when an intersection occurs on the inside
		let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
		let s = Sphere::default();
		let i = Intersection::new(1.0, &s);
		let comps = r.prepare_computations(&i);

		assert_eq!(comps.inside, true);
		assert_eq!(comps.t, i.t);
		assert_eq!(comps.object, i.object);
		assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
		assert_eq!(comps.eye, Vector::new(0.0, 0.0, -1.0));
		assert_eq!(comps.normal, Vector::new(0.0, 0.0, -1.0));

		// the hit should offset the point
		let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
		let mut s = Sphere::default();
		s.set_transform(translation(0.0, 0.0, 1.0));
		let i = Intersection::new(5.0, &s);
		let comps = r.prepare_computations(&i);
		assert!(comps.over_point.tuple.z < -f64::EPSILON / 2.0);
		assert!(comps.point.tuple.z > comps.over_point.tuple.z);
	}
}
