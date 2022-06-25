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

use std::cmp;

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

    use intersection::{Object, hit};
    use point::Point;
    use color::Color;
    use ray::Ray;

    let canvas_pixels: usize = 100;
    let mut c = canvas::Canvas::new(canvas_pixels, canvas_pixels);

    let sphere_origin = Point::new(0.0, 0.0, 0.0);
    let ray_origin = Point::new(0.0, 0.0, -5.0);

    let wall_z: f32 = 10.0;
    let wall_size: f32 = 7.0;

    let pixel_size = wall_size / canvas_pixels as f32;

    let half = wall_size / 2.0;

    let shape = Object::new(sphere_origin);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f32;
            let position = Point::new(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalise());

            match r.intersection(&shape) {
                Ok(Some(result)) => c.write_pixel(x, y, Color::new(1.0, 0.0, 0.0)),
                _ => (),
            }
        }
    }
    c.to_ppm(255, "sphere_shadow.ppm")

}

fn main() {
    // projectile_example();
    sphere_shadow_example();
}
