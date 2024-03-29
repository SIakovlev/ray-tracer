use crate::{
	intersection::{hit, IntersectionComputations},
	primitives::{color::Color, point::Point, ray::Ray, transformations::*},
	shapes::{shape::ConcreteShape, spheres::Sphere},
	visualisation::lights::PointLight,
};

pub struct World {
	pub objects: Vec<Box<dyn ConcreteShape>>,
	pub light: PointLight,
}

const REFLECTION_RECURSION_THRESHOLD: i32 = 3;

impl World {
	pub fn new(objects: Vec<Box<dyn ConcreteShape>>, light: PointLight) -> Self {
		Self { objects, light }
	}

	pub fn shade_hit(&self, comps: &IntersectionComputations, remaining: Option<i32>) -> Color {
		let in_shadow = self.is_shadowed(comps.over_point).unwrap();
		let surface = comps.object.material().lighting(
			comps.object,
			&self.light,
			&comps.over_point,
			&comps.eye,
			&comps.normal,
			in_shadow,
		);
		let reflected_color = self.reflected_color(comps, remaining);
		let refracted_color = self.refracted_color(comps, remaining);

		let material = comps.object.material();
		if material.reflective > 0.0 && material.transparency > 0.0 {
			let reflectance = comps.schlick();
			surface + reflected_color * reflectance + refracted_color * (1.0 - reflectance)
		} else {
			surface + reflected_color + refracted_color
		}
	}

	pub fn color_at(&self, ray: &Ray, remaining: Option<i32>) -> Result<Color, String> {
		let xs = ray.intersect_world(self)?;
		let hits = hit(&xs);
		let color = match hits {
			Some(intersection) => {
				let comps = ray.prepare_computations(&intersection, Some(&xs));
				self.shade_hit(&comps, remaining)
			},
			_ => Color::new(0.0, 0.0, 0.0),
		};
		Ok(color)
	}

	pub fn reflected_color(
		&self,
		comps: &IntersectionComputations,
		remaining: Option<i32>,
	) -> Color {
		if remaining.unwrap_or(REFLECTION_RECURSION_THRESHOLD) == 0 {
			return Color::new(0.0, 0.0, 0.0)
		}

		if comps.object.material().reflective == 0.0 {
			return Color::new(0.0, 0.0, 0.0)
		}

		let reflect_ray = Ray::new(comps.over_point, comps.reflection_vector);
		let color = self
			.color_at(&reflect_ray, Some(remaining.unwrap_or(REFLECTION_RECURSION_THRESHOLD) - 1))
			.expect("Could not compute reflected color");
		color * comps.object.material().reflective
	}

	pub fn refracted_color(
		&self,
		comps: &IntersectionComputations,
		remaining: Option<i32>,
	) -> Color {
		if remaining.unwrap_or(REFLECTION_RECURSION_THRESHOLD) == 0 {
			return Color::new(0.0, 0.0, 0.0)
		}
		if comps.object.material().transparency == 0.0 {
			return Color::new(0.0, 0.0, 0.0)
		}

		// Snell's law
		let ratio = comps.n1 / comps.n2;
		let cos_i = comps.eye.dot(&comps.normal);
		let sin2_t = ratio.powi(2) * (1.0 - cos_i.powi(2));
		if sin2_t > 1.0 {
			return Color::new(0.0, 0.0, 0.0)
		}

		let cos_t = (1.0 - sin2_t).sqrt();
		let direction = comps.normal * (ratio * cos_i - cos_t) - comps.eye * ratio;
		let refract_ray = Ray::new(comps.under_point, direction);
		let color = self
			.color_at(&refract_ray, Some(remaining.unwrap_or(REFLECTION_RECURSION_THRESHOLD) - 1))
			.expect("Could not compute refracted color");
		color * comps.object.material().transparency
	}

	pub fn is_shadowed(&self, point: Point) -> Result<bool, String> {
		let v = self.light.position - point;
		let distance = v.magnitude();
		let direction = v.normalise();

		let r = Ray::new(point, direction);
		let mut intersections = r.intersect_world(self)?;
		intersections.sort_by(|i1, i2| (i1.t).partial_cmp(&i2.t).unwrap());
		match hit(&intersections) {
			Some(h) =>
				if h.t < distance {
					Ok(true)
				} else {
					Ok(false)
				},
			None => Ok(false),
		}
	}
}

impl Default for World {
	fn default() -> Self {
		let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

		let mut s1 = Sphere::default();
		s1.get_material().color = Color::new(0.8, 1.0, 0.6);
		s1.get_material().diffuse = 0.7;
		s1.get_material().specular = 0.2;

		let mut s2 = Sphere::default();
		s2.set_transform(scaling(0.5, 0.5, 0.5));

		Self { objects: vec![Box::new(s1), Box::new(s2)], light }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		intersection::Intersection,
		patterns::color_pattern::ColorPattern,
		primitives::vector::Vector,
		shapes::{plane::Plane, shape::ConcreteShape, spheres::Sphere},
	};

	#[test]
	fn default_test() {
		let w = World::default();
		assert_eq!(w.objects[0].material().color, Color::new(0.8, 1.0, 0.6));
		assert_eq!(w.objects[0].material().diffuse, 0.7);
		assert_eq!(w.objects[0].material().specular, 0.2);
	}

	#[test]
	fn intersect_world() {
		let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
		let w = World::default();

		let xs = r.intersect_world(&w).unwrap();

		assert_eq!(xs.len(), 4);
		approx::assert_relative_eq!(xs[0].t, 4.0);
		approx::assert_relative_eq!(xs[1].t, 4.5);
		approx::assert_relative_eq!(xs[2].t, 5.5);
		approx::assert_relative_eq!(xs[3].t, 6.0);
	}

	#[test]
	fn shade_hit_test() {
		let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
		let w = World::default();
		let i = Intersection::new(4.0, &*w.objects[0]);
		let computations = r.prepare_computations(&i, None);
		let c = w.shade_hit(&computations, None);

		approx::assert_relative_eq!(
			c,
			Color::new(0.38066125, 0.4758265, 0.28549594),
			epsilon = 1e-6
		);

		let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
		let mut w = World::default();
		w.light = PointLight::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
		let i = Intersection::new(0.5, &*w.objects[1]);
		let computations = r.prepare_computations(&i, None);
		let c = w.shade_hit(&computations, None);

		approx::assert_relative_eq!(c, Color::new(0.9049845, 0.9049845, 0.9049845), epsilon = 1e-6);

		let s1 = Sphere::default();
		let mut s2 = Sphere::default();
		s2.set_transform(translation(0.0, 0.0, 10.0));
		let w = World::new(
			vec![Box::new(s1), Box::new(s2)],
			PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0)),
		);
		let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
		let i = Intersection::new(4.0, &*w.objects[1]);
		let comps = r.prepare_computations(&i, None);
		let c = w.shade_hit(&comps, None);

		approx::assert_relative_eq!(c, Color::new(0.1, 0.1, 0.1));
	}

	#[test]
	fn color_at_test() {
		// ray misses
		let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
		let w = World::default();
		let c = w.color_at(&r, None).unwrap();

		assert_eq!(c, Color::new(0.0, 0.0, 0.0));

		// ray hits
		let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
		let w = World::default();
		let c = w.color_at(&r, None).unwrap();

		approx::assert_relative_eq!(
			c,
			Color::new(0.38066125, 0.4758265, 0.28549594),
			epsilon = 1e-6
		);

		// ray hits between two spheres
		let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
		let mut w = World::default();
		w.objects[0].get_material().ambient = 1.0;
		w.objects[1].get_material().ambient = 1.0;
		let c = w.color_at(&r, None).unwrap();

		assert_eq!(c, w.objects[1].material().color);
	}

	#[test]
	fn is_shadowed_test() {
		let w = World::default();

		// no shadow when nothing is collinear with point and light
		let p = Point::new(0.0, 10.0, 0.0);
		assert!(!w.is_shadowed(p).unwrap());

		// the shadow when an object is between the point and the light
		let p = Point::new(10.0, -10.0, 10.0);
		assert!(w.is_shadowed(p).unwrap());

		// no shadow when an object is behind the light
		let p = Point::new(-20.0, 20.0, -20.0);
		assert!(!w.is_shadowed(p).unwrap());

		// no shadow when an object is behind the point
		let p = Point::new(0.0, 10.0, 0.0);
		assert!(!w.is_shadowed(p).unwrap());
	}

	#[test]
	fn test_reflected_color() {
		// reflected color, non-reflective surface
		let mut w = World::default();
		let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
		w.objects[1].get_material().ambient = 1.0;
		let i = Intersection::new(1.0, &*w.objects[1]);
		let comps = r.prepare_computations(&i, None);
		let color = w.reflected_color(&comps, None);
		assert_eq!(color, Color::new(0.0, 0.0, 0.0));

		// reflected color, reflective surface
		let mut w = World::default();
		let mut s = Plane::default();
		s.get_material().reflective = 0.5;
		s.set_transform(translation(0.0, -1.0, 0.0));
		w.objects.push(Box::new(s));

		let r = Ray::new(
			Point::new(0.0, 0.0, -3.0),
			Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
		);
		let i = Intersection::new(2.0_f64.sqrt(), &*w.objects[2]);
		let comps = r.prepare_computations(&i, None);
		let color = w.reflected_color(&comps, None);
		approx::assert_relative_eq!(color, Color::new(0.19032, 0.2379, 0.14274), epsilon = 1e-4);

		// shade hit, reflective surface
		let mut w = World::default();
		let mut s = Plane::default();
		s.get_material().reflective = 0.5;
		s.set_transform(translation(0.0, -1.0, 0.0));
		w.objects.push(Box::new(s));

		let r = Ray::new(
			Point::new(0.0, 0.0, -3.0),
			Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
		);
		let i = Intersection::new(2.0_f64.sqrt(), &*w.objects[2]);
		let comps = r.prepare_computations(&i, None);
		let color = w.shade_hit(&comps, None);
		approx::assert_relative_eq!(color, Color::new(0.87677, 0.92436, 0.82918), epsilon = 1e-4);

		// shade hit, reflective surface max recursion depth
		let color = w.reflected_color(&comps, Some(0));
		assert_eq!(color, Color::new(0.0, 0.0, 0.0));
	}

	#[test]
	fn test_inifinite_recursion_w_mutually_reflective_surfaces() {
		let mut w = World::default();
		w.light = PointLight::new(Point::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0));
		let mut lower = Plane::default();
		lower.get_material().reflective = 1.0;
		lower.set_transform(translation(0.0, -1.0, 0.0));

		let mut upper = Plane::default();
		upper.get_material().reflective = 1.0;
		upper.set_transform(translation(0.0, 1.0, 0.0));

		w.objects.push(Box::new(lower));
		w.objects.push(Box::new(upper));

		let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
		let _ = w.color_at(&r, None);
		assert!(true);
	}

	#[test]
	fn test_refracted_color() {
		let mut w = World::default();
		w.objects[0].get_material().ambient = 1.0;
		w.objects[0].get_material().pattern = Some(ColorPattern::new_test());

		w.objects[1].get_material().transparency = 1.0;
		w.objects[1].get_material().refractive_index = 1.5;

		let r = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0.0, 1.0, 0.0));
		let xs = vec![
			Intersection::new(-0.9899, &*w.objects[0]),
			Intersection::new(-0.4899, &*w.objects[1]),
			Intersection::new(0.4899, &*w.objects[1]),
			Intersection::new(0.9899, &*w.objects[0]),
		];
		let comps = r.prepare_computations(&xs[2], Some(&xs));
		let c = w.refracted_color(&comps, Some(5));
		approx::assert_relative_eq!(c, Color::new(0.0, 0.99888, 0.04725), epsilon = 1e-4);
	}

	#[test]
	fn test_shade_hit_w_transparent_material() {
		let mut w = World::default();

		let mut floor = Plane::default();
		floor.set_transform(translation(0.0, -1.0, 0.0));
		floor.get_material().transparency = 0.5;
		floor.get_material().refractive_index = 1.5;

		w.objects.push(Box::new(floor));

		let mut ball = Sphere::default();
		ball.get_material().color = Color::new(1.0, 0.0, 0.0);
		ball.get_material().ambient = 0.5;
		ball.set_transform(translation(0.0, -3.5, -0.5));

		w.objects.push(Box::new(ball));

		let r = Ray::new(
			Point::new(0.0, 0.0, -3.0),
			Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
		);
		let xs = vec![Intersection::new(2.0_f64.sqrt(), &*w.objects[2])];
		let comps = r.prepare_computations(&xs[0], Some(&xs));
		let color = w.shade_hit(&comps, Some(5));
		approx::assert_relative_eq!(color, Color::new(0.93642, 0.68642, 0.68642), epsilon = 1e-4);
	}

	#[test]
	fn test_schlick() {
		// total_internal_reflection
		let shape = Sphere::new_glass_sphere();
		let r = Ray::new(Point::new(0.0, 0.0, 2.0_f64.sqrt() / 2.0), Vector::new(0.0, 1.0, 0.0));
		let xs = vec![
			Intersection::new(-2.0_f64.sqrt() / 2.0, &shape),
			Intersection::new(2.0_f64.sqrt() / 2.0, &shape),
		];
		let comps = r.prepare_computations(&xs[1], Some(&xs));
		assert_eq!(comps.schlick(), 1.0);

		// perpendicular ray
		let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
		let xs = vec![Intersection::new(-1.0, &shape), Intersection::new(1.0, &shape)];
		let comps = r.prepare_computations(&xs[1], Some(&xs));
		approx::assert_relative_eq!(comps.schlick(), 0.04, epsilon = 1e-4);

		// tangential ray
		let r = Ray::new(Point::new(0.0, 0.99, -2.0), Vector::new(0.0, 0.0, 1.0));
		let xs = vec![Intersection::new(1.8589, &shape)];
		let comps = r.prepare_computations(&xs[0], Some(&xs));
		approx::assert_relative_eq!(comps.schlick(), 0.48873, epsilon = 1e-4);
	}

	#[test]
	fn test_shade_hit_w_reflective_transparent_material() {
		let mut w = World::default();

		let mut floor = Plane::default();
		floor.set_transform(translation(0.0, -1.0, 0.0));
		floor.get_material().transparency = 0.5;
		floor.get_material().reflective = 0.5;
		floor.get_material().refractive_index = 1.5;

		w.objects.push(Box::new(floor));

		let mut ball = Sphere::default();
		ball.get_material().color = Color::new(1.0, 0.0, 0.0);
		ball.get_material().ambient = 0.5;
		ball.set_transform(translation(0.0, -3.5, -0.5));

		w.objects.push(Box::new(ball));

		let r = Ray::new(
			Point::new(0.0, 0.0, -3.0),
			Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
		);
		let xs = vec![Intersection::new(2.0_f64.sqrt(), &*w.objects[2])];
		let comps = r.prepare_computations(&xs[0], Some(&xs));
		let color = w.shade_hit(&comps, Some(5));
		approx::assert_relative_eq!(color, Color::new(0.93391, 0.69643, 0.69243), epsilon = 1e-4);
	}
}
