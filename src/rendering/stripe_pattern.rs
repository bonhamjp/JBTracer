use crate::rendering::pattern::Pattern;

use crate::math::Point;

use crate::math::Color;

use crate::math::Matrix4x4;

#[derive(PartialEq)]
pub struct StripePattern {
  pub color_1: Color,
  pub color_2: Color,
  pub transform: Matrix4x4
}

impl StripePattern {
  pub fn new(color_1: Color, color_2: Color, transform: Matrix4x4) -> StripePattern {
    StripePattern { color_1: color_1, color_2: color_2, transform: transform }
  }
}

impl Pattern for StripePattern {
  fn color_at(&self, point: &Point) -> Color {
    let adjusted_x;
    
    if (point.x < 0.0) {
      adjusted_x = (((point.x) * -1.0) + 1.0) as u64;
    } else {
      adjusted_x = point.x as u64;
    }
    
    if adjusted_x % 2 == 0 {
      return self.color_1;
    } else {
      return self.color_2;
    }
  }

  fn get_transform(&self) -> &Matrix4x4 {
    &self.transform
  }
}
