use opengl_graphics::GlGraphics;

struct Puck {
    position: [f64; 2],
    angle: f64,
    speed: f64,
    color: [f32; 4],
}

impl Puck {
    const RADIUS: f64 = 36.0;

    pub fn new_fixed(pos: [f64; 2]) -> Puck {
        Puck {
            position: [pos[0] - Puck::RADIUS / 2.0, pos[1] - Puck::RADIUS / 2.0],
            angle: 0.0,
            speed: -1.0,
            color: graphics::color::hex(super::colors::COLOR_FRAME),
        }
    }

    pub fn new_active(pos: [f64; 2], color: [f32; 4]) -> Puck {
        Puck {
            position: [pos[0] - Puck::RADIUS / 2.0, pos[1] - Puck::RADIUS / 2.0],
            angle: 90.0,
            speed: 0.0,
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
}

pub struct Plinko {
    id: i32,
    position: [f64; 2],
    color: [f32; 4],
    pucks: Vec<Puck>,
    time: f64,
    num_pucks: i32,
}

impl Plinko {
    pub const BOUNDARY_WIDTH: f64 = 10.0;

    pub fn new(id: i32, color: &str, position: [f64; 2]) -> Plinko {
        // stationary pucks are "pegs" to bounce off of
        let mut pucks = Vec::new();
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
                pucks.push(Puck::new_fixed([
                    position[0] + Plinko::BOUNDARY_WIDTH + (HSPACE * h as f64) + offset,
                    position[1] + (VSPACE * v as f64) - Puck::RADIUS / 2.0,
                ]));
            }
        }

        Plinko {
            id: id,
            position: position,
            color: graphics::color::hex(color),
            pucks: pucks,
            time: 0.0,
            num_pucks: 0,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        if (self.time / 10.0) as i32 >= self.num_pucks {
            self.num_pucks += 1;
            self.pucks.push(Puck::new_active(
                [
                    self.position[0] + (super::common::SIDE_WIDTH / 2) as f64,
                    self.position[1] + Plinko::BOUNDARY_WIDTH + (3.0 * Puck::RADIUS / 4.0),
                ],
                self.color,
            ));
        }
        self.time += delta_time;
    }

    pub fn draw(&mut self, c: &graphics::Context, gl: &mut GlGraphics) {
        let xmin = self.position[0];
        let xmax = xmin + super::common::SIDE_WIDTH as f64;
        let ymin = self.position[1];
        let ymax = ymin + (super::common::CELL_WIDTH * super::common::CELL_EDGES) as f64;

        graphics::rectangle(
            graphics::color::hex(super::colors::COLOR_FRAME),
            [xmin, ymin, xmax - xmin, Plinko::BOUNDARY_WIDTH],
            c.transform,
            gl,
        );
        graphics::rectangle(
            graphics::color::hex(super::colors::COLOR_FRAME),
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
            graphics::color::hex(super::colors::COLOR_FRAME),
            [xmin, ymin, Plinko::BOUNDARY_WIDTH, ymax - ymin],
            c.transform,
            gl,
        );
        graphics::rectangle(
            graphics::color::hex(super::colors::COLOR_FRAME),
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
