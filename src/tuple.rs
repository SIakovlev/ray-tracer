use std::ops::{Add, Sub, Neg, Div, Mul};
use std::cmp::PartialEq;


#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple{ x, y, z, w }
    }

    pub fn dot(&self, rhs: Tuple) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn cross_3D(&self, rhs: Tuple) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
            w: self.w,
        }
    }

    pub fn abs(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }   
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f32> for Tuple {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    #[test]
    fn addition() {
        let t1 = Tuple {x: 3.0, y: -2.0, z: 5.0, w: 1.0 };
        let t2 = Tuple {x: -2.0, y: 3.0, z: 1.0, w: 0.0 };

        let t3 = t2 + t1;

        assert_eq!(&t3, &Tuple {x: 1.0, y: 1.0, z: 6.0, w: 1.0 });
    }

    #[test]
    fn subtraction() {
        let t1 = Tuple {x: 1.0, y: 2.0, z: 3.0, w: 1.0 };
        let t2 = Tuple {x: 8.0, y: 3.0, z: 5.0, w: 4.0 };

        let t3 = t2 - t1;

        assert_eq!(&t3, &Tuple {x: 7.0, y: 1.0, z: 2.0, w: 3.0 })
    }

    #[test]
    fn negation() {
        let t1 = Tuple {x: 1.0, y: 2.0, z: 3.0, w: 1.0 };
        let t2 = Tuple {x: -1.0, y: -2.0, z: -3.0, w: -1.0 };

        assert_eq!(&-t1, &t2)
    }
}