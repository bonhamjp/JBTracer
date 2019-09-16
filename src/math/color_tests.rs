#[cfg(test)]
mod tests {
  use crate::math::Color;

  #[test]
  fn new_sets_values() {
    let color = Color::new(0.5, 0.0, 0.25, 1.0);

    assert_eq!(color.r, 0.5);
    assert_eq!(color.g, 0.0);
    assert_eq!(color.b, 0.25);
    assert_eq!(color.a, 1.0);
  }

  #[test]
  fn empty_sets_0_values() {
    let color = Color::empty();

    assert_eq!(color.r, 0.0);
    assert_eq!(color.g, 0.0);
    assert_eq!(color.b, 0.0);
    assert_eq!(color.a, 0.0);
  }

  #[test]
  fn r_display_returns_r_in_0_255_range() {
    let color_1 = Color::new(0.9, 0.0, 0.0, 1.0);

    assert_eq!(color_1.r_display(), 229);

    let color_2 = Color::new(2.0, 0.0, 0.0, 1.0);

    assert_eq!(color_2.r_display(), 255);

    let color_3 = Color::new(-1.0, 0.0, 0.0, 1.0);

    assert_eq!(color_3.r_display(), 0);
  }

  #[test]
  fn g_display_returns_g_in_0_255_range() {
    let color_1 = Color::new(0.0, 0.8, 0.0, 1.0);

    assert_eq!(color_1.g_display(), 204);

    let color_2 = Color::new(0.0, 2.0, 0.0, 1.0);

    assert_eq!(color_2.g_display(), 255);

    let color_3 = Color::new(0.0, -1.0, 0.0, 1.0);

    assert_eq!(color_3.g_display(), 0);
  }

  #[test]
  fn b_display_returns_b_in_0_255_range() {
    let color_1 = Color::new(0.0, 0.0, 0.7, 1.0);

    assert_eq!(color_1.b_display(), 178);

    let color_2 = Color::new(0.0, 0.0, 2.0, 1.0);

    assert_eq!(color_2.b_display(), 255);

    let color_3 = Color::new(0.0, 0.0, -1.0, 1.0);

    assert_eq!(color_3.b_display(), 0);
  }

  #[test]
  fn a_display_returns_a_in_0_255_range() {
    let color_1 = Color::new(0.0, 0.0, 0.0, 0.6);

    assert_eq!(color_1.a_display(), 153);

    let color_2 = Color::new(0.0, 0.0, 0.0, 2.0);

    assert_eq!(color_2.a_display(), 255);

    let color_3 = Color::new(0.0, 0.0, 0.0, -1.0);

    assert_eq!(color_3.a_display(), 0);
  }

  #[test]
  fn display_values_returns_tuple_of_values_in_0_255_range() {
    let color = Color::new(0.9, 0.8, 0.7, 0.6);

    assert_eq!(color.display_values(), (229, 204, 178, 153));
  }
}
