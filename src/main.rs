extern crate piston;
extern crate piston_window;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::input::*;
use piston::event_loop::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::path::Path;

mod color;

fn main() {
    let opengl = OpenGL::V2_1;
    let mut window: GlutinWindow =  
        WindowSettings::new("Color Picker",
            [640, 480])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();
    let ref mut gl = GlGraphics::new(opengl);
}
