use crate::math::tuple::Tuple;
use crate::math::vector::Vector;

pub struct Point {
  pub x: f64,
  pub y: f64,
  pub z: f64
}

impl Point {
  pub fn new(x: f64, y: f64, z: f64) -> Point {
    Point { x: x, y: y, z: z }
  }

  pub fn empty() -> Point {
    Point { x: 0.0, y: 0.0, z: 0.0}
  }

  pub fn w(&self) -> f64 {
    1.0
  }

  pub fn add_vector(&self, r_hand: &Vector) -> Point {
    let (new_x, new_y, new_z) = self.internal_add(r_hand);

    Point::new(new_x, new_y, new_z)
  }

  pub fn subtract_point(&self, r_hand: &Point) -> Vector {
    let (new_x, new_y, new_z) = self.internal_subtract(r_hand);

    Vector::new(new_x, new_y, new_z)
  }

  pub fn subtract_vector(&self, r_hand: &Vector) -> Point {
    let (new_x, new_y, new_z) = self.internal_subtract(r_hand);

    Point::new(new_x, new_y, new_z)
  }

  pub fn multiply(&self, r_hand: f64) -> Point {
    let (new_x, new_y, new_z) = self.internal_multiply(r_hand);

    Point::new(new_x, new_y, new_z)
  }

  pub fn divide(&self, r_hand: f64) -> Point {
    let (new_x, new_y, new_z) = self.internal_divide(r_hand);
    
    Point::new(new_x, new_y, new_z)
  }

  pub fn normalize(&self) -> Point {
    let magnitude = self.magnitude();

    Point::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
  }
}

impl Tuple for Point {
  fn get_quad(&self) -> (f64, f64, f64, f64) {
    (self.x, self.y, self.z, self.w())
  }
}
