use crate::rendering::pattern::Pattern;

use crate::math::Point;

use crate::math::Color;

use crate::math::Matrix4x4;

#[derive(PartialEq)]
pub struct GradientPattern {
  pub color_1: Color,
  pub color_2: Color,
  pub distance: Color,
  pub transform: Matrix4x4
}

impl GradientPattern {
  pub fn new(color_1: Color, color_2: Color, transform: Matrix4x4) -> GradientPattern {
    let distance = color_2.subtract_color(&color_1);

    GradientPattern { color_1: color_1, color_2: color_2, distance: distance, transform: transform }
  }
}

impl Pattern for GradientPattern {
  fn color_at(&self, point: &Point) -> Color {
    let remainder = point.x - point.x.floor();

    let d_2 = self.color_2.subtract_color(&self.color_1);

    self.color_1.add_color(&self.distance.mult_scalar(remainder))
  }

  fn get_transform(&self) -> &Matrix4x4 {
    &self.transform
  }
}
