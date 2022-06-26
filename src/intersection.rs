use crate::{point::Point, matrix::matrix4d::Matrix4D, vector::Vector};
use crate::spheres::Sphere;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Sphere
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, obj: &'a Sphere) -> Self {
        Intersection {t: t, object: obj}
    }
}

pub fn hit<'a>(intersections: &'a mut Vec<Intersection<'a>>) -> Option<&Intersection<'a>> {
    intersections.sort_by(|i1, i2| (i1.t).partial_cmp(&i2.t).unwrap());
    let mut iter = intersections
        .iter()
        .skip_while(|x| x.t < 0.0);
    iter.next()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        point::Point, 
        vector::Vector,
        matrix::matrix4d::Matrix4D, 
        ray::Ray, 
        transformations::*
    };
    use super::{Sphere, Intersection, hit};

    #[test]
    fn hit_test() {
        // basic intersection
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0));
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let mut xs = vec![i1, i2];
        let i = hit(&mut xs);
        assert_eq!(i.unwrap(), &i1);

        // intersection where there is one point behind a ray
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let mut xs = vec![i1, i2];
        let i = hit(&mut xs);
        assert_eq!(i.unwrap(), &i2);

        // no intersections
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let mut xs = vec![i1, i2];
        let i = hit(&mut xs);
        assert!(i.is_none());

        // more complex example
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let mut xs = vec![i1, i2, i3, i4];
        let i = hit(&mut xs);
        assert_eq!(i.unwrap(), &i4);
    }

    #[test]
    fn obj_transformations() {
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0));
        let t = Matrix4D::identity();
        assert_eq!(s.transform, t);

        let mut s = Sphere::new(Point::new(0.0, 0.0, 0.0));
        let t = translation(2.0, 3.0, 4.0);
        s.set_transform(t);
        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_scaled_sphere() -> Result<(), String> {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        
        let mut s = Sphere::new(Point::new(0.0, 0.0, 0.0));
        s.set_transform(scaling(2.0, 2.0, 2.0));
        let xs = r.intersection(&s)?;

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);

        // intersection does not modify a ray
        assert_eq!(r.origin, Point::new(0.0, 0.0, -5.0));
        assert_eq!(r.direction, Vector::new(0.0, 0.0, 1.0));

        // intersection with shifted sphere
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        
        let mut s = Sphere::new(Point::new(0.0, 0.0, 0.0));
        s.set_transform(translation(5.0, 0.0, 0.0));
        let xs = r.intersection(&s)?;
        assert_eq!(xs.len(), 0);

        Ok(())
    }
}



