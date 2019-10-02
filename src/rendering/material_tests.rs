#[cfg(test)]
mod tests {
  use crate::rendering::shape::Shape;
  use crate::rendering::Sphere;

  use crate::rendering::Material;

  use crate::math::tuple::Tuple;
  use crate::math::Point;
  use crate::math::Vector;

  use crate::math::Color;

  use crate::math::Matrix4x4;

  #[test]
  fn sets_values() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());

    assert_eq!(material.ambient, 0.1);
    assert_eq!(material.diffuse, 0.9);
    assert_eq!(material.specular, 0.9);
    assert_eq!(material.shininess, 200.0);
    assert_eq!(material.reflectiveness, 0.0);
    assert_eq!(material.transparency, 0.0);
    assert_eq!(material.refractive_index, 1.0);
    assert!(material.color_1 == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(material.transform == Matrix4x4::identity());
  }

  #[test]
  fn solid_material_always_returns_single_color() {
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.0, 1.0, 0.0, 1.0), Matrix4x4::identity());
    
    let sphere = &Sphere::new(1, Matrix4x4::identity(), material);

    let solid_material = sphere.get_material();

    assert!(solid_material.color_at(sphere as &dyn Shape, &Point::new(3.0, 0.0, 0.0)) == Color::new(0.0, 1.0, 0.0, 1.0));
    assert!(solid_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 3.0, 0.0)) == Color::new(0.0, 1.0, 0.0, 1.0));
    assert!(solid_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 0.0, 3.0)) == Color::new(0.0, 1.0, 0.0, 1.0));
    assert!(solid_material.color_at(sphere as &dyn Shape, &Point::new(3.0, 2.0, 1.0)) == Color::new(0.0, 1.0, 0.0, 1.0));
    assert!(solid_material.color_at(sphere as &dyn Shape, &Point::new(1.0, 3.0, 2.0)) == Color::new(0.0, 1.0, 0.0, 1.0));
    assert!(solid_material.color_at(sphere as &dyn Shape, &Point::new(2.0, 1.0, 3.0)) == Color::new(0.0, 1.0, 0.0, 1.0));
  }

  #[test]
  fn checker_material_created_with_two_colors_and_transform() {
    let material = Material::checkered(
      0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, 
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );
    
    let sphere = &Sphere::new(1, Matrix4x4::identity(), material);

    let checkered_material = sphere.get_material();

    assert!(checkered_material.color_1 == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(checkered_material.color_2 == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(checkered_material.transform == Matrix4x4::identity());
  }

  #[test]
  fn checker_material_repeats_along_x() {
    let material = Material::checkered(
      0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, 
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );
    
    let sphere = &Sphere::new(1, Matrix4x4::identity(), material);

    let checkered_material = sphere.get_material();

    assert!(checkered_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(checkered_material.color_at(sphere as &dyn Shape, &Point::new(0.99, 0.0, 0.0)) == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(checkered_material.color_at(sphere as &dyn Shape, &Point::new(1.01, 0.0, 0.0)) == Color::new(0.0, 0.0, 0.0, 1.0));
  }

  #[test]
  fn checker_material_repeats_along_y() {
    let material = Material::checkered(
      0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, 
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );
    
    let sphere = &Sphere::new(1, Matrix4x4::identity(), material);

    let checkered_material = sphere.get_material();

    assert!(checkered_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(checkered_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 0.99, 0.0)) == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(checkered_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 1.01, 0.0)) == Color::new(0.0, 0.0, 0.0, 1.0));
  }

  #[test]
  fn checker_material_repeats_along_z() {
    let material = Material::checkered(
      0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, 
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );
    
    let sphere = &Sphere::new(1, Matrix4x4::identity(), material);

    let checkered_material = sphere.get_material();

    assert!(checkered_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(checkered_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 0.0, 0.99)) == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(checkered_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 0.0, 1.01)) == Color::new(0.0, 0.0, 0.0, 1.0));
  }

  #[test]
  fn stripe_material_returns_constant_color_if_only_y_changes() {
    let material = Material::striped(
      0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, 
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );
    
    let sphere = &Sphere::new(1, Matrix4x4::identity(), material);

    let striped_material = sphere.get_material();

    assert!(striped_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(striped_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 0.0, 1.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(striped_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 0.0, 2.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
  }

  #[test]
  fn stripe_material_returns_constant_color_if_only_z_changes() {
    let material = Material::striped(
      0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, 
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );
    
    let sphere = &Sphere::new(1, Matrix4x4::identity(), material);

    let striped_material = sphere.get_material();

    assert!(striped_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(striped_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 1.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(striped_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 2.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
  }

  #[test]
  fn stripe_material_alternates_as_x_changes() {
    let material = Material::striped(
      0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, 
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );
    
    let sphere = &Sphere::new(1, Matrix4x4::identity(), material);

    let striped_material = sphere.get_material();

    assert!(striped_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(striped_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));    
    assert!(striped_material.color_at(sphere as &dyn Shape, &Point::new(1.0, 0.0, 0.0)) == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(striped_material.color_at(sphere as &dyn Shape, &Point::new(-0.1, 0.0, 0.0)) == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(striped_material.color_at(sphere as &dyn Shape, &Point::new(-1.0, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(striped_material.color_at(sphere as &dyn Shape, &Point::new(-1.1, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
  }

  #[test]
  fn ring_material_linearly_interpolates_between_colors() {
    let material = Material::ringed(
      0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, 
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );
    
    let sphere = &Sphere::new(1, Matrix4x4::identity(), material);

    let ringed_material = sphere.get_material();

    assert!(ringed_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 0.0, 0.0)) == Color::new(0.0, 0.0, 0.0, 1.0));
    assert!(ringed_material.color_at(sphere as &dyn Shape, &Point::new(1.0, 0.0, 0.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
    assert!(ringed_material.color_at(sphere as &dyn Shape, &Point::new(0.0, 0.0, 1.0)) == Color::new(1.0, 1.0, 1.0, 1.0));
  }

  #[test]
  fn gradient_material_linearly_interpolates_between_colors() {
    let material = Material::gradient(
      0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, 
      Color::new(1.0, 1.0, 1.0, 1.0), 
      Color::new(0.0, 0.0, 0.0, 1.0), 
      Matrix4x4::identity()
    );
    
    let sphere = &Sphere::new(1, Matrix4x4::identity(), material);

    let gradient_material = sphere.get_material();

    assert!(gradient_material.color_at(sphere as &dyn Shape, &Point::new(0.25, 0.0, 0.0)) == Color::new(0.75, 0.75, 0.75, 1.0));
    assert!(gradient_material.color_at(sphere as &dyn Shape, &Point::new(0.5, 0.0, 0.0)) == Color::new(0.5, 0.5, 0.5, 1.0));
    assert!(gradient_material.color_at(sphere as &dyn Shape, &Point::new(0.75, 0.0, 0.0)) == Color::new(0.25, 0.25, 0.25, 1.0));
  }
}
