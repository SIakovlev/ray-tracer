use crate::{
	intersection::Intersection,
	primitives::{point::Point, ray::Ray, vector::Vector},
	shapes::shape::{ConcreteShape, Shape},
};

use approx::RelativeEq;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Sphere {
	shape: Shape,
}

impl Sphere {
	pub fn new(origin: Point) -> Self {
		Self { shape: Shape::new(origin) }
	}
}

impl ConcreteShape for Sphere {
	fn local_normal_at(&self, point: Point) -> Vector {
		(point - *self.origin()).normalise()
	}

	fn local_intersect<'i>(&'i self, ray: Ray) -> Result<Vec<Intersection<'i>>, String> {
		let obj_to_ray = ray.origin - self.shape.origin;

		let a = ray.direction.dot(&ray.direction);
		if a.relative_eq(&0.0, f64::EPSILON, f64::EPSILON) {
			return Err("Direction is zero or close to zero".to_string())
		}

		let b = 2.0 * ray.direction.dot(&obj_to_ray);
		let c = obj_to_ray.dot(&obj_to_ray) - 1.0;

		let mut is = Vec::new();

		let discriminant: f64 = b * b - 4.0 * a * c;
		if discriminant >= 0.0 {
			let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
			let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
			is.push(Intersection { t: t1, object: self });
			is.push(Intersection { t: t2, object: self });
		}

		Ok(is)
	}

	fn shape(&self) -> &Shape {
		&self.shape
	}

	fn get_shape(&mut self) -> &mut Shape {
		&mut self.shape
	}
}

impl Default for Sphere {
	fn default() -> Self {
		Self { shape: Shape::new(Point::new(0.0, 0.0, 0.0)) }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		primitives::{ray::Ray, transformations::*, vector::Vector},
		shapes::{shape::ConcreteShape, spheres::Sphere},
		visualisation::materials::Material,
	};
	use std::f64;

	#[test]
	fn normal_tests() {
		// basic tests
		let s = Sphere::new(Point::new(0.0, 0.0, 0.0));
		let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
		approx::assert_relative_eq!(n, Vector::new(1.0, 0.0, 0.0));

		let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
		approx::assert_relative_eq!(n, Vector::new(0.0, 1.0, 0.0));

		let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
		approx::assert_relative_eq!(n, Vector::new(0.0, 0.0, 1.0));

		let n =
			s.normal_at(Point::new(3.0f64.sqrt() / 3.0, 3.0f64.sqrt() / 3.0, 3.0f64.sqrt() / 3.0));
		approx::assert_relative_eq!(
			n,
			Vector::new(3.0f64.sqrt() / 3.0, 3.0f64.sqrt() / 3.0, 3.0f64.sqrt() / 3.0)
		);
		approx::assert_relative_eq!(n, n.normalise());

		// test translated sphere
		let mut s = Sphere::new(Point::new(0.0, 0.0, 0.0));
		s.set_transform(translation(0.0, 1.0, 0.0));
		let n = s.normal_at(Point::new(0.0, 1.7071067, -0.7071067));
		approx::assert_relative_eq!(n, Vector::new(0.0, 0.7071067, -0.7071067), epsilon = 1e-6);

		// test scaled and rotated sphere
		let mut s = Sphere::new(Point::new(0.0, 0.0, 0.0));
		s.set_transform(scaling(1.0, 0.5, 1.0) * rotation_z(f64::consts::PI / 5.0f64));
		let n = s.normal_at(Point::new(0.0, 2.0f64.sqrt() / 2.0, -2.0f64.sqrt() / 2.0));
		approx::assert_relative_eq!(n, Vector::new(0.0, 0.9701425, -0.24253562), epsilon = 1e-6);
	}

	#[test]
	fn material_test() {
		// default material
		let mut s = Sphere::default();
		let mut m = Material::default();
		assert_eq!(&s.shape.material, &m);

		// assign material
		m.ambient = 1.0;
		s.set_material(m);
		assert_eq!(s.material().ambient, 1.0);
	}

	#[test]
	fn unit_sphere_intersection() -> Result<(), String> {
		let obj = Sphere::new(Point::new(0.0, 0.0, 0.0));
		// ray intersects a unit sphere at two points
		let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
		let xs = obj.intersects(&r)?;
		// let xs = r.intersection(&obj)?;

		approx::assert_relative_eq!(xs[0].t, 4.0);
		approx::assert_relative_eq!(xs[1].t, 6.0);
		assert_eq!(xs[0].object, xs[1].object);

		// ray intersects a unit sphere at a tangent
		let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
		let xs = obj.intersects(&r)?;

		approx::assert_relative_eq!(xs[0].t, 5.0);
		approx::assert_relative_eq!(xs[1].t, 5.0);
		assert_eq!(xs[0].object, xs[1].object);

		// ray does not intersect a unit sphere
		let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
		let xs = obj.intersects(&r)?;
		assert_eq!(xs.len(), 0);

		// ray originates inside sphere
		let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
		let xs = obj.intersects(&r)?;

		approx::assert_relative_eq!(xs[0].t, -1.0);
		approx::assert_relative_eq!(xs[1].t, 1.0);
		assert_eq!(xs[0].object, xs[1].object);

		// A sphere is behind a ray
		let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
		let xs = obj.intersects(&r)?;

		approx::assert_relative_eq!(xs[0].t, -6.0);
		approx::assert_relative_eq!(xs[1].t, -4.0);
		assert_eq!(xs[0].object, xs[1].object);

		Ok(())
	}

	#[test]
	#[should_panic(expected = "Direction is zero or close to zero")]
	fn unit_sphere_intersection_failure() {
		let obj = Sphere::new(Point::new(0.0, 0.0, 0.0));
		let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 0.0));
		obj.intersects(&r).unwrap();
	}

	#[test]
	fn intersecting_scaled_sphere() -> Result<(), String> {
		let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

		let mut s = Sphere::new(Point::new(0.0, 0.0, 0.0));
		s.set_transform(scaling(2.0, 2.0, 2.0));
		let xs = s.intersects(&r)?;

		assert_eq!(xs.len(), 2);
		assert_eq!(xs[0].t, 3.0);
		assert_eq!(xs[1].t, 7.0);

		// intersection does not modify a ray
		assert_eq!(r.origin, Point::new(0.0, 0.0, -5.0));
		assert_eq!(r.direction, Vector::new(0.0, 0.0, 1.0));

		// intersection with shifted sphere
		let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

		let mut s = Sphere::new(Point::new(0.0, 0.0, 0.0));
		s.set_transform(translation(5.0, 0.0, 0.0));
		let xs = s.intersects(&r)?;
		assert_eq!(xs.len(), 0);

		Ok(())
	}
}
