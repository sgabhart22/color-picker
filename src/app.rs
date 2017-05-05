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
            clear(color::grey(0.9), g);

        let center = c.transform.trans(320.0, 240.0);

        let mut line = line::Line::new(color::BLACK, 1.0);
        let griddy = grid::Grid{ cols: 10, rows: 10, units: 25.0};
        
        griddy.draw(&line, &c.draw_state, center.trans(-130.0, -120.0), g);

        let mut rc: types::ColorComponent = 1.0;
        let mut gc: types::ColorComponent = 0.0;
        let mut bc: types::ColorComponent = 0.0;
        let mut a: types::ColorComponent = 1.0;
        let mut x = 191.0;
        let mut y = 121.0;

        for n in 0..10 {
            let rect = Rectangle::new([rc, gc, bc, a]);
            let dims = rectangle::square(x, y, 23.0);
            rect.draw(dims, &c.draw_state, c.transform, g);
            
            x += 25.0;
            a -= 0.1;
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
