#[cfg(test)]
mod tests {
  use std::f64;

  use crate::rendering::pattern::Pattern;
  use crate::rendering::GradientPattern;

  use crate::math::Point;
  
  use crate::math::Color;

  use crate::math::Matrix4x4;

  #[test]
  fn gradient_pattern_created_with_two_colors_and_transform() {
    let gradient_pattern = GradientPattern::new(
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );

    assert!(gradient_pattern.color_1 == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(gradient_pattern.color_2 == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(gradient_pattern.distance == Color::new(-1.0, -1.0, -1.0, 1.0));
    assert!(gradient_pattern.transform == Matrix4x4::identity());
  }

  #[test]
  fn gradient_pattern_linearly_interpolates_between_colors() {
    let gradient_pattern = GradientPattern::new(
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );

    assert!(gradient_pattern.color_at(&Point::new(0.25, 0.0, 0.0)) == Color::new(0.75, 0.75, 0.75, 1.0));
    assert!(gradient_pattern.color_at(&Point::new(0.5, 0.0, 0.0)) == Color::new(0.5, 0.5, 0.5, 1.0));
    assert!(gradient_pattern.color_at(&Point::new(0.75, 0.0, 0.0)) == Color::new(0.25, 0.25, 0.25, 1.0));
  }
}
