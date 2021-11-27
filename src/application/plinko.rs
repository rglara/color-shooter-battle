use graphics::{Radians, Text, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};
use rand::Rng;

struct Puck {
    position: [f64; 2],
    angle: f64,
    speed: f64,
    color: [f32; 4],
    is_alive: bool,
}

impl Puck {
    const RADIUS: f64 = 18.0;
    const GRAVITY: f64 = 0.3;

    pub fn new_fixed(pos: [f64; 2]) -> Puck {
        Puck {
            position: pos,
            angle: 0.0,
            speed: -1.0,
            color: graphics::color::hex(super::colors::FRAME),
            is_alive: true,
        }
    }

    pub fn new_active(pos: [f64; 2], color: [f32; 4]) -> Puck {
        Puck {
            position: pos,
            angle: 90.0,
            speed: 0.1,
            color: color,
            is_alive: true,
        }
    }

    pub fn draw(&self, c: &graphics::Context, gl: &mut GlGraphics) {
        let rect = [
            self.position[0] - Puck::RADIUS,
            self.position[1] - Puck::RADIUS,
            Puck::RADIUS * 2.0,
            Puck::RADIUS * 2.0,
        ];
        graphics::ellipse(self.color, rect, c.transform, gl);
    }

    pub fn step(&mut self) {
        if self.speed > 0.0 {
            let x = self.speed * self.angle.deg_to_rad().cos();
            let y = self.speed * self.angle.deg_to_rad().sin() + Puck::GRAVITY;
            self.position = [self.position[0] + x, self.position[1] + y];
        }
    }

    pub fn collides_with(&self, rect: [f64; 4]) -> bool {
        let center_rect_x = rect[0] + (rect[2] / 2.0);
        let center_rect_y = rect[1] + (rect[3] / 2.0);
        let distance_x = (self.position[0] - center_rect_x).abs();
        let distance_y = (self.position[1] - center_rect_y).abs();
        if distance_x > ((rect[2] / 2.0) + Puck::RADIUS) {
            return false;
        }
        if distance_y > ((rect[3] / 2.0) + Puck::RADIUS) {
            return false;
        }
        if distance_x <= (rect[2] / 2.0) {
            return true;
        }
        if distance_y <= (rect[3] / 2.0) {
            return true;
        }
        let x_portion = distance_x - (rect[2] / 2.0);
        let y_portion = distance_y - (rect[3] / 2.0);
        let distance = x_portion.hypot(y_portion);
        return distance <= Puck::RADIUS;
    }
}

pub struct PlinkoEvent {
    pub id: i8,
    pub num_shots: i32,
}

impl PlinkoEvent {
    pub fn new(id: i8, num_shots: i32) -> PlinkoEvent {
        PlinkoEvent { id, num_shots }
    }
}

pub struct Plinko {
    id: i8,
    position: [f64; 2],
    color: [f32; 4],
    pins: Vec<Puck>,
    pucks: Vec<Puck>,
    time: f64,
    well_x: f64,
    shot_count: i32,
    pub is_alive: bool,
}

impl Plinko {
    const BOUNDARY_WIDTH: f64 = 10.0;
    const WELL_DIVIDER_WIDTH: f64 = 20.0;
    const MIN_WELL_WIDTH: f64 = 100.0;
    const NEW_PUCK_TIME: f64 = 20.0;
    const WELL_DEPTH: f64 = 20.0;
    const MAX_PUCKS: usize = 10;
    const SCORE_SIZE: u32 = 42;

    pub fn new(id: i8, color: &str, position: [f64; 2]) -> Plinko {
        // stationary pucks are "pins" to bounce off of
        let mut pins = Vec::new();
        const HORZ: i32 = 3;
        const HSPACE: f64 =
            ((super::common::SIDE_WIDTH - (Plinko::BOUNDARY_WIDTH * 2.0) as i32) / HORZ) as f64;
        const VERT: i32 = 5;
        const VSPACE: f64 = ((super::common::CELL_EDGES * super::common::CELL_WIDTH) / VERT) as f64;
        for v in 1..VERT {
            let mut max = HORZ + 1;
            let mut offset = 0.0;
            if v % 2 == 0 {
                max = max - 1;
                offset = HSPACE / 2.0;
            }
            for h in 0..max {
                pins.push(Puck::new_fixed([
                    position[0] + Plinko::BOUNDARY_WIDTH + (HSPACE * h as f64) + offset,
                    position[1] + (VSPACE * v as f64) - Puck::RADIUS,
                ]));
            }
        }

        Plinko {
            id: id,
            position: position,
            color: graphics::color::hex(color),
            pins: pins,
            pucks: Vec::new(),
            time: 0.0,
            well_x: position[0] + super::common::SIDE_WIDTH as f64
                - Plinko::BOUNDARY_WIDTH
                - Plinko::MIN_WELL_WIDTH,
            shot_count: 1,
            is_alive: true,
        }
    }

    pub fn update<F>(&mut self, delta_time: f64, event_callback: F)
    where
        F: FnMut(PlinkoEvent),
    {
        if self.is_alive {
            self.time += delta_time;

            // add pucks gradually over time
            let num_pucks = self.pucks.len();
            if (self.time / Plinko::NEW_PUCK_TIME) as usize >= num_pucks
                && num_pucks < Plinko::MAX_PUCKS
            {
                let mut rng = rand::thread_rng();
                let random_x = rng.gen_range(
                    0.0..(super::common::SIDE_WIDTH as f64
                        - (Puck::RADIUS * 2.0)
                        - (Plinko::BOUNDARY_WIDTH * 2.0)),
                );
                self.pucks.push(Puck::new_active(
                    [
                        self.position[0] + Puck::RADIUS + Plinko::BOUNDARY_WIDTH + random_x,
                        self.position[1] + Plinko::BOUNDARY_WIDTH + (3.0 * Puck::RADIUS / 2.0),
                    ],
                    self.color,
                ));
            }

            // gradually slide well divider to multiply more
            let new_well_x = self.well_x - self.time / 100000.0;
            let min_well_x = self.position[0] + Plinko::BOUNDARY_WIDTH + Plinko::MIN_WELL_WIDTH;
            self.well_x = new_well_x.max(min_well_x);

            // move pucks
            for puck in &mut self.pucks {
                puck.step();
            }
            self.check_collisions(event_callback);
        }
    }

    fn get_min_max(&self) -> [f64; 4] {
        return [
            self.position[0],
            self.position[0] + super::common::SIDE_WIDTH as f64,
            self.position[1],
            self.position[1] + (super::common::CELL_WIDTH * super::common::CELL_EDGES) as f64,
        ];
    }

    fn get_fire_rect(&self) -> [f64; 4] {
        let [xmin, _xmax, _ymin, ymax] = self.get_min_max();
        return [
            xmin,
            ymax - Plinko::WELL_DEPTH / 2.0 - Plinko::BOUNDARY_WIDTH,
            self.well_x - xmin,
            Plinko::WELL_DEPTH / 2.0,
        ];
    }

    fn get_multi_rect(&self) -> [f64; 4] {
        let [_xmin, xmax, _ymin, ymax] = self.get_min_max();
        return [
            self.well_x,
            ymax - Plinko::WELL_DEPTH / 2.0 - Plinko::BOUNDARY_WIDTH,
            xmax - self.well_x,
            Plinko::WELL_DEPTH / 2.0,
        ];
    }

    pub fn draw(&self, c: &graphics::Context, gl: &mut GlGraphics, glyphs: &mut GlyphCache) {
        let text_transform = c.transform.trans(
            self.position[0] + Plinko::BOUNDARY_WIDTH + Plinko::SCORE_SIZE as f64 * 0.3,
            self.position[1] + Plinko::BOUNDARY_WIDTH + Plinko::SCORE_SIZE as f64 * 0.9,
        );
        Text::new_color(self.color, Plinko::SCORE_SIZE)
            .draw(
                &self.shot_count.to_string(),
                glyphs,
                &c.draw_state,
                text_transform,
                gl,
            )
            .expect("Unable to render text");

        for pin in &self.pins {
            pin.draw(&c, gl);
        }

        graphics::rectangle(
            graphics::color::hex(super::colors::FIRE_WELL),
            self.get_fire_rect(),
            c.transform,
            gl,
        );
        graphics::rectangle(
            graphics::color::hex(super::colors::MULTI_WELL),
            self.get_multi_rect(),
            c.transform,
            gl,
        );
        let [xmin, xmax, ymin, ymax] = self.get_min_max();
        graphics::rectangle(
            graphics::color::hex(super::colors::FRAME),
            [
                self.well_x - Plinko::WELL_DIVIDER_WIDTH / 2.0,
                ymax - Plinko::WELL_DEPTH - Plinko::BOUNDARY_WIDTH,
                Plinko::WELL_DIVIDER_WIDTH,
                Plinko::WELL_DEPTH,
            ],
            c.transform,
            gl,
        );

        graphics::rectangle(
            graphics::color::hex(super::colors::FRAME),
            [xmin, ymin, xmax - xmin, Plinko::BOUNDARY_WIDTH],
            c.transform,
            gl,
        );
        graphics::rectangle(
            graphics::color::hex(super::colors::FRAME),
            [
                xmin,
                ymax - Plinko::BOUNDARY_WIDTH,
                xmax - xmin,
                Plinko::BOUNDARY_WIDTH,
            ],
            c.transform,
            gl,
        );
        graphics::rectangle(
            graphics::color::hex(super::colors::FRAME),
            [xmin, ymin, Plinko::BOUNDARY_WIDTH, ymax - ymin],
            c.transform,
            gl,
        );
        graphics::rectangle(
            graphics::color::hex(super::colors::FRAME),
            [
                xmax - Plinko::BOUNDARY_WIDTH,
                ymin,
                Plinko::BOUNDARY_WIDTH,
                ymax - ymin,
            ],
            c.transform,
            gl,
        );

        for puck in &self.pucks {
            puck.draw(&c, gl);
        }
    }

    fn check_collisions<F>(&mut self, mut event_callback: F)
    where
        F: FnMut(PlinkoEvent),
    {
        // pucks with pins

        // pucks with wells
        let multi_rect = self.get_multi_rect();
        let fire_rect = self.get_fire_rect();
        for puck in &mut self.pucks {
            if puck.collides_with(multi_rect) {
                self.shot_count *= 2;
                puck.is_alive = false;
            } else if puck.collides_with(fire_rect) {
                event_callback(PlinkoEvent::new(self.id, self.shot_count));
                self.shot_count = 1;
                puck.is_alive = false;
            }
        }
        self.pucks.retain(|p| p.is_alive);
    }
}
