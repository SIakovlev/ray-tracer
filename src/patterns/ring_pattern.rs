use crate::{color::Color, point::Point, matrix::matrix4d::Matrix4D};

use super::color_pattern::Pattern;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RingPattern {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix4D,
}

impl RingPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self {a: a, b: b, transform: Matrix4D::identity()}
    }
}

impl Pattern for RingPattern {
    fn transform(&self) -> &Matrix4D {
        &self.transform
    }

    fn get_transform(&mut self) -> &mut Matrix4D {
        &mut self.transform
    }

    fn set_transform(&mut self, transform: Matrix4D) {
        self.transform = transform
    }

    fn pattern_at(&self, point: &Point) -> Color {
        let condition = (point.tuple.x.powf(2.0) + point.tuple.z.powf(2.0)).sqrt().floor() as i64 % 2;
        match condition {
            0 => self.a,
            _ => self.b
        }
    }
}