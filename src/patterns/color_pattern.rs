use crate::{color::Color, point::Point, matrix::matrix4d::Matrix4D, shapes::shape::ConcreteShape};
use core::fmt::Debug;
use crate::{patterns::stripe_pattern::StripePattern};


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ColorPattern {
    StripePattern(StripePattern)
}

impl ColorPattern {

    pub fn new_stripe(a: Color, b: Color) -> Self {
        Self::StripePattern(StripePattern::new(a, b))
    }

    fn transform(&self) -> &Matrix4D {
        match self {
            Self::StripePattern(p) => p.transform()
        }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        match self {
            Self::StripePattern(p) => p.stripe_at(point)
        }
    }

    pub fn pattern_at_object<'a>(&self, object: &'a dyn ConcreteShape, point: &Point) -> Color {
        match self {
            Self::StripePattern(p) => p.stripe_at_object(object, point)
        }
    }
}

pub trait Pattern {
    fn set_transform(&mut self, transform: Matrix4D);
    fn transform(&self) -> &Matrix4D;
    fn get_transform(&mut self) -> &mut Matrix4D;
    fn stripe_at(&self, point: &Point) -> Color;
    fn stripe_at_object<'a>(&self, object: &'a dyn ConcreteShape, world_point: &Point) -> Color;
}