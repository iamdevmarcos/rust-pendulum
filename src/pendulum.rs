use speedy2d::{color::Color, Graphics2D};
use crate::vector::Vector;

pub struct Pendulum {
  origin: Vector,
  position: Vector,

  angle: f32,

  angular_velocity: f32,
  angular_acceleration: f32,

  r: f32, // pendulum length
  m: f32, // pendulum mass of the ball
  g: f32, // gravity
}

impl Pendulum {
  pub fn new(x: f32, y: f32, r: f32) -> Pendulum {
    Pendulum { 
      origin: Vector::new(x, y),
      position: Vector::new(0.0, 0.0), 
      angle: 1.0, 
      angular_velocity: 0.0, 
      angular_acceleration: 0.0, 
      r: r, 
      m: 1.0, 
      g: 1.5,
    }
  }

  pub fn update(&mut self) {
    self. angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;
    self.angular_velocity += self.angular_acceleration;
    self.angle += self.angular_velocity;
    self.position.set(self.r * self.angle.sin(), self.r * self.angle.cos());
    self.position.add(&self.origin);
  }

  pub fn draw(&self, graphics: &mut Graphics2D) {
    graphics.draw_line(
      (self.origin.x, self.origin.y),
      (self.position.x, self.position.y),
      3.0,
      Color::BLUE,
    );

    graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::CYAN);
  }
}