#[cfg(test)]
mod tests {
  use std::f64;

  use crate::rendering::math::tuple::Tuple;
  use crate::rendering::math::Point;
  use crate::rendering::math::Vector;
  
  use crate::rendering::math::Color;

  use crate::rendering::math::Matrix4x4;

  use crate::rendering::shapes::shape::Shape;
  use crate::rendering::shapes::Sphere;

  use crate::rendering::Ray;
  
  use crate::rendering::Material;

  #[test]
  fn sphere_created_with_transform_and_material() {
    let transform = Matrix4x4::translate(5.0, -3.0, 2.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(transform, material);

    assert!(sphere.transform == Matrix4x4::translate(5.0, -3.0, 2.0));
    assert!(sphere.material.ambient == 0.1);
    assert!(sphere.material.diffuse == 0.9);
    assert!(sphere.material.specular == 0.9);
    assert!(sphere.material.shininess == 200.0);
  }

  #[test]
  fn ray_intersects_sphere_at_two_point() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 4.0);
    assert!(intersections[1].t == 6.0);
  }

  #[test]
  fn ray_intersects_sphere_at_tangent() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(transform, material);

    let ray = Ray::new(&Point::new(0.0, 1.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 5.0);
    assert!(intersections[1].t == 5.0);
  }

  #[test]
  fn ray_misses_sphere() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(transform, material);

    let ray = Ray::new(&Point::new(0.0, 2.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);
  }

  #[test]
  fn ray_originates_in_sphere() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, 0.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());
    
    assert!(intersections.len() == 2);
    assert!(intersections[0].t == -1.0);
    assert!(intersections[1].t == 1.0);
  }

  #[test]
  fn ray_points_away_from_sphere() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, 5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());
    
    assert!(intersections.len() == 2);
    assert!(intersections[0].t == -6.0);
    assert!(intersections[1].t == -4.0);
  }

  #[test]
  fn intersects_stores_reference_to_intersected_object() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());
    
    assert!(intersections.len() == 2);

    assert!(sphere.is_eq(intersections[0].object));
    assert!(sphere.is_eq(intersections[1].object));
  }
  
  #[test]
  fn intersecting_a_scaled_sphere() {
    let transform = Matrix4x4::scale(2.0, 2.0, 2.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 3.0);
    assert!(intersections[1].t == 7.0);
  }

  #[test]
  fn intersecting_a_translated_sphere() {
    let transform = Matrix4x4::translate(5.0, 0.0, 0.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);
  }

  #[test]
  fn normal_on_sphere_at_furthest_point_along_x_axis() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(transform, material);

    let normal = sphere.normal_at(&Point::new(1.0, 0.0, 0.0));

    assert_eq!(normal.get_quad(), (1.0, 0.0, 0.0, 0.0));
  }

  #[test]
  fn normal_on_sphere_at_furthest_point_along_y_axis() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(transform, material);

    let normal = sphere.normal_at(&Point::new(0.0, 1.0, 0.0));

    assert_eq!(normal.get_quad(), (0.0, 1.0, 0.0, 0.0));
  }

  #[test]
  fn normal_on_sphere_at_furthest_point_along_z_axis() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(transform, material);

    let normal = sphere.normal_at(&Point::new(0.0, 0.0, 1.0));

    assert_eq!(normal.get_quad(), (0.0, 0.0, 1.0, 0.0));
  }

  #[test]
  fn normal_on_sphere_at_non_axial_point() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(transform, material);

    let uniform_offset = (3.0 as f64).sqrt() / 3.0;

    let normal = sphere.normal_at(&Point::new(uniform_offset, uniform_offset, uniform_offset));

    assert_eq!(normal.get_quad(), (uniform_offset, uniform_offset, uniform_offset, 0.0));
  }

  #[test]
  fn normal_on_translated_sphere() {
    let transform = Matrix4x4::translate(0.0, 1.0, 0.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(transform, material);

    let normal = sphere.normal_at(&Point::new(0.0, 1.70711, -0.70711));

    assert_eq!(normal.get_quad(), (0.0, 0.7071067811865475, -0.7071067811865476, 0.0));
  }

  #[test]
  fn normal_on_transformed_sphere() {
    let transform = Matrix4x4::scale(1.0, 0.5, 1.0).mult4x4(& Matrix4x4::rotate_z(f64::consts::PI / 5.0));
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(transform, material);
  
    let normal = sphere.normal_at(&Point::new(0.0, (2.0 as f64).sqrt() / 2.0, -(2.0 as f64).sqrt() / 2.0));

    assert_eq!(normal.get_quad(), (0.0, 0.9701425001453319, -0.24253562503633294, 0.0));
  }
}
