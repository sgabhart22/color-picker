use piston::input::*;
use graphics;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;

use settings;

struct Vec2f {
    x: f64,
    y: f64
}

pub struct App {
    settings: settings::Settings,
    mouse_coords: Vec2f
}

impl App {
    pub fn new(settings: settings::Settings) -> App {
        App {
            settings: settings,
            mouse_coords: Vec2f{ x: 0.0, y: 0.0 }
        }
    } // new

    pub fn on_render(&mut self, args: &RenderArgs,
                     gl: &mut GlGraphics) {
        gl.draw(args.viewport(), |c, g| {
            use graphics::*;
            clear([1.0; 4], g);

        }); // draw
    } // on_render

    pub fn on_button_press(&mut self, button: &Button) {
        
    } // on_button_press

    fn on_mouse_click(&mut self, button: &MouseButton) {
        
    } // on_mouse_click

    pub fn on_mouse_move(&mut self, args: &[f64; 2]) {
        self.mouse_coords.x = args[0];
        self.mouse_coords.y = args[1];
    } // on_mouse_move
} // app
