use crate::tuple::Tuple;
use std::ops::{Add, Sub, Div, Mul, Neg};
use approx::{RelativeEq, AbsDiffEq};


#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vector {
    pub tuple: Tuple,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { 
            tuple: Tuple::new(x, y, z, 0.0) 
        }
    }
    pub fn from_tuple(tuple: Tuple) -> Self {
        Self { 
            tuple: tuple 
        }
    }

    pub fn magnitude(&self) -> f64 {
        self.tuple.abs()
    }

    pub fn normalise(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn dot(&self, rhs: &Vector) -> f64 {
        self.tuple.dot(rhs.tuple)
    }

    pub fn cross(&self, rhs: &Vector) -> Self {
        Vector { 
            tuple: self.tuple.cross_3D(rhs.tuple) 
        }
    }

    pub fn reflect(&self, normal: Vector) -> Self {
        *self - normal * 2.0 * self.dot(&normal)
    }

}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector { tuple: self.tuple + rhs.tuple }
    }
}

impl Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector { tuple: self.tuple - rhs.tuple }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vector { tuple: self.tuple / rhs }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector { tuple: self.tuple * rhs }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            tuple: -self.tuple
        }
    }
}

impl AbsDiffEq for Vector {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        Tuple::abs_diff_eq(&self.tuple, &other.tuple, epsilon)
    }
}

impl RelativeEq for Vector {

    fn default_max_relative() -> f64 {
        f64::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: f64, max_relative: f64) -> bool {
        Tuple::relative_eq(&self.tuple, &other.tuple, epsilon, max_relative)
    }
}


#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;
    use crate::vector::Vector;

    #[test]
    fn is_tuple() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let t = Tuple::new(1.0, 2.0, 3.0, 0.0);

        assert_eq!(&v.tuple, &t);
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);

        let v = Vector::new(0.0, 0.0, 0.0);

        assert_eq!(&(v1 - v2), &v);
    }

    #[test]
    fn magnitude_of_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        approx::assert_relative_eq!(v.magnitude(), f64::sqrt(14.0));

        let v = Vector::new(-1.0, -2.0, -3.0);
        approx::assert_relative_eq!(v.magnitude(), f64::sqrt(14.0));

        let v = Vector::new(1.0, 0.0, 0.0);
        approx::assert_relative_eq!(v.magnitude(), 1.0);

        let v = Vector::new(1.0, 0.0, 1.0);
        approx::assert_relative_eq!(v.magnitude(), f64::sqrt(2.0));
    }

    #[test]
    fn normalise_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        // let v_normalised = Vector::new(0.26726, 0.53452, 0.80178);
        approx::assert_relative_eq!(v.normalise().magnitude(), 1.0)
    }

    #[test]
    fn dot_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        approx::assert_relative_eq!(v1.dot(&v2), 32.0)
    }

    #[test]
    fn cross_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let v12_ = v1.cross(&v2);
        let v21_ = v2.cross(&v1);

        let v12 = Vector::new(-1.0, 2.0, -1.0);
        let v21 = Vector::new(1.0, -2.0, 1.0);
        assert_eq!(&v12_, &v12);
        assert_eq!(&v21_, &v21);
    }

    #[test]
    fn reflection() {
        let v1 = Vector::new(1.0, -1.0, 0.0);
        let n1 = Vector::new(0.0, 1.0, 0.0);
        let r = v1.reflect(n1);
        approx::assert_relative_eq!(r, Vector::new(1.0, 1.0, 0.0));

        let v1 = Vector::new(0.0, -1.0, 0.0);
        let n1 = Vector::new( 2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0, 0.0);
        let r = v1.reflect(n1);
        approx::assert_relative_eq!(r, Vector::new(1.0, 0.0, 0.0));
    }
}