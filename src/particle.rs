use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::vec::Vec;
use rand::Rng;

/// This file describes the structure of a particle

#[derive(Debug,Default, Clone)]
pub struct Particle {
    pub id: u32,
    pub _type: i32,
    pub x: f64,
    pub y: f64,
    pub sx: f64,
    pub sy: f64,
    pub links: i64,
    pub bonds: HashSet<Particle>,
}

impl PartialEq for Particle {
    fn eq(&self, other: &Particle) -> bool {
        self.id == other.id
    }
}

impl Hash for Particle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for Particle {}

impl Particle {
    pub fn random(id: u32, types : u32, width : u32, height : u32) -> Particle {

        // Create random type x and y
        let mut rng = rand::thread_rng();

        let mut p = Particle::default();

        p.id = id;
        p._type = rng.gen_range(0, types as i32);
        p.x    = rng.gen_range(0.0, width as f64) as f64;
        p.y    = rng.gen_range(0.0, height as f64);

        p
    }
}

pub struct Link {
    a: Particle,
    b: Particle,
}

impl Link {
    const LINK_FORCE : f64 = -0.015;
}

#[derive(Default, Clone)]
pub struct Field {
    particles: Vec<Particle>,
}

impl Field {
    pub fn add(&mut self, p : Particle) {
        self.particles.push(p);
    }
}
