use std::ops::{Add, Mul, Sub};
use std::cmp::PartialEq;

use approx::{RelativeEq, AbsDiffEq};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red: red, green: green, blue: blue }
    }

    pub fn normalise(&self, min: f32, max: f32) -> (u32, u32, u32) {
        let red = num::clamp(self.red * max, min, max);
        let green = num::clamp(self.green * max, min, max);
        let blue = num::clamp(self.blue * max, min, max);
        (red as u32, green as u32, blue as u32)
    }
}


impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
    
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs, 
            blue: self.blue * rhs,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green, 
            blue: self.blue * rhs.blue,
        }
    }
}

impl AbsDiffEq for Color {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f32) -> bool {
        f32::abs_diff_eq(&self.red, &other.red, epsilon) &&
        f32::abs_diff_eq(&self.green, &other.green, epsilon) &&
        f32::abs_diff_eq(&self.blue, &other.blue, epsilon)
    }
}

impl RelativeEq for Color {

    fn default_max_relative() -> f32 {
        f32::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: f32, max_relative: f32) -> bool {
        f32::relative_eq(&self.red, &other.red, epsilon, max_relative) &&
        f32::relative_eq(&self.green, &other.green, epsilon, max_relative) &&
        f32::relative_eq(&self.blue, &other.blue, epsilon, max_relative)
    }
}

#[cfg(test)]
mod tests {

    use crate::color::Color;

    #[test]
    fn addition() {
        let c1 = Color{ red: 0.9, green: 0.4, blue: 0.5};
        let c2 = Color{ red: 0.2, green: 0.3, blue: 0.4};
        let c = Color { red: 1.1, green: 0.7, blue: 0.9 };

        approx::assert_relative_eq!(&(c1 + c2), &c);
    }

    #[test]
    fn subtraction() {
        let c1 = Color{ red: 0.9, green: 0.4, blue: 0.5};
        let c2 = Color{ red: 0.2, green: 0.3, blue: 0.4};
        let c = Color { red: 0.7, green: 0.1, blue: 0.1 };

        approx::assert_relative_eq!(&(c1 - c2), &c);
    }

    #[test]
    fn multiplication_by_scalar() {
        let c1 = Color { red: 1.1, green: 0.7, blue: 0.9 };
        let c2 = Color { red: 2.2, green: 1.4, blue: 1.8 };

        approx::assert_relative_eq!(&(c1 * 2.0), &c2);
    }

    #[test]
    fn hadamard_product() {
        let c1 = Color{ red: 0.9, green: 0.4, blue: 0.5};
        let c2 = Color{ red: 0.2, green: 0.3, blue: 0.4};
        let c = Color { red: 0.18, green: 0.12, blue: 0.2 };
        
        approx::assert_relative_eq!(&(c1 * c2), &c);
    }

}

