use std::collections::HashSet;

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
        self == other
    }
}

impl Eq for Particle {}

// impl Default for Particle {
//     fn default() -> Particle {
//         Particle {
//             _type: 0,
//             x: 0.0,
//             y: 0.0,
//             sx: 0.0,
//             sy: 0.0,
//             links: 0,
//             bonds: HashSet::new(),
//         }
//     }
// }
