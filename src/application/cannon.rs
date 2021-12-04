use graphics::Transformed;
use opengl_graphics::GlGraphics;
use rand::Rng;

pub struct Cannon {
    pub id: i8,
    color: [f32; 4],
    x: f64,
    y: f64,
    min_angle_deg: f64,
    max_angle_deg: f64,
    current_angle_deg: f64,
    current_barrel_move: f64,
    loaded_shots: i32,
    shot_delay: i32,
    pub is_alive: bool,
}

impl Cannon {
    const SPEED: f64 = 0.3;
    const SWEEP: f64 = 60.0;
    const RADIUS: i32 = 20;
    const FRAME_DELAY: i32 = 4;

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

        let mut rng = rand::thread_rng();
        Cannon {
            id: id,
            color: graphics::color::hex(hex),
            x: h as f64,
            y: v as f64,
            min_angle_deg: min,
            max_angle_deg: max,
            current_angle_deg: rng.gen_range(min..max),
            current_barrel_move: Cannon::SPEED,
            loaded_shots: 0,
            shot_delay: Cannon::FRAME_DELAY,
            is_alive: true,
        }
    }

    pub fn turn(&mut self) {
        if self.is_alive {
            self.current_angle_deg += self.current_barrel_move;
            if self.current_angle_deg >= self.max_angle_deg {
                self.current_angle_deg = self.max_angle_deg;
                self.current_barrel_move = -Cannon::SPEED;
            } else if self.current_angle_deg <= self.min_angle_deg {
                self.current_angle_deg = self.min_angle_deg;
                self.current_barrel_move = Cannon::SPEED;
            }
        }
    }

    pub fn draw(&self, c: &graphics::Context, gl: &mut GlGraphics) {
        if self.is_alive {
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
    }

    pub fn load(&mut self, num_shots: i32) {
        if self.is_alive {
            self.loaded_shots += num_shots;
        }
    }

    pub fn shoot(&mut self) -> Option<super::bullet::Bullet> {
        if self.is_alive && self.loaded_shots > 0 {
            if self.shot_delay > 0 {
                self.shot_delay -= 1;
            } else {
                self.shot_delay = Cannon::FRAME_DELAY;
                self.loaded_shots -= 1;
                return Some(super::bullet::Bullet::new(
                    self.id,
                    self.color,
                    self.x,
                    self.y,
                    self.current_angle_deg,
                ));
            }
        }
        None
    }

    pub fn check_collision(&mut self, bullet: &mut super::bullet::Bullet) {
        if self.is_alive && self.id != bullet.cannon_id {
            let a = (self.x - bullet.position[0]).abs();
            let b = (self.y - bullet.position[1]).abs();
            let distance = a.hypot(b);
            if distance < (Cannon::RADIUS as f64 + super::bullet::Bullet::RADIUS) {
                self.is_alive = false;
                bullet.is_alive = false;
            }
        }
    }
}
