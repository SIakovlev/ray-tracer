#![feature(adt_const_params, generic_const_exprs)]

mod tuple;
mod point;
mod vector;
mod projectile;
mod color;
mod canvas;
mod matrix;
mod transformations;
mod ray;
mod intersection;
mod lights;
mod materials;
mod shapes;
mod world;
mod camera;

use std::cmp;

use crate::{materials::Material, lights::PointLight, transformations::*, world::World, camera::Camera, point::Point, vector::Vector};

fn projectile_example() {
    let start = point::Point::new(0.0, 1.0, 0.0);
    let velocity = vector::Vector::new(1.0, 1.8, 0.0).normalise() * 10.25;

    let mut p = projectile::Projectile {
        position: start,
        velocity: velocity
    };

    let gravity = vector::Vector::new(0.0, -0.1, 0.0);
    let wind = vector::Vector::new(-0.01, 0.0, 0.0);

    let e = projectile::Environment {
        gravity: gravity,
        wind: wind
    };

    let mut c = canvas::Canvas::new(900, 550);
    let color_bullet = color::Color::new(1.0, 0.0, 0.0);

    let mut n_ticks = 0;
    while p.position.tuple.y >= 0.0 {
        println!("Projectile position: {:?}, ticks: {}", p.position, n_ticks);
        c.write_pixel(
            cmp::min(p.position.tuple.x as usize, c.width - 1), 
            c.height - cmp::min(p.position.tuple.y as usize + 1, c.height), 
            color_bullet
        );
        projectile::tick(&e, &mut p);
        n_ticks += 1;
    }
    c.to_ppm(255, "test.ppm")
}

fn sphere_shadow_example() {

    use intersection::hit;
    use shapes::spheres::Sphere;
    use point::Point;
    use color::Color;
    use ray::Ray;
    use shapes::shape::ConcreteShape;

    let canvas_pixels: usize = 1000;
    let mut c = canvas::Canvas::new(canvas_pixels, canvas_pixels);
    let sphere_origin = Point::new(0.0, 0.0, 0.0);
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z: f64 = 100.0;
    let wall_size: f64 = 70.0;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut shape = Sphere::new(sphere_origin);
    shape.set_material(Material::default());
    shape.get_material().color = Color::new(1.0, 0.2, 1.0);

    // let light_position = Point::new(-10.0, 10.0, -10.0);
    // let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalise());
            // compute intersections first
            let mut xs = match shape.intersects(&r) {
                Ok(value) => value,
                Err(err) => panic!("{}", &err),
            };

            // check hits and depending on the result color corresponding pixels
            match hit(&mut xs) {
                Some(hit_value) => {
                    let point = r.position(hit_value.t);
                    let normal = hit_value.object.normal_at(point);
                    let eye = -r.direction;
                    let color = shape.material().lighting(&light, &point, &eye, &normal, false);

                    c.write_pixel(x, y, color);
                },
                _ => (),
            }
        }
    }
    c.to_ppm(255, "sphere_shadow.ppm");

}

fn sphere_scene_example() {
    use shapes::spheres::Sphere;
    use color::Color;
    use shapes::shape::ConcreteShape;
    use std::f64;

    let mut camera = Camera::new(1000.0, 500.0, f64::consts::PI/3.0);
    camera.transform = view_transform(
        Point::new(0.0, 1.5, -5.0), 
        Point::new(0.0, 1.0, 0.0), 
        Vector::new(0.0, 1.0, 0.0));

    // create floor
    let mut floor = Sphere::default();
    floor.set_transform(scaling(10.0, 0.01, 10.0));
    floor.get_material().color = Color::new(1.0, 0.9, 0.9);
    floor.get_material().specular = 0.0;

    // creat left wall
    let mut left_wall = Sphere::default();
    left_wall.set_transform(translation(0.0, 0.0, 5.0) 
        * rotation_y(-f64::consts::PI/4.0) 
        * rotation_x(f64::consts::PI/2.0) 
        * scaling(10.0, 0.01, 10.0));
    left_wall.set_material(*floor.material());

    // create right wall
    let mut right_wall = Sphere::default();
    right_wall.set_transform(translation(0.0, 0.0, 5.0) 
        * rotation_y(f64::consts::PI/4.0) 
        * rotation_x(f64::consts::PI/2.0) 
        * scaling(10.0, 0.01, 10.0));
    right_wall.set_material(*floor.material());

    let mut middle = Sphere::default();
    middle.set_transform(translation(-0.5, 1.0, 0.5));
    middle.get_material().color = Color::new(0.1, 1.0, 0.5);
    middle.get_material().diffuse = 0.7;
    middle.get_material().specular = 0.3;

    let mut right = Sphere::default();
    right.set_transform(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5));
    right.get_material().color = Color::new(0.5, 1.0, 0.1);
    right.get_material().diffuse = 0.7;
    right.get_material().specular = 0.3;

    let mut left = Sphere::default();
    left.set_transform(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
    left.get_material().color = Color::new(1.0, 0.8, 0.1);
    left.get_material().diffuse = 0.7;
    left.get_material().specular = 0.3;

    let light = PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0)
    );

    let world = World::new(vec![floor, left_wall, right_wall, middle, right, left], light);
    match camera.render(&world) {
        Ok(canvas) => canvas.to_ppm(255, "spheres.ppm"),
        Err(err) => println!("{}", err),
    }
    
}

fn main() {
    // projectile_example();
    // sphere_shadow_example();
    sphere_scene_example();
}
