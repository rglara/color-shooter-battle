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
    const RADIUS: f64 = 5.0;
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

    pub fn draw(&self, c: &graphics::Context, gl: &mut GlGraphics) {
        let rect = [
            self.position[0],
            self.position[1],
            Bullet::RADIUS * 2.0,
            Bullet::RADIUS * 2.0,
        ];
        graphics::ellipse(self.color, rect, c.transform, gl);
    }

    fn check_boundary_collisions(&mut self, grid_rect: [f64; 4]) {
        if (self.position[0] < grid_rect[0])
            || ((self.position[0] + (Bullet::RADIUS * 2.0)) > (grid_rect[0] + grid_rect[2]))
        {
            self.angle = 180.0 - self.angle;
        }
        if (self.position[1] < grid_rect[1])
            || ((self.position[1] + (Bullet::RADIUS * 2.0)) > (grid_rect[1] + grid_rect[3]))
        {
            self.angle = 360.0 - self.angle;
        }
    }
}
