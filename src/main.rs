use core::fmt;
use std::ops::Mul;

struct Vec2 {
    x: f64,
    y: f64,
}
impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

struct Particle {
    mass: f64,
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
}

struct System {
    time: f64,
    dt: f64,
    iters: i64,
}
struct Particles {
    //Probably create a struct that contains a collection of all particles for easier looping over
    //particle particle interactions
}
impl Particle {
    fn update_position(&mut self, dt: f64) {
        // Updates position with verlet integration
        self.position.x += self.velocity.x * dt + self.acceleration.x * (dt * dt * 0.5);
        self.position.y += self.velocity.y * dt + self.acceleration.y * (dt * dt * 0.5);
    }
    fn describe_particle(&self) {
        println!(
            "mass: {}, velocity: {}, position: {} acceleration: {}",
            self.mass, self.position, self.velocity, self.acceleration,
        );
    }
}

fn main() {
    let mut system = System {
        time: 0.0,
        dt: 0.0001,
        iters: 0,
    };
    let mut sun = Particle {
        mass: 10.0,
        position: Vec2 { x: 2.0, y: 2.0 },
        velocity: Vec2 { x: 2.0, y: 2.0 },
        acceleration: Vec2 { x: 1.0, y: -1.0 },
    };

    while system.time < 10.0 {
        sun.update_position(system.dt);
        if system.iters % 50000 == 0 {
            sun.describe_particle();
        }
        system.time += system.dt;
        system.iters += 1;
    }
    println!("{}", system.iters)
}
