use crate::{color::{Color}, lights::PointLight, point::Point, vector::Vector};
use approx::{AbsDiffEq, RelativeEq};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {color: color, ambient: ambient, diffuse: diffuse, specular: specular, shininess: shininess}
    }

    pub fn lighting(&self, light: &PointLight, point: &Point, eye: &Vector, normal: &Vector, in_shadow: bool) -> Color {
        let effective_color = self.color * light.intensity;
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
        Self { color: Color::new(1.0, 1.0, 1.0), ambient: 0.1, diffuse: 0.9, specular: 0.9, shininess: 200.0 }
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
    use crate::{vector::Vector, lights::PointLight, color::Color, point::Point};

    use super::Material;

    #[test]
    fn lighting_test() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);

        // Lighting with the eye between the light and the surface
        let eye = Vector::new(0.0, 0.0, -1.0);
        let n = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Point::new(0.0, 0.0, -10.0), 
            Color::new(1.0, 1.0, 1.0)
        );

        let result = m.lighting(&light, &position, &eye, &n, false);
        approx::assert_relative_eq!(result, Color::new(1.9, 1.9, 1.9));

        // Lighting with the eye between the light and the surface, eye offset 45 deg
        let eye = Vector::new(0.0, 2.0f64.sqrt()/2.0, -2.0f64.sqrt()/2.0);
        let n = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Point::new(0.0, 0.0, -10.0), 
            Color::new(1.0, 1.0, 1.0)
        );
        let result = m.lighting(&light, &position, &eye, &n, false);
        approx::assert_relative_eq!(result, Color::new(1.0, 1.0, 1.0));

        // Lighting with the surface in shadow
        let eye = Vector::new(0.0, 0.0, -1.0);
        let n = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Point::new(0.0, 0.0, -10.0), 
            Color::new(1.0, 1.0, 1.0)
        );

        let result = m.lighting(&light, &position, &eye, &n, true);
        approx::assert_relative_eq!(result, Color::new(0.1, 0.1, 0.1));

        // Lighting with eye opposite surface
        let eye = Vector::new(0.0, 0.0, -1.0);
        let n = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Point::new(0.0, 10.0, -10.0), 
            Color::new(1.0, 1.0, 1.0)
        );

        let result = m.lighting(&light, &position, &eye, &n, false);
        approx::assert_relative_eq!(result, Color::new(0.736396, 0.736396, 0.736396), epsilon=1e-5);

        // Lighting with eye in the path of the reflection vector
        let eye = Vector::new(0.0, -2.0f64.sqrt()/2.0, -2.0f64.sqrt()/2.0);
        let n = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Point::new(0.0, 10.0, -10.0), 
            Color::new(1.0, 1.0, 1.0)
        );

        let result = m.lighting(&light, &position, &eye, &n, false);
        approx::assert_relative_eq!(result, Color::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928));

        // Lighting with light behind the surface
        let eye = Vector::new(0.0, 0.0, -1.0);
        let n = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Point::new(0.0, 0.0, 10.0), 
            Color::new(1.0, 1.0, 1.0)
        );

        let result = m.lighting(&light, &position, &eye, &n, false);
        approx::assert_relative_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}