use std::f64;

use crate::{point::Point, vector::Vector, ray::Ray, intersection::Intersection, 
    shapes::shape::{ConcreteShape, Shape}};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Plane {
    shape: Shape
}

impl Plane {
    pub fn new(origin: Point) -> Self {
        Self { shape: Shape::new(origin) }
    }
}

impl ConcreteShape for Plane {
    
    #[allow(unused_variables)]
    fn local_normal_at(&self, point: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }

    fn local_intersect<'a>(&'a self, ray: Ray) -> Result<Vec<Intersection<'a>>, String> {
        let mut is = Vec::new();
        if ray.direction.tuple.y < f64::EPSILON {
            Ok(is)
        } else {
            let t = -ray.origin.tuple.y / ray.direction.tuple.y;
            is.push(Intersection {t: t, object: self});
            Ok(is)
        }
    }

    fn shape(&self) -> &Shape {
        &self.shape
    }

    fn get_shape(&mut self) -> &mut Shape {
        &mut self.shape
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self { shape: Shape::new(Point::new(0.0, 0.0, 0.0)) }
    }
}