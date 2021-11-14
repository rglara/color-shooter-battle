use opengl_graphics::GlGraphics;

pub struct Plinko {
    id: i32,
    position: [f64; 2],
    color: [f32; 4],
}

impl Plinko {
    pub fn new(id: i32, color: &str, position: [f64; 2]) -> Plinko {
        Plinko {
            id: id,
            position: position,
            color: graphics::color::hex(color),
        }
    }

    pub fn update(&mut self) {}

    pub fn draw(&mut self, c: &graphics::Context, gl: &mut GlGraphics) {
        let xline = [
            self.position[0],
            self.position[1],
            self.position[0] + super::common::SIDE_WIDTH as f64,
            self.position[1] + (super::common::CELL_WIDTH * super::common::CELL_EDGES) as f64,
        ];
        graphics::line(self.color, 1.0, xline, c.transform, gl);
        graphics::line(
            self.color,
            1.0,
            [xline[2], xline[1], xline[0], xline[3]],
            c.transform,
            gl,
        );
    }
}
