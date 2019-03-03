extern crate uuid;

extern crate rand;


use crate::particle::Link;
use crate::particle::Particle;
use crate::particle::Field;
use std::vec::Vec;

// Graphics subsystem (piston)
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use uuid::Uuid;


/// Learning Rust project
/// Create life using simple particles

mod particle;

const SCENE_WIDTH : u32 = 1600;
const SCENE_HEIGHT : u32 = 1200;

// TODO: Move to separate module to handle all the graphics rendering
const NODE_RADIUS : u32 = 5;
const NODE_COUNT : u32 = 800;
const MAX_DIST : u32 = 100;
const MAX_DIST2 : u32 = MAX_DIST * MAX_DIST;
const SPEED : f32 = 4.0;
const SKIP_FRAMES : u32 = 3;
const BORDER : u32 = 30;

const FW : u32 = SCENE_WIDTH / MAX_DIST;
const FH : u32 = SCENE_HEIGHT / MAX_DIST;

const COUPLING :[[i8; 3]; 3] = [[1,1,-1], [1,1,1], [1,1,1]]; 


pub struct App {
	gl: GlGraphics,
	particles : Vec<Particle>,
}


impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];


        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x, y) = (args.width / 2.0,
                      args.height / 2.0);

        let particles = &self.particles;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // let transform = c.transform.trans(x, y).rot_rad(rotation).trans(-25.0, -25.0);
            // rectangle(RED, square, transform, gl);

            for p in particles {
            	ellipse(RED, rectangle::square(p.x, p.y, 10.0), c.transform, gl)
            }
            
        });
    }

    fn update(&mut self, args: &UpdateArgs) {

    }
}


fn main() {
    println!("Generating particles...");

    let links : Vec<Link> = Vec::new();

    // array for dividing scene into parts to reduce complexity
    //let mut fields = vec![vec![Field::default(); FH as usize]; FW as usize ];

    let mut particles : Vec<Particle> = Vec::new();

    // Populate the scene with random particles
    for n in 0..NODE_COUNT {
    	println!("Generating particle with id: {}", n);

    	let p = Particle::random(n, COUPLING.len() as u32, SCENE_WIDTH, SCENE_HEIGHT);

    	particles.push(p.clone());


    	// Add particle to field
    	// TODO: Check the types of particle coordinates, do they really need to be float?
    	//fields[(p.x as u32 / MAX_DIST) as usize ][(p.y as u32 / MAX_DIST) as usize ].add(p);

    }

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Particles of Life",
            [SCENE_WIDTH, SCENE_HEIGHT]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new simulation and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        particles: particles
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }


}
