use crate::{color::Color, point::Point, shapes::shape::ConcreteShape, matrix::matrix4d::Matrix4D};

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

    fn stripe_at(&self, point: &Point) -> Color {
        match point.tuple.x.floor() as i64 % 2 {
            0 => self.a,
            _ => self.b
        }
    }

    fn stripe_at_object<'a>(&self, object: &'a dyn ConcreteShape, world_point: &Point) -> Color {
        let obj_point = object.transform().inverse().expect("Could not invert object transform") * (*world_point);
        let pattern_point = self.transform.inverse().expect("Could not invert pattern transform") * obj_point;

        self.stripe_at(&pattern_point)
    }
}

#[cfg(test)]
mod tests {
    use super::Pattern;
    use crate::patterns::stripe_pattern::StripePattern;
    use crate::{color::Color, point::Point};


    #[test]
    fn test_stripe_pattern() {
        let white = Color::new(0.0, 0.0, 0.0);
        let black = Color::new(1.0, 1.0, 1.0);

        let pattern = StripePattern::new(white, black);
        
        // constant in y
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 1.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 2.0, 0.0)), white);

        // constant in z
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 1.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 2.0)), white);

        // alternates in x
        assert_eq!(pattern.stripe_at(&Point::new(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(0.9, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(&Point::new(1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.stripe_at(&Point::new(-0.1, 0.0, 0.0)), black);
        assert_eq!(pattern.stripe_at(&Point::new(-1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.stripe_at(&Point::new(-1.1, 0.0, 0.0)), white);
    }
}