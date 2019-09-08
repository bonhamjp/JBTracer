use crate::rendering::Intersection;

use crate::math::Point;
use crate::math::Vector;

pub struct Ray {
  pub origin: Point,
  pub direction: Vector
}

impl Ray {
  pub fn new(origin: &Point, direction: &Vector) -> Ray {
    let origin_point = Point::new(origin.x, origin.y, origin.z);
    let direction_vector = Vector::new(direction.x, direction.y, direction.z);
    
    Ray { origin: origin_point, direction: direction_vector }
  }

  pub fn position(&self, t: f64) -> Point {
    let start_position = Point::new(self.origin.x, self.origin.y, self.origin.z);
    let projected_direction = self.direction.multiply(t);

    start_position.add_vector(&projected_direction)
  }
}
