#[cfg(test)]
mod tests {
  use std::f64;

  use crate::rendering::pattern::Pattern;
  use crate::rendering::SolidPattern;

  use crate::math::Point;
  
  use crate::math::Color;

  use crate::math::Matrix4x4;

  #[test]
  fn solid_pattern_created_with_one_color_and_transform() {
    let solid_pattern = SolidPattern::new(Color::new(1.0, 0.0, 0.0, 1.0), Matrix4x4::identity());

    assert!(solid_pattern.color == Color::new(1.0, 0.0, 0.0, 1.0));
    assert!(solid_pattern.transform == Matrix4x4::identity());
  }

  #[test]
  fn solid_pattern_always_returns_single_color() {
    let solid_pattern = SolidPattern::new(Color::new(0.0, 1.0, 0.0, 1.0), Matrix4x4::identity());

    assert!(solid_pattern.color_at(&Point::new(3.0, 0.0, 0.0)) == Color::new(0.0, 1.0, 0.0, 1.0));
    assert!(solid_pattern.color_at(&Point::new(0.0, 3.0, 0.0)) == Color::new(0.0, 1.0, 0.0, 1.0));
    assert!(solid_pattern.color_at(&Point::new(0.0, 0.0, 3.0)) == Color::new(0.0, 1.0, 0.0, 1.0));
    assert!(solid_pattern.color_at(&Point::new(3.0, 2.0, 1.0)) == Color::new(0.0, 1.0, 0.0, 1.0));
    assert!(solid_pattern.color_at(&Point::new(1.0, 3.0, 2.0)) == Color::new(0.0, 1.0, 0.0, 1.0));
    assert!(solid_pattern.color_at(&Point::new(2.0, 1.0, 3.0)) == Color::new(0.0, 1.0, 0.0, 1.0));
  }
}
