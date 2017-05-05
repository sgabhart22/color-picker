use piston::input::*;
use graphics;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;

use settings;

const RED:    [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const ORANGE: [f32; 4] = [1.0, 0.6471, 0.0, 1.0];
const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const GREEN:  [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE:   [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const INDIGO: [f32; 4] = [0.5019, 0.0, 0.5019, 1.0];
const VIOLET: [f32; 4] = [0.9333, 0.5098, 0.9333, 1.0];

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
            clear(color::grey(0.9), g);

        let center = c.transform.trans(320.0, 240.0);

        let mut line = line::Line::new(color::BLACK, 1.0);
        let griddy = grid::Grid{ cols: 10, rows: 10, units: 25.0};
        
        griddy.draw(&line, &c.draw_state, center.trans(-130.0, -120.0), g);

        let mut color = RED;
        let mut x = 191.0;
        let mut y = 121.0;

        for m in 0..7		 {
            for n in 0..10 {
            let rect = Rectangle::new(color);
            let dims = rectangle::square(x, y, 23.0);
            rect.draw(dims, &c.draw_state, c.transform, g);
            
            x += 25.0;
            color[3] -= 0.09;
            }

            match m {
                0 => color = ORANGE,
                1 => color = YELLOW,
                2 => color = GREEN,
                3 => color = BLUE,
                4 => color = INDIGO,
                5 => color = VIOLET,
                _ => color = color::WHITE
            }; // match

            x = 191.0;
            y += 25.0;
        }

    
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
