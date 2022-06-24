use crate::tuple::Tuple;
use crate::vector::Vector;
use std::ops::{Add, Sub};


#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Point {
    pub tuple: Tuple,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { 
            tuple: Tuple::new(x, y, z, 1.0) 
        }
    }
}

// Adding vector to a point
impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        let t = self.tuple + rhs.tuple;
        Point {tuple: t}
    }
}

// Subtracting two points
impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        let t = self.tuple - rhs.tuple;
        Vector::from_tuple(t)
    }
}

// Subtracting vector from a point
impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        let t = self.tuple - rhs.tuple;
        Point {tuple: t}
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;
    use crate::point::Point;
    use crate::vector::Vector;


    #[test]
    fn is_tuple() {
        let p = Point::new(1.0, 2.0, 3.0);
        let t = Tuple::new(1.0, 2.0, 3.0, 1.0);

        assert_eq!(&p.tuple, &t);
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        let v = Vector::new(-2.0, -4.0, -6.0);

        assert_eq!(&(p1 - p2), &v)
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        let p2 = Point::new(-2.0, -4.0, -6.0);

        assert_eq!(&(p1 - v), &p2)
    }

}