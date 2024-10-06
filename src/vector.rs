pub struct Vector {
  pub x: f32,
  pub y: f32,
}

impl Vector {
  pub fn new(x: f32, y: f32) -> Vector {
    Vector { x, y }
  }

  pub fn add(&mut self, other: &Vector) -> &Vector {
    self.x += other.x;
    self.y += other.y;
    self
  }

  pub fn set(&mut self, x: f32, y: f32) -> &Vector {
    self.x = x;
    self.y = y;
    self
  }
}