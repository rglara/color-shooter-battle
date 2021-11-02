use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

const COLOR_BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0]; // black
const COLOR_FRAME: [f32; 4] = [0.2, 0.2, 0.2, 1.0]; // gray
const COLOR_GRID: [f32; 4] = [0.1, 0.1, 0.1, 1.0]; // dark gray
const COLOR_PLAYER_1NW: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; // red
const COLOR_PLAYER_2NE: [f32; 4] = [0.0, 1.0, 0.0, 1.0]; // green
const COLOR_PLAYER_3SW: [f32; 4] = [0.0, 0.0, 1.0, 1.0]; // blue
const COLOR_PLAYER_4SE: [f32; 4] = [1.0, 1.0, 0.0, 1.0]; // yellow

const BORDER_SIZE: i32 = 20;
const SIDE_WIDTH: i32 = 300;
const CELL_WIDTH: i32 = 10;
const CELL_EDGES: i32 = 45;
const CANNON_RADIUS: i32 = 40;

fn calc_index(x: i32, y: i32) -> usize {
    return ((x * CELL_EDGES * 2) + y) as usize;
}

pub struct Grid {
    pub cells: [i8; (CELL_EDGES * CELL_EDGES * 4) as usize],
}

impl Grid {
    pub fn new() -> Grid {
        let mut c = [0; (CELL_EDGES * CELL_EDGES * 4) as usize];
        for y in 0..(CELL_EDGES * 2) {
            for x in 0..(CELL_EDGES * 2) {
                let index = calc_index(x, y);
                if x < CELL_EDGES && y < CELL_EDGES {
                    c[index] = 1;
                } else if x >= CELL_EDGES && y < CELL_EDGES {
                    c[index] = 2;
                } else if x < CELL_EDGES && y >= CELL_EDGES {
                    c[index] = 3;
                } else if x >= CELL_EDGES && y >= CELL_EDGES {
                    c[index] = 4;
                }
            }
        }
        Grid { cells: c }
    }
}

pub struct Cannon {
    pub color: [f32; 4],
    pub x: f64,
    pub y: f64,
    pub min_angle: f64,
    pub max_angle: f64,
}

impl Cannon {
    pub fn draw(&mut self, c: &graphics::Context, gl: &mut GlGraphics) {
        let rect = [
            self.x - (CANNON_RADIUS / 2) as f64,
            self.y - (CANNON_RADIUS / 2) as f64,
            CANNON_RADIUS as f64,
            CANNON_RADIUS as f64,
        ];
        graphics::ellipse(self.color, rect, c.transform, gl);
    }
}

pub struct App {
    gl: GlGraphics,
    grid: Grid,
    cannons: [Cannon; 4],
}

impl App {
    pub fn new(g: GlGraphics) -> App {
        App {
            gl: g,
            grid: Grid::new(),
            cannons: [
                Cannon {
                    color: COLOR_PLAYER_1NW,
                    x: (2 * BORDER_SIZE + SIDE_WIDTH + (CANNON_RADIUS * 3 / 4)) as f64,
                    y: (BORDER_SIZE + (CANNON_RADIUS * 3 / 4)) as f64,
                    min_angle: 0.0,
                    max_angle: 90.0,
                },
                Cannon {
                    color: COLOR_PLAYER_2NE,
                    x: (2 * BORDER_SIZE + SIDE_WIDTH + (CELL_WIDTH * CELL_EDGES * 2)
                        - (CANNON_RADIUS * 3 / 4)) as f64,
                    y: (BORDER_SIZE + (CANNON_RADIUS * 3 / 4)) as f64,
                    min_angle: 0.0,
                    max_angle: 90.0,
                },
                Cannon {
                    color: COLOR_PLAYER_3SW,
                    x: (2 * BORDER_SIZE + SIDE_WIDTH + (CANNON_RADIUS * 3 / 4)) as f64,
                    y: (BORDER_SIZE + (CELL_WIDTH * CELL_EDGES * 2) - (CANNON_RADIUS * 3 / 4))
                        as f64,
                    min_angle: 0.0,
                    max_angle: 90.0,
                },
                Cannon {
                    color: COLOR_PLAYER_4SE,
                    x: (2 * BORDER_SIZE + SIDE_WIDTH + (CELL_WIDTH * CELL_EDGES * 2)
                        - (CANNON_RADIUS * 3 / 4)) as f64,
                    y: (BORDER_SIZE + (CELL_WIDTH * CELL_EDGES * 2) - (CANNON_RADIUS * 3 / 4))
                        as f64,
                    min_angle: 0.0,
                    max_angle: 90.0,
                },
            ],
        }
    }

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
            let grid_left = current_x;
            let grid_right = grid_left + (CELL_WIDTH * CELL_EDGES * 2);
            let grid_top = draw_full_row(&c, gl, 0, window_width);
            let grid_bottom = grid_top + (CELL_WIDTH * CELL_EDGES * 2);
            for i in 0..(CELL_EDGES * 2) {
                let x = CELL_WIDTH + current_x;
                let mut current_y = grid_top;
                for j in 0..(CELL_EDGES * 2) {
                    let y = CELL_WIDTH + current_y;
                    let rect = [
                        current_x as f64,
                        current_y as f64,
                        CELL_WIDTH as f64,
                        CELL_WIDTH as f64,
                    ];
                    let color = match self.grid.cells[calc_index(i, j)] {
                        1 => COLOR_PLAYER_1NW,
                        2 => COLOR_PLAYER_2NE,
                        3 => COLOR_PLAYER_3SW,
                        4 => COLOR_PLAYER_4SE,
                        _ => COLOR_BACKGROUND,
                    };
                    graphics::rectangle(color, rect, c.transform, gl);
                    current_y = y;
                }
                let xline = [x as f64, grid_top as f64, x as f64, grid_bottom as f64];
                graphics::line(COLOR_GRID, 1.0, xline, c.transform, gl);
                current_x = x;
            }

            for y in 0..(CELL_EDGES * 2) {
                let axis = (grid_top + (y * CELL_WIDTH)) as f64;
                let yline = [grid_left as f64, axis, grid_right as f64, axis];
                graphics::line(COLOR_GRID, 1.0, yline, c.transform, gl);
            }

            current_x = draw_full_column(&c, gl, current_x, window_height);
            draw_full_column(&c, gl, current_x + SIDE_WIDTH, window_height);
            draw_full_row(&c, gl, grid_bottom, window_width);

            for cannon in &mut self.cannons {
                cannon.draw(&c, gl);
            }
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}
}
