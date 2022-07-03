use crate::{point::Point, vector::Vector, matrix::matrix4d::Matrix4D, materials::Material};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Sphere {
    pub origin: Point,
    pub transform: Matrix4D,
    pub material: Material,
}

impl Sphere {
    pub fn new(origin: Point) -> Self {
        Self { origin: origin, transform: Matrix4D::identity(), material: Material::default() }
    }

    pub fn set_transform(&mut self, transform: Matrix4D) {
        self.transform = transform;
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        let obj_point = self.transform.inverse().unwrap() * point;
        let obj_normal = (obj_point - self.origin).normalise();
        let mut world_normal = self.transform.inverse().unwrap().transpose() * obj_normal;
        world_normal.tuple.w = 0.0;
        world_normal.normalise()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self { origin: Point::new(0.0, 0.0, 0.0), transform: Matrix4D::identity(), material: Material::default() }
    }
}

#[cfg(test)]
mod tests {
    use crate::{point::Point, vector::Vector, spheres::Sphere, transformations::*, materials::Material};
    use std::f64;

    #[test]
    fn normal_tests() {
        // basic tests
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0));
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
        approx::assert_relative_eq!(n, Vector::new(1.0, 0.0, 0.0));

        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
        approx::assert_relative_eq!(n, Vector::new(0.0, 1.0, 0.0));

        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
        approx::assert_relative_eq!(n, Vector::new(0.0, 0.0, 1.0));

        let n = s.normal_at(Point::new(3.0f64.sqrt() / 3.0, 3.0f64.sqrt() / 3.0, 3.0f64.sqrt() / 3.0));
        approx::assert_relative_eq!(n, Vector::new(3.0f64.sqrt() / 3.0, 3.0f64.sqrt() / 3.0, 3.0f64.sqrt() / 3.0));
        approx::assert_relative_eq!(n, n.normalise());

        // test translated sphere
        let mut s = Sphere::new(Point::new(0.0, 0.0, 0.0));
        s.set_transform(
            translation(0.0, 1.0, 0.0)
        );
        let n = s.normal_at(Point::new(0.0, 1.7071067, -0.7071067));
        approx::assert_relative_eq!(n, Vector::new(0.0, 0.7071067, -0.7071067), epsilon=1e-6);
        
        // test scaled and rotated sphere
        let mut s = Sphere::new(Point::new(0.0, 0.0, 0.0));
        s.set_transform(
            scaling(1.0, 0.5, 1.0) * rotation_z(f64::consts::PI/5.0f64)
        );
        let n = s.normal_at(Point::new(0.0, 2.0f64.sqrt() / 2.0, -2.0f64.sqrt() / 2.0));
        approx::assert_relative_eq!(n, Vector::new(0.0, 0.9701425, -0.24253562), epsilon=1e-6);
    }

    #[test]
    fn material_test() {
        // default material
        let mut s = Sphere::default();
        let mut m = Material::default();
        assert_eq!(&s.material, &m);

        // assign material
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material.ambient, 1.0);
    }
}