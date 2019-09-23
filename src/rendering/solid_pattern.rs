use crate::rendering::pattern::Pattern;

use crate::math::Point;

use crate::math::Color;

use crate::math::Matrix4x4;

#[derive(PartialEq)]
pub struct SolidPattern {
  pub color: Color,
  pub transform: Matrix4x4
}

impl SolidPattern {
  pub fn new(color: Color, transform: Matrix4x4) -> SolidPattern {
    SolidPattern { color: color, transform: transform }
  }
}

impl Pattern for SolidPattern {
  fn color_at(&self, point: &Point) -> Color {
    self.color
  }

  fn get_transform(&self) -> &Matrix4x4 {
    &self.transform
  }
}
