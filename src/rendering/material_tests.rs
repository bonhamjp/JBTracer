#[cfg(test)]
mod tests {
  use crate::rendering::Material;

  use crate::math::Color;

  #[test]
  fn new_sets_values() {
    let material = Material::new(Color::new(0.5, 0.1, 0.75, 1.0), 0.1, 0.2, 0.3, 0.4);

    assert_eq!(material.color.r, 0.5);
    assert_eq!(material.color.g, 0.1);
    assert_eq!(material.color.b, 0.75);
    assert_eq!(material.color.a, 1.0);
    assert_eq!(material.ambient, 0.1);
    assert_eq!(material.diffuse, 0.2);
    assert_eq!(material.specular, 0.3);
    assert_eq!(material.shininess, 0.4);
  }

  #[test]
  fn default_sets_base_values() {
    let material = Material::default();

    assert_eq!(material.color.r, 1.0);
    assert_eq!(material.color.g, 1.);
    assert_eq!(material.color.b, 1.0);
    assert_eq!(material.color.a, 1.0);
    assert_eq!(material.ambient, 0.1);
    assert_eq!(material.diffuse, 0.9);
    assert_eq!(material.specular, 0.9);
    assert_eq!(material.shininess, 200.0);
  }
}