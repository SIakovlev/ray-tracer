use crate::vector::Vector;
use crate::point::Point;

#[derive(Debug)]
pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

pub fn tick(env: &Environment, proj: &mut Projectile) {
    proj.position = proj.position + proj.velocity;
    proj.velocity = proj.velocity + env.gravity + env.wind;
}