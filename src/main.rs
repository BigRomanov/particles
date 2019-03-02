use crate::particle::Particle;

/// Learning Rust project
/// Create life using simple particles
mod particle;

fn main() {
    println!("Paricles...");

    let w = 1600; // width
    let h = 1200; // height

    let mut p = Particle::default();

    p.x = 1.0;
    p.y = 1.1;
}
