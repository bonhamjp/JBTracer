use crate::rendering::pattern::Pattern;

use crate::math::Point;

use crate::math::Color;

use crate::math::Matrix4x4;

#[derive(PartialEq)]
pub struct RingPattern {
  pub color_1: Color,
  pub color_2: Color,
  pub transform: Matrix4x4
}

impl RingPattern {
  pub fn new(color_1: Color, color_2: Color, transform: Matrix4x4) -> RingPattern {
    RingPattern { color_1: color_1, color_2: color_2, transform: transform }
  }
}

impl Pattern for RingPattern {
  fn color_at(&self, point: &Point) -> Color {
    if ((point.x * point.x + point.z * point.z).sqrt().floor() as u64) % 2 != 0 {
      self.color_1
    } else {
      self.color_2
    }
  }

  fn get_transform(&self) -> &Matrix4x4 {
    &self.transform
  }
}
