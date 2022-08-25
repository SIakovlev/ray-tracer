use crate::{
    shapes::{spheres::Sphere, plane::Plane}, 
    lights::PointLight, 
    point::Point, 
    color::Color, 
    transformations::*, 
    intersection::{IntersectionComputations, hit}, 
    ray::Ray,
    shapes::shape::ConcreteShape,
};

// #[derive(Debug)]
pub struct World {
    pub objects: Vec<Box<dyn ConcreteShape>>,
    pub light: PointLight,
}

impl World {
    pub fn new(objects: Vec<Box<dyn ConcreteShape>>, light: PointLight) -> Self {
        Self { objects: objects, light: light }
    }

    pub fn shade_hit(&self, comps: &IntersectionComputations) -> Color 
    {
        let in_shadow = self.is_shadowed(comps.over_point).unwrap();
        comps.object.material().lighting(comps.object, &self.light, &comps.over_point, &comps.eye, &comps.normal, in_shadow)
    }

    pub fn color_at(&self, ray: &Ray) -> Result<Color, String> {
        let mut xs = ray.intersect_world(self)?;
        let hits = hit(&mut xs);
        let color = match hits {
            Some(intersection) => {
                let comps = ray.prepare_computations(intersection);
                self.shade_hit(&comps)
            },
            _ => Color::new(0.0, 0.0, 0.0)
        };
        Ok(color)
    }

    pub fn is_shadowed(&self, point: Point) -> Result<bool, String> {
        let v = self.light.position - point;
        let distance = v.magnitude();
        let direction = v.normalise();

        let r = Ray::new(point, direction);
        let mut intersections = r.intersect_world(self)?;
        
        match hit(&mut intersections) {
            Some(h) => {
                if h.t < distance {
                    return Ok(true)
                } else {
                    return Ok(false)
                }
            }
            None => return Ok(false)
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut s1 = Sphere::default();
        s1.get_material().color = Color::new(0.8, 1.0, 0.6);
        s1.get_material().diffuse = 0.7;
        s1.get_material().specular = 0.2;

        let mut s2 = Sphere::default();
        s2.set_transform(scaling(0.5, 0.5, 0.5));

        Self { objects: vec![Box::new(s1), Box::new(s2)], light: light }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ray::Ray, point::Point, vector::Vector, color::Color, lights::PointLight, 
        intersection::Intersection, shapes::spheres::Sphere, transformations::translation,
        shapes::shape::ConcreteShape
    };
    use super::World;

    #[test]
    fn default_test() {
        let w = World::default();
        assert_eq!(w.objects[0].material().color, Color::new(0.8, 1.0, 0.6));
        assert_eq!(w.objects[0].material().diffuse, 0.7);
        assert_eq!(w.objects[0].material().specular, 0.2);
    }

    #[test]
    fn intersect_world() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let w = World::default();

        let xs = r.intersect_world(&w).unwrap();

        assert_eq!(xs.len(), 4);
        approx::assert_relative_eq!(xs[0].t, 4.0);
        approx::assert_relative_eq!(xs[1].t, 4.5);
        approx::assert_relative_eq!(xs[2].t, 5.5);
        approx::assert_relative_eq!(xs[3].t, 6.0);
    }
    
    #[test]
    fn shade_hit_test() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let w = World::default();
        let i = Intersection::new(4.0, &*w.objects[0]);
        let computations = r.prepare_computations(&i);
        let c = w.shade_hit(&computations);

        approx::assert_relative_eq!(c, Color::new(0.38066125, 0.4758265, 0.28549594), epsilon=1e-6);

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let mut w = World::default();
        w.light = PointLight::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        let i = Intersection::new(0.5, &*w.objects[1]);
        let computations = r.prepare_computations(&i);
        let c = w.shade_hit(&computations);

        approx::assert_relative_eq!(c, Color::new(0.9049845, 0.9049845, 0.9049845), epsilon=1e-6);

        let s1 = Sphere::default();
        let mut s2 = Sphere::default();
        s2.set_transform(translation(0.0, 0.0, 10.0));
        let w = World::new(
            vec![Box::new(s1), Box::new(s2)],
            PointLight::new(
                Point::new(0.0, 0.0, -10.0), 
                Color::new(1.0, 1.0, 1.0)
            )
        );
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, &*w.objects[1]);
        let comps = r.prepare_computations(&i);
        let c = w.shade_hit(&comps);

        approx::assert_relative_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn color_at_test() {
        // ray misses
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let w = World::default();
        let c = w.color_at(&r).unwrap();

        assert_eq!(c, Color::new(0.0, 0.0, 0.0));

        // ray hits
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let w = World::default();
        let c = w.color_at(&r).unwrap();

        approx::assert_relative_eq!(c, Color::new(0.38066125, 0.4758265, 0.28549594), epsilon=1e-6);

        // ray hits between two spheres
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let mut w = World::default();
        w.objects[0].get_material().ambient = 1.0;
        w.objects[1].get_material().ambient = 1.0;
        let c = w.color_at(&r).unwrap();

        assert_eq!(c, w.objects[1].material().color);
    }

    #[test]
    fn is_shadowed_test() {
        let w = World::default();

        // no shadow when nothing is collinear with point and light
        let p = Point::new(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(p).unwrap());

        // the shadow when an object is between the point and the light
        let p = Point::new(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(p).unwrap());

        // no shadow when an object is behind the light
        let p = Point::new(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(p).unwrap());

        // no shadow when an object is behind the point
        let p = Point::new(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(p).unwrap());
    }

}