#[cfg(test)]
mod tests {
  use std::f64;

  use crate::rendering::pattern::Pattern;
  use crate::rendering::StripePattern;

  use crate::math::Point;
  
  use crate::math::Color;

  use crate::math::Matrix4x4;

  #[test]
  fn stripe_pattern_created_with_two_colors_and_transform() {
    let stripe_pattern = StripePattern::new(
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );

    assert!(stripe_pattern.color_1 == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(stripe_pattern.color_2 == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(stripe_pattern.transform == Matrix4x4::identity());
  }

  #[test]
  fn stripe_pattern_returns_constant_color_if_only_y_changes() {
    let stripe_pattern = StripePattern::new(
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );

    assert!(stripe_pattern.color_at(&Point::new(0.0, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(stripe_pattern.color_at(&Point::new(0.0, 0.0, 1.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(stripe_pattern.color_at(&Point::new(0.0, 0.0, 2.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
  }

  #[test]
  fn stripe_pattern_returns_constant_color_if_only_z_changes() {
    let stripe_pattern = StripePattern::new(
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );

    assert!(stripe_pattern.color_at(&Point::new(0.0, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(stripe_pattern.color_at(&Point::new(0.0, 1.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(stripe_pattern.color_at(&Point::new(0.0, 2.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
  }

  #[test]
  fn stripe_pattern_alternates_as_x_changes() {
    let stripe_pattern = StripePattern::new(
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );

    assert!(stripe_pattern.color_at(&Point::new(0.0, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(stripe_pattern.color_at(&Point::new(0.0, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));    
    assert!(stripe_pattern.color_at(&Point::new(1.0, 0.0, 0.0)) == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(stripe_pattern.color_at(&Point::new(-0.1, 0.0, 0.0)) == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(stripe_pattern.color_at(&Point::new(-1.0, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(stripe_pattern.color_at(&Point::new(-1.1, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
  }
}
