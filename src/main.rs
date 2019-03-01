use crate::particle::Particle;

/// Learning Rust project
/// Create life using simple particles
mod particle;

fn main() {
    println!("Paricles...");

    let p = Particle {};

    p.x = 1.0;
    p.y = 1.1;
}
