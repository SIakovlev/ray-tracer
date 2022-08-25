use crate::{color::Color, point::Point, matrix::matrix4d::Matrix4D};

use super::color_pattern::Pattern;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct StripePattern {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix4D,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        StripePattern {a: a, b: b, transform: Matrix4D::identity()}
    }
}

impl Pattern for StripePattern {
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
        match point.tuple.x.floor() as i64 % 2 {
            0 => self.a,
            _ => self.b
        }
    }
}