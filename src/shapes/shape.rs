use crate::{matrix::matrix4d::Matrix4D, point::Point, vector::Vector, materials::Material, ray::Ray, intersection::Intersection};

pub trait ConcreteShape<'a, 'b> {
    fn intersects(&'a self, r: &'b Ray) -> Result<Vec<Intersection<'a>>, String>
    {
        let local_ray = r.transform(
            self.transform().inverse().expect("Cannot apply object transformation")
        );
        self.local_intersect(local_ray)
    }

    fn local_intersect(&'a self, ray: Ray) -> Result<Vec<Intersection<'a>>, String>;

    fn normal_at(&self, point: Point) -> Vector {
        let local_point = self.transform().inverse().unwrap() * point;
        let local_normal = self.local_normal_at(local_point);
        let mut world_normal = self.transform().inverse().unwrap().transpose() * local_normal;
        world_normal.tuple.w = 0.0;
        world_normal.normalise()
    }

    fn local_normal_at(&self, point: Point) -> Vector;

    fn transform(&self) -> &Matrix4D;
    fn material(&self) -> &Material;
    fn origin(&self) -> &Point;

    fn get_material(&mut self) -> &mut Material;

    fn set_transform(&mut self, transform: Matrix4D);
    fn set_material(&mut self, material: Material);
    fn set_origin(&mut self, origin: Point);
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Shape {
    pub origin: Point,
    pub transform: Matrix4D,
    pub material: Material,
}

impl<'a, 'b> Shape {
    pub fn new(origin: Point) -> Self {
        Self { origin: origin, transform: Matrix4D::identity(), material: Material::default() }
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self { origin: Point::new(0.0, 0.0, 0.0), transform: Matrix4D::identity(), material: Material::default() }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        point::Point, 
        vector::Vector,
        matrix::matrix4d::Matrix4D, 
        ray::Ray, 
        transformations::*,
        intersection::Intersection, materials::Material, color::Color,
    };

    use super::Shape;

    #[test]
    fn basic_attributes() {
        let mut s = Shape::default();
        // default transform
        assert_eq!(s.transform, Matrix4D::identity());
        
        // assigning a transform
        s.transform = translation(2.0, 3.0, 4.0);
        assert_eq!(&s.transform, &translation(2.0, 3.0, 4.0));
        
        // default material
        assert_eq!(&s.material, &Material::default());

        // assigning a color
        s.material.color = Color::new(0.2, 0.4, 0.3);
        assert_eq!(s.material.color.red, 0.2);
        assert_eq!(s.material.color.green, 0.4);
        assert_eq!(s.material.color.blue, 0.3);

        let mut m = Material::default();
        m.ambient = 1.0;

        s.material = m;
        assert_eq!(s.material, m);
    }
}