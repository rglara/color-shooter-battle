use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub rotation: f64,  // Rotation for the square.
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}
}
