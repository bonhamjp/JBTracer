#[cfg(test)]
mod tests {
  use std::f64;

  use crate::rendering::math::Point;
  use crate::rendering::math::Vector;
  
  use crate::rendering::math::Color;
  
  use crate::rendering::math::Matrix4x4;

  use crate::rendering::shapes::shape::Shape;
  use crate::rendering::shapes::Sphere;

  use crate::rendering::PointLight;

  use crate::rendering::Material;

  #[test]
  fn new_sets_values() {
    let point_light = PointLight::new(Color::new(0.5, 0.1, 0.75, 1.0), Point::new(1.0, 2.0, 3.0));

    assert_eq!(point_light.intensity.r, 0.5);
    assert_eq!(point_light.intensity.g, 0.1);
    assert_eq!(point_light.intensity.b, 0.75);
    assert_eq!(point_light.intensity.a, 1.0);
    assert_eq!(point_light.position.x, 1.0);
    assert_eq!(point_light.position.y, 2.0);
    assert_eq!(point_light.position.z, 3.0);
    assert_eq!(point_light.position.w, 1.0);
  }

  #[test]
  fn default_sets_base_values() {
    let point_light = PointLight::default();

    assert_eq!(point_light.intensity.r, 1.0);
    assert_eq!(point_light.intensity.g, 1.0);
    assert_eq!(point_light.intensity.b, 1.0);
    assert_eq!(point_light.intensity.a, 1.0);
    assert_eq!(point_light.position.x, 0.0);
    assert_eq!(point_light.position.y, 0.0);
    assert_eq!(point_light.position.z, 0.0);
    assert_eq!(point_light.position.w, 1.0);
  }

  #[test]
  fn lighting_with_eye_between_light_and_surface() {
    let point_light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));    
    
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = &Sphere::new(Matrix4x4::identity(), material);
    
    let position = Point::empty();
    let eye_v = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);

    let lighting_color = point_light.lighting(sphere, &position, &eye_v, &normal, false);

    assert_eq!(lighting_color.r, 1.9);
    assert_eq!(lighting_color.g, 1.9);
    assert_eq!(lighting_color.b, 1.9);
    assert_eq!(lighting_color.a, 1.0);
  }

  #[test]
  fn lighting_with_eye_between_light_and_surface_eye_offset_45_degrees() {
    let point_light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));

    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = &Sphere::new(Matrix4x4::identity(), material);
    
    let position = Point::empty();
    let eye_v = Vector::new(0.0, (2.0 as f64).sqrt() / 2.0, -(2.0 as f64).sqrt() / 2.0);
    let normal = Vector::new(0.0, 0.0, -1.0);

    let lighting_color = point_light.lighting(sphere, &position, &eye_v, &normal, false);

    assert_eq!(lighting_color.r, 1.0);
    assert_eq!(lighting_color.g, 1.0);
    assert_eq!(lighting_color.b, 1.0);
    assert_eq!(lighting_color.a, 1.0);
  }

  #[test]
  fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
    let point_light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0));
    
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = &Sphere::new(Matrix4x4::identity(), material);

    let position = Point::empty();
    let eye_v = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);

    let lighting_color = point_light.lighting(sphere, &position, &eye_v, &normal, false);

    assert_eq!(lighting_color.r, 0.7363961030678927);
    assert_eq!(lighting_color.g, 0.7363961030678927);
    assert_eq!(lighting_color.b, 0.7363961030678927);
    assert_eq!(lighting_color.a, 1.0);
  }

  #[test]
  fn lighting_with_eye_facing_reflection_vector() {
    let point_light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0));

    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = &Sphere::new(Matrix4x4::identity(), material);

    let position = Point::empty();
    let eye_v = Vector::new(0.0, -(2.0 as f64).sqrt() / 2.0, -(2.0 as f64).sqrt() / 2.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    
    let lighting_color = point_light.lighting(sphere, &position, &eye_v, &normal, false);

    assert_eq!(lighting_color.r, 1.6363961030678928);
    assert_eq!(lighting_color.g, 1.6363961030678928);
    assert_eq!(lighting_color.b, 1.6363961030678928);
    assert_eq!(lighting_color.a, 1.0);
  }

  #[test]
  fn lighting_with_light_behind_surface() {
    let point_light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(0.0, 0.0, 10.0));
    
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = &Sphere::new(Matrix4x4::identity(), material);

    let position = Point::empty();
    let eye_v = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);

    let lighting_color = point_light.lighting(sphere, &position, &eye_v, &normal, false);

    assert_eq!(lighting_color.r, 0.1);
    assert_eq!(lighting_color.g, 0.1);
    assert_eq!(lighting_color.b, 0.1);
    assert_eq!(lighting_color.a, 1.0);
  }

  #[test]
  fn lighting_with_surface_in_shadow() {
    let point_light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));

    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = &Sphere::new(Matrix4x4::identity(), material);
    
    let position = Point::empty();
    let eye_v = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);

    let lighting_color = point_light.lighting(sphere, &position, &eye_v, &normal, true);

    assert_eq!(lighting_color.r, 0.1);
    assert_eq!(lighting_color.g, 0.1);
    assert_eq!(lighting_color.b, 0.1);
    assert_eq!(lighting_color.a, 1.0);
  }

  #[test]
  fn lighting_object_with_pattern() {
    let point_light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));

    let material = Material::striped(
      1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 
      Color::new(1.0, 1.0, 1.0, 1.0),
      Color::new(0.0, 0.0, 0.0, 1.0),
      Matrix4x4::identity()
    );
    let sphere = &Sphere::new(Matrix4x4::identity(), material);
    
    let eye_v = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);

    let lighting_color_1 = point_light.lighting(
      sphere, 
      &Point::new(0.9, 0.0, 0.0), 
      &eye_v, 
      &normal, 
      false
    );

    assert!(lighting_color_1 == Color::new(1.0, 1.0, 1.0, 1.0));

    let lighting_color_2 = point_light.lighting(
      sphere, 
      &Point::new(1.1, 0.0, 0.0), 
      &eye_v, 
      &normal, 
      false
    );

    assert!(lighting_color_2 == Color::new(0.0, 0.0, 0.0, 1.0));
  }

  #[test]
  fn pattern_position_with_object_transformation() {
    let material = Material::solid(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0),  Matrix4x4::identity());
    let sphere = &Sphere::new(Matrix4x4::scale(2.0, 2.0, 2.0), material);

    let pattern_point = sphere.get_material().convert_point(sphere, &Point::new(2.0, 3.0, 4.0));

    assert_eq!(pattern_point.x, 1.0);
    assert_eq!(pattern_point.y, 1.5);
    assert_eq!(pattern_point.z, 2.0);
  }

  #[test]
  fn lighting_pattern_with_pattern_transformation() {
    let material = Material::solid(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0),  Matrix4x4::scale(2.0, 2.0, 2.0));
    let sphere = &Sphere::new(Matrix4x4::identity(), material);

    let pattern_point = sphere.get_material().convert_point(sphere, &Point::new(2.0, 3.0, 4.0));

    assert_eq!(pattern_point.x, 1.0);
    assert_eq!(pattern_point.y, 1.5);
    assert_eq!(pattern_point.z, 2.0);
  }

  #[test]
  fn lighting_pattern_with_both_object_and_pattern_transformation() {
    let material = Material::solid(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::translate(0.5, 1.0, 1.5));
    let sphere = &Sphere::new(Matrix4x4::scale(2.0, 2.0, 2.0), material);
    
    let pattern_point = sphere.get_material().convert_point(sphere, &Point::new(2.5, 3.0, 3.5));

    assert_eq!(pattern_point.x, 0.75);
    assert_eq!(pattern_point.y, 0.5);
    assert_eq!(pattern_point.z, 0.25);
  }
}
