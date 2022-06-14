use crate::tuple::Tuple;
use std::ops::{Add, Sub, Div};


#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vector {
    pub tuple: Tuple,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { 
            tuple: Tuple::new(x, y, z, 0.0) 
        }
    }
    pub fn from_tuple(tuple: Tuple) -> Self {
        Self { 
            tuple: tuple 
        }
    }

    pub fn magnitude(&self) -> f32 {
        self.tuple.abs()
    }

    pub fn normalise(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn dot(&self, rhs: &Vector) -> f32 {
        self.tuple.dot(rhs.tuple)
    }

    pub fn cross(&self, rhs: &Vector) -> Self {
        Vector { 
            tuple: self.tuple.cross_3D(rhs.tuple) 
        }
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

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vector { tuple: self.tuple / rhs }
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
        approx::assert_relative_eq!(v.magnitude(), f32::sqrt(14.0));

        let v = Vector::new(-1.0, -2.0, -3.0);
        approx::assert_relative_eq!(v.magnitude(), f32::sqrt(14.0));

        let v = Vector::new(1.0, 0.0, 0.0);
        approx::assert_relative_eq!(v.magnitude(), 1.0);

        let v = Vector::new(1.0, 0.0, 1.0);
        approx::assert_relative_eq!(v.magnitude(), f32::sqrt(2.0));
    }

    #[test]
    fn normalise_vector () {
        let v = Vector::new(1.0, 2.0, 3.0);
        // let v_normalised = Vector::new(0.26726, 0.53452, 0.80178);
        approx::assert_relative_eq!(v.normalise().magnitude(), 1.0)
    }

    #[test]
    fn dot_product () {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        approx::assert_relative_eq!(v1.dot(&v2), 32.0)
    }

    #[test]
    fn cross_product () {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let v12_ = v1.cross(&v2);
        let v21_ = v2.cross(&v1);

        let v12 = Vector::new(-1.0, 2.0, -1.0);
        let v21 = Vector::new(1.0, -2.0, 1.0);
        assert_eq!(&v12_, &v12);
        assert_eq!(&v21_, &v21);
    }
}