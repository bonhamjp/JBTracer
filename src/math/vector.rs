use crate::math::tuple::Tuple;
use crate::math::point::Point;

pub struct Vector {
  pub x: f64,
  pub y: f64,
  pub z: f64
}

impl Vector {
  pub fn new(x: f64, y: f64, z: f64) -> Vector {
    Vector { x: x, y: y, z: z }
  }

  pub fn empty() -> Vector {
    Vector { x: 0.0, y: 0.0, z: 0.0}
  }

  pub fn x_axis() -> Vector {
    Vector { x: 1.0, y: 0.0, z: 0.0 }
  }

  pub fn y_axis() -> Vector {
    Vector { x: 0.0, y: 1.0, z: 0.0 }
  }

  pub fn z_axis() -> Vector {
    Vector { x: 0.0, y: 0.0, z: 1.0 }
  }

  pub fn w(&self) -> f64 {
    0.0
  }

  pub fn add_vector(&self, r_hand: &Vector) -> Vector {
    let (new_x, new_y, new_z) = self.internal_add(r_hand);

    Vector::new(new_x, new_y, new_z)
  }

  pub fn add_point(&self, r_hand: &Point) -> Point {
    let (new_x, new_y, new_z) = self.internal_add(r_hand);

    Point::new(new_x, new_y, new_z)
  }

  pub fn subtract_vector(&self, r_hand: &Vector) -> Vector {
    let (new_x, new_y, new_z) = self.internal_subtract(r_hand);

    Vector::new(new_x, new_y, new_z)
  }

  pub fn multiply(&self, r_hand: f64) -> Vector {
    let (new_x, new_y, new_z) = self.internal_multiply(r_hand);

    Vector::new(new_x, new_y, new_z)
  }

  pub fn divide(&self, r_hand: f64) -> Vector {
    let (new_x, new_y, new_z) = self.internal_divide(r_hand);
    
    Vector::new(new_x, new_y, new_z)
  }

  pub fn normalize(&self) -> Vector {
    let magnitude = self.magnitude();

    Vector::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
  }

  pub fn cross(&self, r_hand: &Vector) -> Vector {
    let new_x = self.y * r_hand.z - self.z * r_hand.y;
    let new_y = self.z * r_hand.x - self.x * r_hand.z;
    let new_z = self.x * r_hand.y - self.y * r_hand.x;
    
    Vector::new(new_x, new_y, new_z)
  }
}

impl Tuple for Vector {
  fn get_quad(&self) -> (f64, f64, f64, f64) {
    (self.x, self.y, self.z, self.w())
  }
}
