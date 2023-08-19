use crate::{
	intersection::{Intersection, IntersectionComputations},
	primitives::{matrix::matrix4d::Matrix4D, point::Point, vector::Vector},
	shapes::shape::ConcreteShape,
	visualisation::world::World,
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
		xs: Option<&'a Vec<Intersection<'a>>>,
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

		let mut n1 = 1.0;
		let mut n2 = 1.0;

		match xs {
			Some(intersections) => {
				let mut container: Vec<&dyn ConcreteShape> = vec![];
				for i in intersections {
					if intersection == i {
						if container.len() == 0 {
							n1 = 1.0;
						} else {
							n1 = container.last().unwrap().material().refractive_index;
						}
					}

					if container.contains(&i.object) {
						let index = container.iter().position(|x| *x == i.object).unwrap();
						container.remove(index);
					} else {
						container.push(i.object);
					}

					if intersection == i {
						if container.len() == 0 {
							n2 = 1.0;
						} else {
							n2 = container.last().unwrap().material().refractive_index;
						}
						break
					}
				}
			},

			None => n2 = intersection.object.material().refractive_index,
		}

		IntersectionComputations {
			t: intersection.t,
			object: intersection.object,
			point,
			over_point: point + normal * 1e-6,
			under_point: point - normal * 1e-6,
			eye,
			normal,
			reflection_vector,
			inside,
			n1,
			n2,
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
		primitives::transformations::*,
		shapes::{plane::Plane, shape::ConcreteShape, spheres::Sphere},
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
		let comps = r.prepare_computations(&i, None);

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
		let comps = r.prepare_computations(&i, None);

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
		let comps = r.prepare_computations(&i, None);
		assert!(comps.over_point.tuple.z < -f64::EPSILON / 2.0);
		assert!(comps.point.tuple.z > comps.over_point.tuple.z);
	}

	#[test]
	fn reflection_test() {
		// hit default plane with ray under 45 deg angle and check reflected ray
		let r = Ray::new(
			Point::new(0.0, 1.0, -1.0),
			Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
		);
		let s = Plane::default();
		let i = Intersection::new(2.0_f64.sqrt(), &s);
		let comps = r.prepare_computations(&i, None);

		assert_eq!(
			comps.reflection_vector,
			Vector::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
		);
	}

	#[test]
	fn refraction_test() {
		let mut a = Sphere::new_glass_sphere();
		a.set_transform(scaling(2.0, 2.0, 2.0));
		a.get_material().refractive_index = 1.5;

		let mut b = Sphere::new_glass_sphere();
		b.set_transform(translation(0.0, 0.0, -0.25));
		b.get_material().refractive_index = 2.0;

		let mut c = Sphere::new_glass_sphere();
		c.set_transform(translation(0.0, 0.0, 0.25));
		c.get_material().refractive_index = 2.5;

		let r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::new(0.0, 0.0, 1.0));
		let xs = vec![
			Intersection::new(2.0, &a),
			Intersection::new(2.75, &b),
			Intersection::new(3.25, &c),
			Intersection::new(4.75, &b),
			Intersection::new(5.25, &c),
			Intersection::new(6.0, &a),
		];

		let comps = r.prepare_computations(&xs[0], Some(&xs));
		assert_eq!(comps.n1, 1.0);
		assert_eq!(comps.n2, 1.5);

		let comps = r.prepare_computations(&xs[1], Some(&xs));
		assert_eq!(comps.n1, 1.5);
		assert_eq!(comps.n2, 2.0);

		let comps = r.prepare_computations(&xs[2], Some(&xs));
		assert_eq!(comps.n1, 2.0);
		assert_eq!(comps.n2, 2.5);

		let comps = r.prepare_computations(&xs[3], Some(&xs));
		assert_eq!(comps.n1, 2.5);
		assert_eq!(comps.n2, 2.5);

		let comps = r.prepare_computations(&xs[4], Some(&xs));
		assert_eq!(comps.n1, 2.5);
		assert_eq!(comps.n2, 1.5);

		let comps = r.prepare_computations(&xs[5], Some(&xs));
		assert_eq!(comps.n1, 1.5);
		assert_eq!(comps.n2, 1.0);
	}
}
