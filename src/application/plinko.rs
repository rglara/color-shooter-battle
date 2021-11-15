use graphics::Radians;
use opengl_graphics::GlGraphics;

struct Puck {
    position: [f64; 2],
    angle: f64,
    speed: f64,
    color: [f32; 4],
}

impl Puck {
    const RADIUS: f64 = 36.0;
    const GRAVITY: f64 = 0.3;

    pub fn new_fixed(pos: [f64; 2]) -> Puck {
        Puck {
            position: [pos[0] - Puck::RADIUS / 2.0, pos[1] - Puck::RADIUS / 2.0],
            angle: 0.0,
            speed: -1.0,
            color: graphics::color::hex(super::colors::FRAME),
        }
    }

    pub fn new_active(pos: [f64; 2], color: [f32; 4]) -> Puck {
        Puck {
            position: [pos[0] - Puck::RADIUS / 2.0, pos[1] - Puck::RADIUS / 2.0],
            angle: 90.0,
            speed: 0.1,
            color: color,
        }
    }

    pub fn draw(&mut self, c: &graphics::Context, gl: &mut GlGraphics) {
        let rect = [
            self.position[0],
            self.position[1],
            Puck::RADIUS,
            Puck::RADIUS,
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
}

pub struct Plinko {
    id: i32,
    position: [f64; 2],
    color: [f32; 4],
    pins: Vec<Puck>,
    pucks: Vec<Puck>,
    time: f64,
    well_x: f64,
}

impl Plinko {
    const BOUNDARY_WIDTH: f64 = 10.0;
    const WELL_WIDTH: f64 = 20.0;
    const NEW_PUCK_TIME: f64 = 10.0;
    const WELL_DEPTH: f64 = 20.0;

    pub fn new(id: i32, color: &str, position: [f64; 2]) -> Plinko {
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
                    position[1] + (VSPACE * v as f64) - Puck::RADIUS / 2.0,
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
            well_x: position[0]
                + Plinko::BOUNDARY_WIDTH
                + (2.0 * super::common::SIDE_WIDTH as f64 / 3.0),
        }
    }

    fn check_collisions(&mut self) {}

    pub fn update(&mut self, delta_time: f64) {
        self.time += delta_time;
        if (self.time / Plinko::NEW_PUCK_TIME) as usize >= self.pucks.len() {
            self.pucks.push(Puck::new_active(
                [
                    self.position[0] + (super::common::SIDE_WIDTH / 2) as f64,
                    self.position[1] + Plinko::BOUNDARY_WIDTH + (3.0 * Puck::RADIUS / 4.0),
                ],
                self.color,
            ));
        }
        for puck in &mut self.pucks {
            puck.step();
        }
        self.check_collisions();
    }

    pub fn draw(&mut self, c: &graphics::Context, gl: &mut GlGraphics) {
        let xmin = self.position[0];
        let xmax = xmin + super::common::SIDE_WIDTH as f64;
        let ymin = self.position[1];
        let ymax = ymin + (super::common::CELL_WIDTH * super::common::CELL_EDGES) as f64;

        for pin in &mut self.pins {
            pin.draw(&c, gl);
        }

        graphics::rectangle(
            graphics::color::hex(super::colors::FIRE_WELL),
            [
                xmin,
                ymax - Plinko::WELL_DEPTH / 2.0 - Plinko::BOUNDARY_WIDTH,
                self.well_x - xmin,
                Plinko::WELL_DEPTH / 2.0,
            ],
            c.transform,
            gl,
        );
        graphics::rectangle(
            graphics::color::hex(super::colors::MULTI_WELL),
            [
                self.well_x,
                ymax - Plinko::WELL_DEPTH / 2.0 - Plinko::BOUNDARY_WIDTH,
                xmax - self.well_x,
                Plinko::WELL_DEPTH / 2.0,
            ],
            c.transform,
            gl,
        );
        graphics::rectangle(
            graphics::color::hex(super::colors::FRAME),
            [
                self.well_x - Plinko::WELL_WIDTH / 2.0,
                ymax - Plinko::WELL_DEPTH - Plinko::BOUNDARY_WIDTH,
                Plinko::WELL_WIDTH,
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

        for puck in &mut self.pucks {
            puck.draw(&c, gl);
        }
    }
}
