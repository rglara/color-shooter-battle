use opengl_graphics::GlGraphics;
use piston::{Button, Key, RenderArgs, UpdateArgs};

mod bullet;
mod cannon;
mod colors;
mod common;
mod grid;

pub struct App {
    gl: GlGraphics,
    grid: grid::Grid,
    cannons: [cannon::Cannon; 4],
    bullets: Vec<bullet::Bullet>,
    field_rect: [f64; 4],
}

impl App {
    pub fn new(g: GlGraphics) -> App {
        App {
            gl: g,
            grid: grid::Grid::new(),
            cannons: [
                cannon::Cannon::new(1, colors::PLAYER1_CANNON, true, true),
                cannon::Cannon::new(2, colors::PLAYER2_CANNON, false, true),
                cannon::Cannon::new(3, colors::PLAYER3_CANNON, true, false),
                cannon::Cannon::new(4, colors::PLAYER4_CANNON, false, false),
            ],
            bullets: Vec::new(),
            field_rect: [0.0, 0.0, 10.0, 10.0],
        }
    }

    pub const fn get_width() -> i32 {
        let grid_size = common::CELL_WIDTH * common::CELL_EDGES * 2;
        return grid_size + (4 * common::BORDER_SIZE) + (2 * common::SIDE_WIDTH);
    }
    pub const fn get_height() -> i32 {
        let grid_size = common::CELL_WIDTH * common::CELL_EDGES * 2;
        return grid_size + (2 * common::BORDER_SIZE);
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            fn draw_full_column(
                c: &graphics::Context,
                gl: &mut GlGraphics,
                x: i32,
                height: i32,
            ) -> i32 {
                let rect = [x as f64, 0.0, common::BORDER_SIZE as f64, height as f64];
                graphics::rectangle(
                    graphics::color::hex(colors::COLOR_FRAME),
                    rect,
                    c.transform,
                    gl,
                );
                return x + common::BORDER_SIZE;
            }

            fn draw_full_row(
                c: &graphics::Context,
                gl: &mut GlGraphics,
                y: i32,
                width: i32,
            ) -> i32 {
                let rect = [0.0, y as f64, width as f64, common::BORDER_SIZE as f64];
                graphics::rectangle(
                    graphics::color::hex(colors::COLOR_FRAME),
                    rect,
                    c.transform,
                    gl,
                );
                return y + common::BORDER_SIZE;
            }

            // Clear the screen.
            graphics::clear(graphics::color::hex(colors::COLOR_BACKGROUND), gl);

            let window_width = App::get_width();
            let window_height = App::get_height();

            let mut current_x = draw_full_column(&c, gl, 0, window_height);
            current_x = draw_full_column(&c, gl, current_x + common::SIDE_WIDTH, window_height);
            let grid_left = current_x;
            let grid_right = grid_left + (common::CELL_WIDTH * common::CELL_EDGES * 2);
            let grid_top = draw_full_row(&c, gl, 0, window_width);
            let grid_bottom = grid_top + (common::CELL_WIDTH * common::CELL_EDGES * 2);
            self.field_rect = [
                grid_left as f64,
                grid_top as f64,
                (grid_right - grid_left) as f64,
                (grid_bottom - grid_top) as f64,
            ];
            for i in 0..(common::CELL_EDGES * 2) {
                let x = common::CELL_WIDTH + current_x;
                let mut current_y = grid_top;
                for j in 0..(common::CELL_EDGES * 2) {
                    let y = common::CELL_WIDTH + current_y;
                    let rect = [
                        current_x as f64,
                        current_y as f64,
                        common::CELL_WIDTH as f64,
                        common::CELL_WIDTH as f64,
                    ];
                    let color = match self.grid.cells[common::calc_logical_index(i, j)] {
                        1 => graphics::color::hex(colors::PLAYER1_FIELD),
                        2 => graphics::color::hex(colors::PLAYER2_FIELD),
                        3 => graphics::color::hex(colors::PLAYER3_FIELD),
                        4 => graphics::color::hex(colors::PLAYER4_FIELD),
                        _ => graphics::color::hex(colors::COLOR_BACKGROUND),
                    };
                    graphics::rectangle(color, rect, c.transform, gl);
                    current_y = y;
                }
                let xline = [x as f64, grid_top as f64, x as f64, grid_bottom as f64];
                graphics::line(
                    graphics::color::hex(colors::COLOR_GRID),
                    1.0,
                    xline,
                    c.transform,
                    gl,
                );
                current_x = x;
            }

            for y in 0..(common::CELL_EDGES * 2) {
                let axis = (grid_top + (y * common::CELL_WIDTH)) as f64;
                let yline = [grid_left as f64, axis, grid_right as f64, axis];
                graphics::line(
                    graphics::color::hex(colors::COLOR_GRID),
                    1.0,
                    yline,
                    c.transform,
                    gl,
                );
            }

            current_x = draw_full_column(&c, gl, current_x, window_height);
            draw_full_column(&c, gl, current_x + common::SIDE_WIDTH, window_height);
            draw_full_row(&c, gl, grid_bottom, window_width);

            for cannon in &mut self.cannons {
                cannon.draw(&c, gl);
            }
            for bullet in &mut self.bullets {
                bullet.draw(&c, gl);
            }
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        for cannon in &mut self.cannons {
            cannon.turn();
        }
        for bullet in &mut self.bullets {
            bullet.step(self.field_rect);
            if self.grid.check_collision(
                bullet.position[0] - self.field_rect[0],
                bullet.position[1] - self.field_rect[1],
                bullet.cannon_id,
            ) {
                bullet.is_alive = false;
            }
        }
        self.bullets.retain(|b| b.is_alive);
    }

    pub fn handle_button(&mut self, button: &Button) {
        match button {
            Button::Keyboard(key) => match key {
                Key::D1 => {
                    self.fire_cannon(1);
                }
                Key::D2 => {
                    self.fire_cannon(2);
                }
                Key::D3 => {
                    self.fire_cannon(3);
                }
                Key::D4 => {
                    self.fire_cannon(4);
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn fire_cannon(&mut self, cannon_id: i8) {
        for cannon in &mut self.cannons {
            if cannon.id == cannon_id {
                let bullet = cannon.shoot();
                self.bullets.push(bullet);
            }
        }
    }
}
