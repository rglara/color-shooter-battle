use graphics::Radians;
use opengl_graphics::GlGraphics;

pub struct Bullet {
    pub cannon_id: i8,
    color: [f32; 4],
    pub position: [f64; 2],
    angle: f64,
    speed: f64,
    pub is_alive: bool,
}

impl Bullet {
    pub const RADIUS: f64 = 5.0;
    const SPEED: f64 = 1.5;

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
        if self.is_alive {
            let x = self.speed * self.angle.deg_to_rad().cos();
            let y = self.speed * self.angle.deg_to_rad().sin();
            self.position = [self.position[0] + x, self.position[1] + y];
            self.check_boundary_collisions(grid_rect);
        }
    }

    pub fn draw(&self, c: &graphics::Context, gl: &mut GlGraphics) {
        if self.is_alive {
            let rect = [
                self.position[0] - Bullet::RADIUS,
                self.position[1] - Bullet::RADIUS,
                Bullet::RADIUS * 2.0,
                Bullet::RADIUS * 2.0,
            ];
            graphics::ellipse(self.color, rect, c.transform, gl);
        }
    }

    fn check_boundary_collisions(&mut self, grid_rect: [f64; 4]) {
        if self.is_alive {
            self.angle = super::common::check_circle_boundary_collisions(
                true,
                grid_rect,
                self.position,
                Bullet::RADIUS,
                self.angle,
            );
        }
    }
}
