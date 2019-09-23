#[cfg(test)]
mod tests {
  use std::f64;

  use crate::rendering::pattern::Pattern;
  use crate::rendering::RingPattern;

  use crate::math::Point;
  
  use crate::math::Color;

  use crate::math::Matrix4x4;

  #[test]
  fn ring_pattern_created_with_two_colors_and_transform() {
    let ring_pattern = RingPattern::new(
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );

    assert!(ring_pattern.color_1 == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(ring_pattern.color_2 == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(ring_pattern.transform == Matrix4x4::identity());
  }

  #[test]
  fn ring_pattern_linearly_interpolates_between_colors() {
    let ring_pattern = RingPattern::new(
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );

    assert!(ring_pattern.color_at(&Point::new(0.0, 0.0, 0.0)) == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(ring_pattern.color_at(&Point::new(1.0, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(ring_pattern.color_at(&Point::new(0.0, 0.0, 1.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
  }
}
