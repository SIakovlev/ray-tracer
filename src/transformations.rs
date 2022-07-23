use crate::{matrix::matrix4d::Matrix4D, vector::Vector, point::Point};

pub fn translation(x: f64, y: f64, z: f64) -> Matrix4D {
    Matrix4D::new(
        [[1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0]]
    )
}
    
pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4D{
    Matrix4D::new(
        [[x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0]]
    )
}

pub fn rotation_x(alpha: f64) -> Matrix4D {
    Matrix4D::new(
        [[1.0, 0.0, 0.0, 0.0],
        [0.0, alpha.cos(), -alpha.sin(), 0.0],
        [0.0, alpha.sin(), alpha.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]]
    )
}

pub fn rotation_y(alpha: f64) -> Matrix4D {
    Matrix4D::new(
        [[alpha.cos(), 0.0, alpha.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-alpha.sin(), 0.0, alpha.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]]
    )
}

pub fn rotation_z(alpha: f64) -> Matrix4D {
    Matrix4D::new(
        [[alpha.cos(), -alpha.sin(), 0.0, 0.0],
        [alpha.sin(), alpha.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]]
    )
}

pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix4D {
    Matrix4D::new(
        [[1.0, x_y, x_z, 0.0],
        [y_x, 1.0, y_z, 0.0],
        [z_x, z_y, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]]
    )
}

pub fn view_transform(from: Point, to: Point, up: Vector) -> Matrix4D {

    let forward = (to - from).normalise();
    let upn = (up).normalise();
    let left = forward.cross(&upn);
    let true_up = left.cross(&forward);

    Matrix4D { data: [[left.tuple.x, left.tuple.y, left.tuple.z, 0.0],
        [true_up.tuple.x, true_up.tuple.y, true_up.tuple.z, 0.0],
        [-forward.tuple.x, -forward.tuple.y, -forward.tuple.z, 0.0],
        [0.0, 0.0, 0.0, 1.0]] } * translation(-from.tuple.x, -from.tuple.y, -from.tuple.z)
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
        use std::f64;

        // x axis tests
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter_x = rotation_x(f64::consts::PI / 4.0);
        let full_quarter_x = rotation_x(f64::consts::PI / 2.0);

        approx::assert_relative_eq!(half_quarter_x * p, Point::new(0.0, 2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0));
        approx::assert_relative_eq!(full_quarter_x * p, Point::new(0.0, 0.0, 1.0));
        
        // test rotation inverse
        approx::assert_relative_eq!(half_quarter_x.inverse().unwrap() * p, Point::new(0.0, 2.0f64.sqrt() / 2.0, -2.0f64.sqrt() / 2.0));
        
        // y axis tests
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter_y = rotation_y(f64::consts::PI / 4.0);
        let full_quarter_y = rotation_y(f64::consts::PI / 2.0);

        approx::assert_relative_eq!(half_quarter_y * p, Point::new(2.0f64.sqrt() / 2.0, 0.0, 2.0f64.sqrt() / 2.0));
        approx::assert_relative_eq!(full_quarter_y * p, Point::new(1.0, 0.0, 0.0));

        // z axis tests
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter_z = rotation_z(f64::consts::PI / 4.0);
        let full_quarter_z = rotation_z(f64::consts::PI / 2.0);

        approx::assert_relative_eq!(half_quarter_z * p, Point::new(-2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0, 0.0));
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
    #[allow(non_snake_case)]
    fn chaining_test() {
        use std::f64;
        let p = Point::new(1.0, 0.0, 1.0);

        let A = rotation_x(f64::consts::PI / 2.0);
        let B = scaling(5.0, 5.0, 5.0);
        let C = translation(10.0, 5.0, 7.0);

        let T = C * B * A;
        approx::assert_relative_eq!(T * p, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn view_transform_test() {
        // view transform for the default orientation
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        approx::assert_relative_eq!(t, Matrix4D::identity());

        // view transform matrix looking in the positive z direction
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        approx::assert_relative_eq!(t, scaling(-1.0, 1.0, -1.0));

        // view transform moves the world
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        approx::assert_relative_eq!(t, translation(0.0, 0.0, -8.0));

        // an arbitrary view transform
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        approx::assert_relative_eq!(t, Matrix4D::new(
            [[-0.50709254, 0.50709254, 0.6761234, -2.366432], 
            [0.76771593, 0.6060915, 0.12121832, -2.828427], 
            [-0.35856858, 0.59761435, -0.71713716, -2.3841858e-7], 
            [0.0, 0.0, 0.0, 1.0]]), 
            epsilon=1e-6
        );

    }
}