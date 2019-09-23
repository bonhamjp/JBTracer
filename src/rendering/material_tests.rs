#[cfg(test)]
mod tests {
  use crate::rendering::Material;

  use crate::rendering::pattern::Pattern;
  use crate::rendering::SolidPattern;

  use crate::math::Color;

  use crate::math::Matrix4x4;

  #[test]
  fn new_sets_values() {
    let pattern = &SolidPattern::new(Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let material = Material::new(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, pattern);

    assert_eq!(material.ambient, 0.1);
    assert_eq!(material.diffuse, 0.9);
    assert_eq!(material.specular, 0.9);
    assert_eq!(material.shininess, 200.0);
    assert_eq!(material.reflectiveness, 0.0);
    assert_eq!(material.transparency, 0.0);
    assert_eq!(material.refractive_index, 1.0);
  }
}
