use graphics::{Radians, Transformed};
use opengl_graphics::GlGraphics;
use piston::{Button, Key, RenderArgs, UpdateArgs};

mod colors;

const BORDER_SIZE: i32 = 20;
const SIDE_WIDTH: i32 = 300;
const CELL_WIDTH: i32 = 10;
const CELL_EDGES: i32 = 45;

fn calc_logical_index(x: i32, y: i32) -> usize {
    return ((x * CELL_EDGES * 2) + y) as usize;
}

fn calc_physical_index(xpos: f64, ypos: f64) -> usize {
    let x = xpos / CELL_WIDTH as f64;
    let y = ypos / CELL_WIDTH as f64;
    return calc_logical_index(x as i32, y as i32);
}

pub struct Grid {
    pub cells: [i8; (CELL_EDGES * CELL_EDGES * 4) as usize],
}

impl Grid {
    pub fn new() -> Grid {
        let mut c = [0; (CELL_EDGES * CELL_EDGES * 4) as usize];
        for y in 0..(CELL_EDGES * 2) {
            for x in 0..(CELL_EDGES * 2) {
                let index = calc_logical_index(x, y);
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

    pub fn check_collision(&mut self, x: f64, y: f64, cannon_id: i8) -> bool {
        let index = calc_physical_index(x, y);
        if self.cells[index] != cannon_id {
            self.cells[index] = cannon_id;
            return true;
        }
        return false;
    }
}

pub struct Bullet {
    pub cannon_id: i8,
    color: [f32; 4],
    pub position: [f64; 2],
    angle: f64,
    speed: f64,
    pub is_alive: bool,
}

impl Bullet {
    const RADIUS: f64 = 10.0;
    const SPEED: f64 = 1.0;

    pub fn new(cannon_id: i8, color: [f32; 4], x: f64, y: f64, angle: f64) -> Bullet {
        Bullet {
            cannon_id: cannon_id,
            color: color,
            position: [x, y],
            angle: angle,
            speed: Bullet::SPEED,
            is_alive: true,
        }
    }

    pub fn step(&mut self, grid_rect: [f64; 4]) {
        let x = self.speed * self.angle.deg_to_rad().cos();
        let y = self.speed * self.angle.deg_to_rad().sin();
        self.position = [self.position[0] + x, self.position[1] + y];
        self.check_boundary_collisions(grid_rect);
    }

    pub fn draw(&mut self, c: &graphics::Context, gl: &mut GlGraphics) {
        let rect = [
            self.position[0],
            self.position[1],
            Bullet::RADIUS,
            Bullet::RADIUS,
        ];
        graphics::ellipse(self.color, rect, c.transform, gl);
    }

    fn check_boundary_collisions(&mut self, grid_rect: [f64; 4]) {
        if (self.position[0] < grid_rect[0])
            || ((self.position[0] + Bullet::RADIUS) > (grid_rect[0] + grid_rect[2]))
        {
            self.angle = 180.0 - self.angle;
        }
        if (self.position[1] < grid_rect[1])
            || ((self.position[1] + Bullet::RADIUS) > (grid_rect[1] + grid_rect[3]))
        {
            self.angle = 360.0 - self.angle;
        }
    }
}

pub struct Cannon {
    id: i8,
    color: [f32; 4],
    x: f64,
    y: f64,
    min_angle_deg: f64,
    max_angle_deg: f64,
    current_angle_deg: f64,
    current_barrel_move: f64,
}

impl Cannon {
    const SPEED: f64 = 0.4;
    const SWEEP: f64 = 60.0;
    const RADIUS: i32 = 40;

    pub fn new(id: i8, hex: &str, is_left: bool, is_top: bool) -> Cannon {
        let mut h = 2 * BORDER_SIZE + SIDE_WIDTH;
        if is_left {
            h += Cannon::RADIUS * 3 / 4;
        } else {
            h += (CELL_WIDTH * CELL_EDGES * 2) - (Cannon::RADIUS * 3 / 4);
        }
        let mut v = BORDER_SIZE;
        if is_top {
            v += Cannon::RADIUS * 3 / 4;
        } else {
            v += (CELL_WIDTH * CELL_EDGES * 2) - (Cannon::RADIUS * 3 / 4);
        }
        let neutral: f64;
        if is_left {
            if is_top {
                neutral = 45.0;
            } else {
                neutral = 315.0;
            }
        } else {
            if is_top {
                neutral = 135.0;
            } else {
                neutral = 225.0;
            }
        }
        let min = neutral - Cannon::SWEEP;
        let max = neutral + Cannon::SWEEP;

        Cannon {
            id: id,
            color: graphics::color::hex(hex),
            x: h as f64,
            y: v as f64,
            min_angle_deg: min,
            max_angle_deg: max,
            current_angle_deg: min,
            current_barrel_move: Cannon::SPEED,
        }
    }

    pub fn turn(&mut self) {
        self.current_angle_deg += self.current_barrel_move;
        if self.current_angle_deg >= self.max_angle_deg {
            self.current_angle_deg = self.max_angle_deg;
            self.current_barrel_move = -Cannon::SPEED;
        } else if self.current_angle_deg <= self.min_angle_deg {
            self.current_angle_deg = self.min_angle_deg;
            self.current_barrel_move = Cannon::SPEED;
        }
    }

    pub fn draw(&mut self, c: &graphics::Context, gl: &mut GlGraphics) {
        let base = [
            self.x - (Cannon::RADIUS / 2) as f64,
            self.y - (Cannon::RADIUS / 2) as f64,
            Cannon::RADIUS as f64,
            Cannon::RADIUS as f64,
        ];
        graphics::ellipse(self.color, base, c.transform, gl);
        let barrel = [
            0.0,
            0.0,
            (3 * Cannon::RADIUS / 4) as f64,
            (Cannon::RADIUS / 3) as f64,
        ];
        let barrel_transform = c
            .transform
            .trans(self.x, self.y)
            .rot_deg(self.current_angle_deg)
            .trans(0.0, (Cannon::RADIUS / -8) as f64);
        graphics::rectangle(self.color, barrel, barrel_transform, gl);
    }

    pub fn shoot(&mut self) -> Bullet {
        Bullet::new(self.id, self.color, self.x, self.y, self.current_angle_deg)
    }
}

pub struct App {
    gl: GlGraphics,
    grid: Grid,
    cannons: [Cannon; 4],
    bullets: Vec<Bullet>,
    field_rect: [f64; 4],
}

impl App {
    pub fn new(g: GlGraphics) -> App {
        App {
            gl: g,
            grid: Grid::new(),
            cannons: [
                Cannon::new(1, colors::PLAYER1_CANNON, true, true),
                Cannon::new(2, colors::PLAYER2_CANNON, false, true),
                Cannon::new(3, colors::PLAYER3_CANNON, true, false),
                Cannon::new(4, colors::PLAYER4_CANNON, false, false),
            ],
            bullets: Vec::new(),
            field_rect: [0.0, 0.0, 10.0, 10.0],
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
                graphics::rectangle(colors::COLOR_FRAME, rect, c.transform, gl);
                return x + BORDER_SIZE;
            }

            fn draw_full_row(
                c: &graphics::Context,
                gl: &mut GlGraphics,
                y: i32,
                width: i32,
            ) -> i32 {
                let rect = [0.0, y as f64, width as f64, BORDER_SIZE as f64];
                graphics::rectangle(colors::COLOR_FRAME, rect, c.transform, gl);
                return y + BORDER_SIZE;
            }

            // Clear the screen.
            graphics::clear(colors::COLOR_BACKGROUND, gl);

            let window_width = App::get_width();
            let window_height = App::get_height();

            let mut current_x = draw_full_column(&c, gl, 0, window_height);
            current_x = draw_full_column(&c, gl, current_x + SIDE_WIDTH, window_height);
            let grid_left = current_x;
            let grid_right = grid_left + (CELL_WIDTH * CELL_EDGES * 2);
            let grid_top = draw_full_row(&c, gl, 0, window_width);
            let grid_bottom = grid_top + (CELL_WIDTH * CELL_EDGES * 2);
            self.field_rect = [
                grid_left as f64,
                grid_top as f64,
                (grid_right - grid_left) as f64,
                (grid_bottom - grid_top) as f64,
            ];
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
                    let color = match self.grid.cells[calc_logical_index(i, j)] {
                        1 => graphics::color::hex(colors::PLAYER1_FIELD),
                        2 => graphics::color::hex(colors::PLAYER2_FIELD),
                        3 => graphics::color::hex(colors::PLAYER3_FIELD),
                        4 => graphics::color::hex(colors::PLAYER4_FIELD),
                        _ => colors::COLOR_BACKGROUND,
                    };
                    graphics::rectangle(color, rect, c.transform, gl);
                    current_y = y;
                }
                let xline = [x as f64, grid_top as f64, x as f64, grid_bottom as f64];
                graphics::line(colors::COLOR_GRID, 1.0, xline, c.transform, gl);
                current_x = x;
            }

            for y in 0..(CELL_EDGES * 2) {
                let axis = (grid_top + (y * CELL_WIDTH)) as f64;
                let yline = [grid_left as f64, axis, grid_right as f64, axis];
                graphics::line(colors::COLOR_GRID, 1.0, yline, c.transform, gl);
            }

            current_x = draw_full_column(&c, gl, current_x, window_height);
            draw_full_column(&c, gl, current_x + SIDE_WIDTH, window_height);
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
