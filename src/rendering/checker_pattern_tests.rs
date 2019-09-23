#[cfg(test)]
mod tests {
  use std::f64;

  use crate::rendering::pattern::Pattern;
  use crate::rendering::CheckerPattern;

  use crate::math::Point;
  
  use crate::math::Color;

  use crate::math::Matrix4x4;

  #[test]
  fn checker_pattern_created_with_two_colors_and_transform() {
    let checker_pattern = CheckerPattern::new(
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );

    assert!(checker_pattern.color_1 == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(checker_pattern.color_2 == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(checker_pattern.transform == Matrix4x4::identity());
  }

  #[test]
  fn checker_pattern_repeats_along_x() {
    let checker_pattern = CheckerPattern::new(
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );

    assert!(checker_pattern.color_at(&Point::new(0.0, 0.0, 0.0)) == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(checker_pattern.color_at(&Point::new(0.99, 0.0, 0.0)) == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(checker_pattern.color_at(&Point::new(1.01, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
  }

  #[test]
  fn checker_pattern_repeats_along_y() {
    let checker_pattern = CheckerPattern::new(
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );

    assert!(checker_pattern.color_at(&Point::new(0.0, 0.0, 0.0)) == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(checker_pattern.color_at(&Point::new(0.0, 0.99, 0.0)) == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(checker_pattern.color_at(&Point::new(0.0, 1.01, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
  }

  #[test]
  fn checker_pattern_repeats_along_z() {
    let checker_pattern = CheckerPattern::new(
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );

    assert!(checker_pattern.color_at(&Point::new(0.0, 0.0, 0.0)) == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(checker_pattern.color_at(&Point::new(0.0, 0.0, 0.99)) == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(checker_pattern.color_at(&Point::new(0.0, 0.0, 1.01)) == Color::new(1.0, 1.0, 1.0, 1.0));
  }
}
