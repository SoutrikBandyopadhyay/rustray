use rustray::canvas::to_ppm::*;

use rustray::canvas::*;
use rustray::color;
use rustray::datastruct::*;
use std::fs::write;

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

    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;

    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let mut ball = Projectile::new(pos, vel);
    for i in 0..100 {
        tick(env, &mut ball);

        println!("ball.position = {:?}", ball.position);
        canvas.write_pixel(i, 50, color!(1.0));
    }

    let ppm = canvas.to_ppm();
    write("./output.ppm", ppm).expect("Unable to write");
}
