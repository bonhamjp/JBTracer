#[cfg(test)]
mod tests {
  use std::f64;

  use crate::rendering::shape::Shape;
  use crate::rendering::Sphere;

  use crate::rendering::Ray;
  use crate::rendering::Intersection;
  
  use crate::rendering::Material;

  use crate::math::tuple::Tuple;
  use crate::math::Point;
  use crate::math::Vector;
  
  use crate::math::Color;

  use crate::math::Matrix4x4;

  #[test]
  fn default_spehere_creates_sphere_with_default_transform_and_material() {
    let sphere = Sphere::default();

    assert!(sphere.transform == Matrix4x4::identity());
    assert!(sphere.material == Material::default());
  }

  #[test]
  fn sphere_created_with_transform_and_material() {
    let sphere = Sphere::new(
      Matrix4x4::translate(5.0, -3.0, 2.0), 
      Material::new(Color::new(0.0, 0.5, 1.0, 1.0), 0.2, 0.3, 0.4, 0.5)
    );

    assert!(sphere.transform == Matrix4x4::translate(5.0, -3.0, 2.0));
    assert!(sphere.material == Material::new(Color::new(0.0, 0.5, 1.0, 1.0), 0.2, 0.3, 0.4, 0.5));
  }

  #[test]
  fn ray_intersects_sphere_at_two_point() {
    let sphere = Sphere::default();

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray);

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 4.0);
    assert!(intersections[1].t == 6.0);
  }

  #[test]
  fn ray_intersects_sphere_at_tangent() {
    let sphere = Sphere::default();

    let ray = Ray::new(&Point::new(0.0, 1.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray);

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 5.0);
    assert!(intersections[1].t == 5.0);
  }

  #[test]
  fn ray_misses_sphere() {
    let sphere = Sphere::default();

    let ray = Ray::new(&Point::new(0.0, 2.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray);

    assert!(intersections.len() == 0);
  }

  #[test]
  fn ray_originates_in_sphere() {
    let sphere = Sphere::default();

    let ray = Ray::new(&Point::new(0.0, 0.0, 0.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray);
    
    assert!(intersections.len() == 2);
    assert!(intersections[0].t == -1.0);
    assert!(intersections[1].t == 1.0);
  }

  #[test]
  fn ray_points_away_from_sphere() {
    let sphere = Sphere::default();

    let ray = Ray::new(&Point::new(0.0, 0.0, 5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray);
    
    assert!(intersections.len() == 2);
    assert!(intersections[0].t == -6.0);
    assert!(intersections[1].t == -4.0);
  }

  #[test]
  fn intersects_stores_reference_to_intersected_object() {
    let sphere = Sphere::default();

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray);
    
    assert!(intersections.len() == 2);
    
    let collided_object_1 = intersections[0].object.as_any().downcast_ref::<Sphere>().unwrap();  
    let collided_object_2 = intersections[1].object.as_any().downcast_ref::<Sphere>().unwrap();

    assert!(sphere.is_eq(intersections[0].object));
    assert!(sphere.is_eq(intersections[1].object));
  }

  #[test]
  fn sphere_transform_defaults_to_identity() {
    let sphere = Sphere::default();

    assert!(sphere.get_transform() == &Matrix4x4::identity());
  }

  #[test]
  fn sphere_transformation_can_be_set() {
    let mut sphere = Sphere::default();

    sphere.set_transform(Matrix4x4::scale(0.2, 0.3, 99.9));

    assert!(sphere.get_transform() == &Matrix4x4::scale(0.2, 0.3, 99.9));
  }
  
  #[test]
  fn intersecting_a_scaled_sphere() {
    let mut sphere = Sphere::default();

    sphere.set_transform(Matrix4x4::scale(2.0, 2.0, 2.0));

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray);

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 3.0);
    assert!(intersections[1].t == 7.0);
  }

  #[test]
  fn intersecting_a_translated_sphere() {
    let mut sphere = Sphere::default();

    sphere.set_transform(Matrix4x4::translate(5.0, 0.0, 0.0));

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray);

    assert!(intersections.len() == 0);
  }

  #[test]
  fn normal_on_sphere_at_furthest_point_along_x_axis() {
    let sphere = Sphere::default();

    let normal = sphere.normal_at(&Point::new(1.0, 0.0, 0.0));

    assert_eq!(normal.get_quad(), (1.0, 0.0, 0.0, 0.0));
  }

  #[test]
  fn normal_on_sphere_at_furthest_point_along_y_axis() {
    let sphere = Sphere::default();

    let normal = sphere.normal_at(&Point::new(0.0, 1.0, 0.0));

    assert_eq!(normal.get_quad(), (0.0, 1.0, 0.0, 0.0));
  }

  #[test]
  fn normal_on_sphere_at_furthest_point_along_z_axis() {
    let sphere = Sphere::default();

    let normal = sphere.normal_at(&Point::new(0.0, 0.0, 1.0));

    assert_eq!(normal.get_quad(), (0.0, 0.0, 1.0, 0.0));
  }

  #[test]
  fn normal_on_sphere_at_non_axial_point() {
    let sphere = Sphere::default();

    let uniform_offset = (3.0 as f64).sqrt() / 3.0;

    let normal = sphere.normal_at(&Point::new(uniform_offset, uniform_offset, uniform_offset));

    assert_eq!(normal.get_quad(), (uniform_offset, uniform_offset, uniform_offset, 0.0));
  }

  #[test]
  fn normal_on_translated_sphere() {
    let mut sphere = Sphere::default();

    sphere.set_transform(Matrix4x4::translate(0.0, 1.0, 0.0));

    let normal = sphere.normal_at(&Point::new(0.0, 1.70711, -0.70711));

    assert_eq!(normal.get_quad(), (0.0, 0.7071067811865475, -0.7071067811865476, 0.0));
  }

  #[test]
  fn normal_on_transformed_sphere() {
    let mut sphere = Sphere::default();

    let z_rotation = Matrix4x4::rotate_z(f64::consts::PI / 5.0);
    let scaling = Matrix4x4::scale(1.0, 0.5, 1.0);

    let transform = scaling.mult4x4(&z_rotation);

    sphere.set_transform(transform);
  
    let normal = sphere.normal_at(&Point::new(0.0, (2.0 as f64).sqrt() / 2.0, -(2.0 as f64).sqrt() / 2.0));

    assert_eq!(normal.get_quad(), (0.0, 0.9701425001453319, -0.24253562503633294, 0.0));
  }

  #[test]
  fn sphere_material_initialized_to_default() {
    let sphere = Sphere::default();

    let sphere_material = sphere.get_material();

    assert!(sphere_material == &Material::default());
  }

  #[test]
  fn sphere_material_can_be_set() {
    let mut sphere = Sphere::default();

    sphere.set_material(Material::new(Color::new(0.5, 0.1, 0.75, 1.0), 0.1, 0.2, 0.3, 0.4));

    assert!(sphere.get_material() == &Material::new(Color::new(0.5, 0.1, 0.75, 1.0), 0.1, 0.2, 0.3, 0.4));
  }
}
