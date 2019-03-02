use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::vec::Vec;

/// This file describes the structure of a particle

#[derive(Default)]
pub struct Particle {
    pub id: i32,
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

pub struct Link {
    a: Particle,
    b: Particle,
}

pub struct Field {
    particles: Vec<Particle>,
}
