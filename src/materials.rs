use crate::{
	color::Color, lights::PointLight, patterns::color_pattern::ColorPattern, point::Point,
	shapes::shape::ConcreteShape, vector::Vector,
};
use approx::{AbsDiffEq, RelativeEq};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Material {
	pub pattern: Option<ColorPattern>,
	pub color: Color,
	pub reflective: f64,
	pub ambient: f64,
	pub diffuse: f64,
	pub specular: f64,
	pub shininess: f64,
}

impl Material {
	pub fn new(
		pattern: Option<ColorPattern>,
		color: Color,
		reflective: f64,
		ambient: f64,
		diffuse: f64,
		specular: f64,
		shininess: f64,
	) -> Self {
		Self { pattern, color, reflective, ambient, diffuse, specular, shininess }
	}

	pub fn lighting(
		&self,
		object: &dyn ConcreteShape,
		light: &PointLight,
		point: &Point,
		eye: &Vector,
		normal: &Vector,
		in_shadow: bool,
	) -> Color {
		let mut color = self.color;
		if let Some(pattern) = self.pattern {
			color = pattern.pattern_at_object(object, point);
		}

		let effective_color = color * light.intensity;
		let light_dir = (light.position - *point).normalise();

		let ambient = effective_color * self.ambient;
		let mut diffuse = Color::new(0.0, 0.0, 0.0);
		let mut specular = Color::new(0.0, 0.0, 0.0);

		let light_dot_normal = light_dir.dot(&normal);
		if light_dot_normal >= 0.0 && !in_shadow {
			diffuse = effective_color * self.diffuse * light_dot_normal;
			let reflect_dir = -light_dir.reflect(*normal);
			let reflect_dot_eye = reflect_dir.dot(eye);
			if reflect_dot_eye > 0.0 {
				let factor = reflect_dot_eye.powf(self.shininess);
				specular = light.intensity * self.specular * factor;
			}
		}
		ambient + diffuse + specular
	}
}

impl Default for Material {
	fn default() -> Self {
		Self {
			pattern: None,
			color: Color::new(1.0, 1.0, 1.0),
			reflective: 0.0,
			ambient: 0.1,
			diffuse: 0.9,
			specular: 0.9,
			shininess: 200.0,
		}
	}
}

impl AbsDiffEq for Material {
	type Epsilon = f64;

	fn default_epsilon() -> Self::Epsilon {
		f64::default_epsilon()
	}

	fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
		Color::abs_diff_eq(&self.color, &other.color, epsilon) &&
			f64::abs_diff_eq(&self.ambient, &other.ambient, epsilon) &&
			f64::abs_diff_eq(&self.diffuse, &other.diffuse, epsilon) &&
			f64::abs_diff_eq(&self.specular, &other.specular, epsilon) &&
			f64::abs_diff_eq(&self.shininess, &other.shininess, epsilon)
	}
}

impl RelativeEq for Material {
	fn default_max_relative() -> f64 {
		f64::default_max_relative()
	}

	fn relative_eq(&self, other: &Self, epsilon: f64, max_relative: f64) -> bool {
		Color::relative_eq(&self.color, &other.color, epsilon, max_relative) &&
			f64::relative_eq(&self.ambient, &other.ambient, epsilon, max_relative) &&
			f64::relative_eq(&self.diffuse, &other.diffuse, epsilon, max_relative) &&
			f64::relative_eq(&self.specular, &other.specular, epsilon, max_relative) &&
			f64::relative_eq(&self.shininess, &other.shininess, epsilon, max_relative)
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		color::Color,
		intersection::Intersection,
		lights::PointLight,
		patterns::color_pattern::ColorPattern,
		point::Point,
		ray::Ray,
		shapes::{plane::Plane, spheres::Sphere},
		vector::Vector,
	};

	use super::Material;

	#[test]
	fn lighting_test() {
		let s = Sphere::default();
		let m = Material::default();
		let position = Point::new(0.0, 0.0, 0.0);

		// Lighting with the eye between the light and the surface
		let eye = Vector::new(0.0, 0.0, -1.0);
		let n = Vector::new(0.0, 0.0, -1.0);
		let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

		let result = m.lighting(&s, &light, &position, &eye, &n, false);
		approx::assert_relative_eq!(result, Color::new(1.9, 1.9, 1.9));

		// Lighting with the eye between the light and the surface, eye offset 45 deg
		let eye = Vector::new(0.0, 2.0f64.sqrt() / 2.0, -2.0f64.sqrt() / 2.0);
		let n = Vector::new(0.0, 0.0, -1.0);
		let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
		let result = m.lighting(&s, &light, &position, &eye, &n, false);
		approx::assert_relative_eq!(result, Color::new(1.0, 1.0, 1.0));

		// Lighting with the surface in shadow
		let eye = Vector::new(0.0, 0.0, -1.0);
		let n = Vector::new(0.0, 0.0, -1.0);
		let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

		let result = m.lighting(&s, &light, &position, &eye, &n, true);
		approx::assert_relative_eq!(result, Color::new(0.1, 0.1, 0.1));

		// Lighting with eye opposite surface
		let eye = Vector::new(0.0, 0.0, -1.0);
		let n = Vector::new(0.0, 0.0, -1.0);
		let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

		let result = m.lighting(&s, &light, &position, &eye, &n, false);
		approx::assert_relative_eq!(
			result,
			Color::new(0.736396, 0.736396, 0.736396),
			epsilon = 1e-5
		);

		// Lighting with eye in the path of the reflection vector
		let eye = Vector::new(0.0, -2.0f64.sqrt() / 2.0, -2.0f64.sqrt() / 2.0);
		let n = Vector::new(0.0, 0.0, -1.0);
		let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

		let result = m.lighting(&s, &light, &position, &eye, &n, false);
		approx::assert_relative_eq!(
			result,
			Color::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928)
		);

		// Lighting with light behind the surface
		let eye = Vector::new(0.0, 0.0, -1.0);
		let n = Vector::new(0.0, 0.0, -1.0);
		let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

		let result = m.lighting(&s, &light, &position, &eye, &n, false);
		approx::assert_relative_eq!(result, Color::new(0.1, 0.1, 0.1));

		// Lighting with pattern applied
		let mut m1 = Material::default();
		m1.pattern =
			Some(ColorPattern::new_stripe(Color::new(1.0, 1.0, 1.0), Color::new(0.0, 0.0, 0.0)));
		m1.ambient = 1.0;
		m1.diffuse = 0.0;
		m1.specular = 0.0;
		let eye = Vector::new(0.0, 0.0, -1.0);
		let n = Vector::new(0.0, 0.0, -1.0);
		let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
		let c1 = m1.lighting(&s, &light, &Point::new(0.9, 0.0, 0.0), &eye, &n, false);
		let c2 = m1.lighting(&s, &light, &Point::new(1.1, 0.0, 0.0), &eye, &n, false);

		approx::assert_relative_eq!(c1, Color::new(1.0, 1.0, 1.0));
		approx::assert_relative_eq!(c2, Color::new(0.0, 0.0, 0.0));
	}

	#[test]
	fn test_reflection_vector() {
		let p = Plane::default();
		let r = Ray::new(
			Point::new(0.0, 1.0, -1.0),
			Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
		);
		let i = Intersection::new(2.0_f64.sqrt(), &p);
		let comps = r.prepare_computations(&i);
		assert_eq!(
			comps.reflection_vector,
			Vector::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
		);
	}
}
