use graphics::Radians;
use opengl_graphics::GlGraphics;

pub struct Puck {
    position: [f64; 2],
    angle: f64,
    speed: f64,
    color: [f32; 4],
    pub is_alive: bool,
}

impl Puck {
    pub const RADIUS: f64 = 18.0;
    // const GRAVITY: f64 = 0.2;
    const INITIAL_SPEED: f64 = 0.6;

    pub fn new_fixed(pos: [f64; 2]) -> Puck {
        Puck {
            position: pos,
            angle: 0.0,
            speed: -1.0,
            color: graphics::color::hex(super::super::colors::FRAME),
            is_alive: true,
        }
    }

    pub fn new_active(pos: [f64; 2], angle: f64, color: [f32; 4]) -> Puck {
        Puck {
            position: pos,
            angle: angle,
            speed: Puck::INITIAL_SPEED,
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

    pub fn step(&mut self, boundaries: [f64; 4]) {
        if self.speed > 0.0 {
            self.check_boundary_collisions(boundaries);
            let x = self.speed * self.angle.deg_to_rad().cos();
            let y = self.speed * self.angle.deg_to_rad().sin();
            self.position = [self.position[0] + x, self.position[1] + y];
        }
    }

    fn check_boundary_collisions(&mut self, boundaries: [f64; 4]) {
        if self.is_alive {
            self.angle = super::super::common::check_circle_boundary_collisions(
                true,
                boundaries,
                self.position,
                Puck::RADIUS,
                self.angle,
            );
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

    pub fn bounce(&mut self, rect: [f64; 4]) {
        self.angle = super::super::common::check_circle_boundary_collisions(
            false,
            rect,
            self.position,
            Puck::RADIUS,
            self.angle,
        );
    }
}
