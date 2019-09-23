use crate::rendering::pattern::Pattern;

use crate::math::Point;

use crate::math::Color;

use crate::math::Matrix4x4;

#[derive(PartialEq)]
pub struct CheckerPattern {
  pub color_1: Color,
  pub color_2: Color,
  pub transform: Matrix4x4
}

impl CheckerPattern {
  pub fn new(color_1: Color, color_2: Color, transform: Matrix4x4) -> CheckerPattern {
    CheckerPattern { color_1: color_1, color_2: color_2, transform: transform }
  }
}

impl Pattern for CheckerPattern {
  fn color_at(&self, point: &Point) -> Color {
    let summed_floor = (point.x.floor() as u64).wrapping_add(point.y.floor() as u64).wrapping_add(point.z.floor() as u64); 
    if summed_floor % 2 != 0 {
      self.color_1
    } else {
      self.color_2
    }
  }

  fn get_transform(&self) -> &Matrix4x4 {
    &self.transform
  }
}
