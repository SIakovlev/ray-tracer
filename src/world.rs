use crate::{
    spheres::Sphere, 
    lights::PointLight, 
    point::Point, 
    color::Color, 
    transformations::*, 
    intersection::{IntersectionComputations, hit}, ray::Ray
};

#[derive(Debug)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub light: PointLight,
}

impl World {
    pub fn new(objects: Vec<Sphere>, light: PointLight) -> Self {
        Self { objects: objects, light: light }
    }

    pub fn shade_hit(&self, comps: &IntersectionComputations) -> Color {
        comps.object.material.lighting(&self.light, &comps.point, &comps.eye, &comps.normal)
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
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut s1 = Sphere::default();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::default();
        s2.set_transform(scaling(0.5, 0.5, 0.5));

        Self { objects: vec![s1, s2], light: light }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ray::Ray, point::Point, vector::Vector, color::Color, lights::PointLight, intersection::Intersection};
    use super::World;

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
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, shape);
        let computations = r.prepare_computations(&i);
        let c = w.shade_hit(&computations);

        approx::assert_relative_eq!(c, Color::new(0.38066125, 0.4758265, 0.28549594));

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let mut w = World::default();
        w.light = PointLight::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape);
        let computations = r.prepare_computations(&i);
        let c = w.shade_hit(&computations);

        approx::assert_relative_eq!(c, Color::new(0.9049845, 0.9049845, 0.9049845));
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

        assert_eq!(c, Color::new(0.38066125, 0.4758265, 0.28549594));

        // ray hits between two spheres
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let mut w = World::default();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;
        let c = w.color_at(&r).unwrap();

        assert_eq!(c, w.objects[1].material.color);
    }

    
}