

use crate::matrix::matrix4d::Matrix4D;

pub fn translation(x: f32, y: f32, z: f32) -> Matrix4D {
    Matrix4D::new(
        [[1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0]]
    )
}
    
pub fn scaling(x: f32, y: f32, z: f32) -> Matrix4D{
    Matrix4D::new(
        [[x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0]]
    )
}

pub fn rotation_x(alpha: f32) -> Matrix4D {
    Matrix4D::new(
        [[1.0, 0.0, 0.0, 0.0],
        [0.0, alpha.cos(), -alpha.sin(), 0.0],
        [0.0, alpha.sin(), alpha.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]]
    )
}

pub fn rotation_y(alpha: f32) -> Matrix4D {
    Matrix4D::new(
        [[alpha.cos(), 0.0, alpha.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-alpha.sin(), 0.0, alpha.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]]
    )
}

pub fn rotation_z(alpha: f32) -> Matrix4D {
    Matrix4D::new(
        [[alpha.cos(), -alpha.sin(), 0.0, 0.0],
        [alpha.sin(), alpha.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]]
    )
}

pub fn shearing(x_y: f32, x_z: f32, y_x: f32, y_z: f32, z_x: f32, z_y: f32) -> Matrix4D {
    Matrix4D::new(
        [[1.0, x_y, x_z, 0.0],
        [y_x, 1.0, y_z, 0.0],
        [z_x, z_y, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]]
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point::Point, vector::Vector};

    #[test]
    fn translation_test() {
        // simple translation
        let transform = translation(5.0, -3.0, 2.0);
        let p1 = Point::new(-3.0, 4.0, 5.0);

        let p = Point::new(2.0, 1.0, 7.0);

        assert_eq!(transform*p1, p);
        
        // does not effect vectors
        let transform = translation(5.0, -3.0, 2.0);
        let v1 = Vector::new(-3.0, 4.0, 5.0);

        assert_eq!(transform*v1, v1);
    }

    #[test]
    fn scaling_test() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p1 = Point::new(-4.0, 6.0, 8.0);

        let p = Point::new(-8.0, 18.0, 32.0);

        assert_eq!(transform*p1, p);

        let transform = scaling(2.0, 3.0, 4.0);
        let v1 = Vector::new(-4.0, 6.0, 8.0);

        let v = Vector::new(-8.0, 18.0, 32.0);

        assert_eq!(transform*v1, v);
        
        // inverse scaling
        let transform = scaling(2.0, 3.0, 4.0).inverse().unwrap();
        let v1 = Vector::new(-4.0, 6.0, 8.0);

        let v = Vector::new(-2.0, 2.0, 2.0);

        assert_eq!(transform*v1, v);
    }

    #[test]
    fn rotation_test() {
        use std::f32;

        // x axis tests
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter_x = rotation_x(f32::consts::PI / 4.0);
        let full_quarter_x = rotation_x(f32::consts::PI / 2.0);

        approx::assert_relative_eq!(half_quarter_x * p, Point::new(0.0, 2.0f32.sqrt() / 2.0, 2.0f32.sqrt() / 2.0));
        approx::assert_relative_eq!(full_quarter_x * p, Point::new(0.0, 0.0, 1.0));
        
        // test rotation inverse
        approx::assert_relative_eq!(half_quarter_x.inverse().unwrap() * p, Point::new(0.0, 2.0f32.sqrt() / 2.0, -2.0f32.sqrt() / 2.0));
        
        // y axis tests
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter_y = rotation_y(f32::consts::PI / 4.0);
        let full_quarter_y = rotation_y(f32::consts::PI / 2.0);

        approx::assert_relative_eq!(half_quarter_y * p, Point::new(2.0f32.sqrt() / 2.0, 0.0, 2.0f32.sqrt() / 2.0));
        approx::assert_relative_eq!(full_quarter_y * p, Point::new(1.0, 0.0, 0.0));

        // z axis tests
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter_z = rotation_z(f32::consts::PI / 4.0);
        let full_quarter_z = rotation_z(f32::consts::PI / 2.0);

        approx::assert_relative_eq!(half_quarter_z * p, Point::new(-2.0f32.sqrt() / 2.0, 2.0f32.sqrt() / 2.0, 0.0));
        approx::assert_relative_eq!(full_quarter_z * p, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_test() {
        let p = Point::new(2.0, 3.0, 4.0);

        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        approx::assert_relative_eq!(transform * p, Point::new(5.0, 3.0, 4.0));

        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        approx::assert_relative_eq!(transform * p, Point::new(6.0, 3.0, 4.0));

        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        approx::assert_relative_eq!(transform * p, Point::new(2.0, 5.0, 4.0));

        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        approx::assert_relative_eq!(transform * p, Point::new(2.0, 7.0, 4.0));

        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        approx::assert_relative_eq!(transform * p, Point::new(2.0, 3.0, 6.0));

        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        approx::assert_relative_eq!(transform * p, Point::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn chaining_test() {
        use std::f32;
        let p = Point::new(1.0, 0.0, 1.0);

        let A = rotation_x(f32::consts::PI / 2.0);
        let B = scaling(5.0, 5.0, 5.0);
        let C = translation(10.0, 5.0, 7.0);

        let T = C * B * A;
        approx::assert_relative_eq!(T * p, Point::new(15.0, 0.0, 7.0));
    }
}