use rustray::datastruct::*;

#[derive(Debug, Copy, Clone)]
struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple,
}
#[derive(Debug, Copy, Clone)]
struct Environment {
    pub wind: Tuple,
    pub gravity: Tuple,
}

impl Projectile {
    pub fn new(position: Tuple, velocity: Tuple) -> Self {
        Self { position, velocity }
    }
}

impl Environment {
    pub fn new(wind: Tuple, gravity: Tuple) -> Self {
        Self { wind, gravity }
    }
}

fn tick(env: Environment, proj: &mut Projectile) {
    proj.position = proj.position + proj.velocity * 0.01;
    proj.velocity = proj.velocity + env.gravity + env.wind;
}
pub fn main() {
    let pos = Tuple::point(0.0, 20.0, 0.0);
    let vel = Tuple::vector(10.0, 0.0, 0.0);
    let gravity = Tuple::vector(0.0, -9.8, 0.0);
    let wind = Tuple::vector(-0.01, 0.0, 0.0);
    let env = Environment::new(wind, gravity);

    let mut ball = Projectile::new(pos, vel);
    for _ in 0..100 {
        tick(env, &mut ball);
        println!("ball.position = {:?}", ball.position);
    }
}
