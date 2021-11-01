use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

const COLOR_BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0]; // black
const COLOR_FRAME: [f32; 4] = [0.2, 0.2, 0.2, 1.0]; // gray
const COLOR_GRID: [f32; 4] = [0.1, 0.1, 0.1, 1.0]; // dark gray

const BORDER_SIZE: i32 = 20;
const SIDE_WIDTH: i32 = 300;
const CELL_WIDTH: i32 = 10;
const CELL_EDGES: i32 = 45;

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    pub const fn get_width() -> i32 {
        let grid_size = CELL_WIDTH * CELL_EDGES * 2;
        return grid_size + (4 * BORDER_SIZE) + (2 * SIDE_WIDTH);
    }
    pub const fn get_height() -> i32 {
        let grid_size = CELL_WIDTH * CELL_EDGES * 2;
        return grid_size + (2 * BORDER_SIZE);
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            fn draw_full_column(
                c: &graphics::Context,
                gl: &mut GlGraphics,
                x: i32,
                height: i32,
            ) -> i32 {
                let rect = [x as f64, 0.0, BORDER_SIZE as f64, height as f64];
                graphics::rectangle(COLOR_FRAME, rect, c.transform, gl);
                return x + BORDER_SIZE;
            }

            fn draw_full_row(
                c: &graphics::Context,
                gl: &mut GlGraphics,
                y: i32,
                width: i32,
            ) -> i32 {
                let rect = [0.0, y as f64, width as f64, BORDER_SIZE as f64];
                graphics::rectangle(COLOR_FRAME, rect, c.transform, gl);
                return y + BORDER_SIZE;
            }

            // Clear the screen.
            graphics::clear(COLOR_BACKGROUND, gl);

            let window_width = App::get_width();
            let window_height = App::get_height();

            let mut current_x = draw_full_column(&c, gl, 0, window_height);
            current_x = draw_full_column(&c, gl, current_x + SIDE_WIDTH, window_height);
            let current_y = draw_full_row(&c, gl, 0, window_width);

            let grid_left = current_x;
            let grid_right = grid_left + (CELL_WIDTH * CELL_EDGES * 2);
            let grid_top = current_y;
            let grid_bottom = grid_top + (CELL_WIDTH * CELL_EDGES * 2);
            for _i in 0..(CELL_EDGES * 2) {
                let x = CELL_WIDTH + current_x;
                let xline = [x as f64, grid_top as f64, x as f64, grid_bottom as f64];
                graphics::line(COLOR_GRID, 1.0, xline, c.transform, gl);
                for j in 0..(CELL_EDGES * 2) {
                    let y = CELL_WIDTH + ((j + 1) * CELL_WIDTH);
                    let yline = [grid_left as f64, y as f64, grid_right as f64, y as f64];
                    graphics::line(COLOR_GRID, 1.0, yline, c.transform, gl);
                }
                current_x = x;
            }

            current_x = draw_full_column(&c, gl, current_x, window_height);
            draw_full_column(&c, gl, current_x + SIDE_WIDTH, window_height);
            draw_full_row(&c, gl, grid_bottom, window_width);
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}
}
