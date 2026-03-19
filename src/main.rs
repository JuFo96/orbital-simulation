use core::fmt;
use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
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
    fn update(&mut self, dt: f64) {
        // Updates position with velocity verlet integration
        let old_acceleration = self.acceleration;
        self.position += self.velocity * dt + self.acceleration * (dt * dt * 0.5);
        self.acceleration = self.apply_forces() / self.mass;
        self.velocity += (self.acceleration + old_acceleration) * (dt * 0.5);
    }

    fn describe_particle(&self) {
        println!(
            "mass: {}, velocity: {}, position: {} acceleration: {}",
            self.mass, self.position, self.velocity, self.acceleration,
        );
    }

    fn apply_forces(&self) -> Vec2 {
        //        let mut forces = Vec2 { x: 0.0, y: 0.0 };
        let gravity = Vec2 { x: -9.8, y: 0.0 };
        return gravity;
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
        sun.update(system.dt);
        if system.iters % 50000 == 0 {
            sun.describe_particle();
        }
        system.time += system.dt;
        system.iters += 1;
    }
    println!("{}", system.iters)
}
