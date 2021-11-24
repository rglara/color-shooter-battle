use graphics::Transformed;
use opengl_graphics::GlGraphics;

pub struct Cannon {
    pub id: i8,
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
    const RADIUS: i32 = 20;

    pub fn new(id: i8, hex: &str, is_left: bool, is_top: bool) -> Cannon {
        let mut h = 2 * super::common::BORDER_SIZE + super::common::SIDE_WIDTH;
        if is_left {
            h += Cannon::RADIUS * 3 / 2;
        } else {
            h += (super::common::CELL_WIDTH * super::common::CELL_EDGES * 2)
                - (Cannon::RADIUS * 3 / 2);
        }
        let mut v = super::common::BORDER_SIZE;
        if is_top {
            v += Cannon::RADIUS * 3 / 2;
        } else {
            v += (super::common::CELL_WIDTH * super::common::CELL_EDGES * 2)
                - (Cannon::RADIUS * 3 / 2);
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

    pub fn draw(&self, c: &graphics::Context, gl: &mut GlGraphics) {
        let base = [
            self.x - Cannon::RADIUS as f64,
            self.y - Cannon::RADIUS as f64,
            (Cannon::RADIUS * 2) as f64,
            (Cannon::RADIUS * 2) as f64,
        ];
        graphics::ellipse(self.color, base, c.transform, gl);
        let barrel = [
            0.0,
            0.0,
            (3 * Cannon::RADIUS / 2) as f64,
            (2 * Cannon::RADIUS / 3) as f64,
        ];
        let barrel_transform = c
            .transform
            .trans(self.x, self.y)
            .rot_deg(self.current_angle_deg)
            .trans(0.0, (Cannon::RADIUS / -4) as f64);
        graphics::rectangle(self.color, barrel, barrel_transform, gl);
    }

    pub fn shoot(&self) -> super::bullet::Bullet {
        super::bullet::Bullet::new(self.id, self.color, self.x, self.y, self.current_angle_deg)
    }
}
