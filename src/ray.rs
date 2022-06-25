use approx::RelativeEq;

use crate::{
    point::Point, vector::Vector, intersection::{Intersection, Object}, 
    matrix::matrix4d::Matrix4D
};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector
}

impl<'a, 'b> Ray {
    fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }

    fn position(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }

    fn intersection(&'a self, object: &'b Object) -> Result<Option<Vec<Intersection<'b>>>, String> {
        let obj_to_ray = self.origin - object.origin;

        let a = self.direction.dot(&self.direction);
        if a.relative_eq(&0.0, f32::EPSILON, f32::EPSILON) {
            return Err("Direction is zero or close to zero".to_string())
        }


        let b = 2.0 * self.direction.dot(&obj_to_ray);
        let c = obj_to_ray.dot(&obj_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return Ok(None)
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let mut is = Vec::new();
        is.push(Intersection {t: t1, object: object});
        is.push(Intersection {t: t2, object: object});

        Ok(Some(is))
    }

    fn transform(&self, transformation: Matrix4D) -> Self {
        Ray {
            origin: transformation * self.origin, 
            direction: transformation * self.direction 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_test() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        approx::assert_relative_eq!(r.position(0.0), Point::new(2.0, 3.0, 4.0));
        approx::assert_relative_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
        approx::assert_relative_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
        approx::assert_relative_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn unit_sphere_intersection() -> Result<(), String> {
        let obj = Object {origin: Point::new(0.0, 0.0, 0.0), transforms: Vec::new()};
        // ray intersects a unit sphere at two points
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = r.intersection(&obj)?.unwrap();

        approx::assert_relative_eq!(xs[0].t, 4.0);
        approx::assert_relative_eq!(xs[1].t, 6.0);
        assert_eq!(xs[0].object, xs[1].object);


        // ray intersects a unit sphere at a tangent
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = r.intersection(&obj)?.unwrap();

        approx::assert_relative_eq!(xs[0].t, 5.0);
        approx::assert_relative_eq!(xs[1].t, 5.0);
        assert_eq!(xs[0].object, xs[1].object);

        // ray does not intersect a unit sphere
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        assert!(r.intersection(&obj)?.is_none());

        // ray originates inside sphere
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = r.intersection(&obj)?.unwrap();

        approx::assert_relative_eq!(xs[0].t, -1.0);
        approx::assert_relative_eq!(xs[1].t, 1.0);
        assert_eq!(xs[0].object, xs[1].object);

        // A sphere is behind a ray
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = r.intersection(&obj)?.unwrap();

        approx::assert_relative_eq!(xs[0].t, -6.0);
        approx::assert_relative_eq!(xs[1].t, -4.0);
        assert_eq!(xs[0].object, xs[1].object);
        
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Direction is zero or close to zero")]
    fn unit_sphere_intersection_failure() {
        let obj = Object {origin: Point::new(0.0, 0.0, 0.0), transforms: Vec::new()};
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 0.0));
        r.intersection(&obj).unwrap();
    }

    #[test]
    fn transform_test() {
        use crate::transformations::*;
        // translating a ray
        let r1 = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let t = translation(3.0, 4.0, 5.0);

        let r2 = r1.transform(t);

        assert_eq!(r2.origin, Point::new(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Vector::new(0.0, 1.0, 0.0));

        // scaling a ray
        let r1 = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let t = scaling(2.0, 3.0, 4.0);

        let r2 = r1.transform(t);

        assert_eq!(r2.origin, Point::new(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Vector::new(0.0, 3.0, 0.0));
    }
}

