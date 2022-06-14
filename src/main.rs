mod tuple;
mod point;
mod vector;
mod projectile;
mod color;
mod canvas;

fn main() {
    let mut p = projectile::Projectile {
        position: point::Point::new(0.0, 1.0, 0.0),
        velocity: vector::Vector::new(1.0, 1.0, 0.0).normalise()
    };

    let e = projectile::Environment {
        gravity: vector::Vector::new(0.0, -0.1, 0.0),
        wind: vector::Vector::new(-0.01, 0.0, 0.0)
    };

    let mut n_ticks = 0;
    while p.position.tuple.y >= 0.0 {
        println!("Projectile position: {:?}, ticks: {}", p.position, n_ticks);
        projectile::tick(&e, &mut p);
        n_ticks += 1;
    }
}
