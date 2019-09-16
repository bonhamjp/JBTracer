use crate::rendering::Intersection;

use crate::math::Matrix4x4;
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

  pub fn transform(&self, transformation: &Matrix4x4) -> Ray {
    let (new_position_x, new_position_y, new_position_z, _) = transformation.mult4x1(&self.origin);
    let (new_direction_x, new_direction_y, new_direction_z, _) = transformation.mult4x1(&self.direction);

    let new_origin = Point::new(new_position_x, new_position_y, new_position_z);
    let new_direction = Vector::new(new_direction_x, new_direction_y, new_direction_z);

    Ray { origin: new_origin, direction: new_direction }
  }
}
