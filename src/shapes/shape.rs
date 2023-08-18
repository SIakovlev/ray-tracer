use crate::{
	intersection::Intersection,
	matrix::matrix4d::Matrix4D,
	primitives::{point::Point, ray::Ray, vector::Vector},
	visualisation::materials::Material,
};
use core::fmt::Debug;

pub trait ConcreteShape {
	fn intersects<'a, 'b>(&'a self, r: &'b Ray) -> Result<Vec<Intersection<'a>>, String> {
		let local_ray =
			r.transform(self.transform().inverse().expect("Cannot apply object transformation"));
		self.local_intersect(local_ray)
	}
	fn local_intersect<'a, 'b>(&'a self, ray: Ray) -> Result<Vec<Intersection<'a>>, String>;

	fn normal_at(&self, point: Point) -> Vector {
		let local_point = self.transform().inverse().unwrap() * point;
		let local_normal = self.local_normal_at(local_point);
		let mut world_normal = self.transform().inverse().unwrap().transpose() * local_normal;
		world_normal.tuple.w = 0.0;
		world_normal.normalise()
	}
	fn local_normal_at(&self, point: Point) -> Vector;

	fn transform(&self) -> &Matrix4D {
		&self.shape().transform
	}

	fn material(&self) -> &Material {
		&self.shape().material
	}

	fn origin(&self) -> &Point {
		&self.shape().origin
	}

	fn set_transform(&mut self, transform: Matrix4D) {
		self.get_shape().transform = transform;
	}

	fn set_material(&mut self, material: Material) {
		self.get_shape().material = material
	}

	fn get_material(&mut self) -> &mut Material {
		&mut self.get_shape().material
	}

	fn set_origin(&mut self, origin: Point) {
		self.get_shape().origin = origin
	}

	fn get_shape(&mut self) -> &mut Shape;
	fn shape(&self) -> &Shape;
}

impl<'a> Debug for dyn ConcreteShape + 'a {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Shape. Origin: {:?}, Material: {:?}, Transform: {:?}",
			self.origin(),
			self.material(),
			self.transform()
		)
	}
}

impl<'a> PartialEq for dyn ConcreteShape + 'a {
	fn eq(&self, other: &Self) -> bool {
		self.transform() == other.transform() &&
			self.origin() == other.origin() &&
			self.material() == other.material()
	}
}

impl<'a> PartialOrd for dyn ConcreteShape + 'a {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.shape().partial_cmp(other.shape())
	}
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Shape {
	pub origin: Point,
	pub transform: Matrix4D,
	pub material: Material,
}

impl Shape {
	pub fn new(origin: Point) -> Self {
		Self { origin, transform: Matrix4D::identity(), material: Material::default() }
	}
}

impl Default for Shape {
	fn default() -> Self {
		Self {
			origin: Point::new(0.0, 0.0, 0.0),
			transform: Matrix4D::identity(),
			material: Material::default(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		matrix::matrix4d::Matrix4D,
		primitives::{color::Color, transformations::*},
	};

	#[test]
	fn basic_attributes() {
		let mut s = Shape::default();
		// default transform
		assert_eq!(s.transform, Matrix4D::identity());

		// assigning a transform
		s.transform = translation(2.0, 3.0, 4.0);
		assert_eq!(&s.transform, &translation(2.0, 3.0, 4.0));

		// default material
		assert_eq!(&s.material, &Material::default());

		// assigning a color
		s.material.color = Color::new(0.2, 0.4, 0.3);
		assert_eq!(s.material.color.red, 0.2);
		assert_eq!(s.material.color.green, 0.4);
		assert_eq!(s.material.color.blue, 0.3);

		let mut m = Material::default();
		m.ambient = 1.0;

		s.material = m;
		assert_eq!(s.material, m);
	}
}
