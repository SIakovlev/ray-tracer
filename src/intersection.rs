use crate::{point::Point, matrix::matrix4d::Matrix4D};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Object {
    pub origin: Point,
    pub transforms: Vec<Matrix4D>
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Object
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, obj: &'a Object) -> Self {
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

    use crate::point::Point;
    use super::{Object, Intersection, hit};

    #[test]
    fn hit_test() {
        // basic intersection
        let s = Object{origin: Point::new(0.0, 0.0, 0.0), transforms: Vec::new()};
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
}



