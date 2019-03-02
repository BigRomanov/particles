extern crate uuid;

extern crate rand;


use crate::particle::Link;
use crate::particle::Particle;
use crate::particle::Field;
use std::vec::Vec;

use uuid::Uuid;

use rand::Rng;

/// Learning Rust project
/// Create life using simple particles

mod particle;

const w : u32 = 1600;
const h : u32 = 1200;

// TODO: Move to separate module to handle all the graphics rendering
const NODE_RADIUS : u32 = 5;
const NODE_COUNT : u32 = 800;
const MAX_DIST : u32 = 100;
const MAX_DIST2 : u32 = MAX_DIST * MAX_DIST;
const SPEED : f32 = 4.0;
const SKIP_FRAMES : u32 = 3;
const BORDER : u32 = 30;

const FW : u32 = w / MAX_DIST + 1;
const FH : u32 = h / MAX_DIST + 1;

const COUPLING :[[i8; 3]; 3] = [[1,1,-1], [1,1,1], [1,1,1]]; 


fn main() {
    println!("Generating particles...");

    let links : Vec<Link> = Vec::new();

    // array for dividing scene into parts to reduce complexity
    let fields = vec![vec![Field::default(); FW as usize]; FH as usize ];

    // 


    let mut p = Particle::default();

    p.x = 1.0;
    p.y = 1.1;
}
