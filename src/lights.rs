use crate::{point::Point, color::Color};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> Self {
        Self { position: position, intensity: intensity }
    }
}

#[cfg(test)]
mod tests {
    use super::PointLight;
    use crate::{point::Point, color::Color};

    #[test]
    fn initialisation() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point::new(1.0, 1.0, 1.0);

        let light = PointLight::new(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
