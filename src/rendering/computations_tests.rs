#[cfg(test)]
mod tests {
  use std::f64;

  use crate::rendering::shape::Shape;
  use crate::rendering::Sphere;
  use crate::rendering::Plane;
  use crate::rendering::SmoothTriangle;

  use crate::rendering::Material;

  use crate::rendering::Intersection;
  use crate::rendering::Ray;
  use crate::rendering::Computations;

  use crate::math::tuple::Tuple;
  use crate::math::Point;
  use crate::math::Vector;

  use crate::math::Color;

  use crate::math::Matrix4x4;

  #[test]
  fn new_computation_holds_computed_state_of_intersection() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(1, transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let mut intersection = Intersection::new(4.0, &sphere, Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    assert!(computations.t == intersections[0].t);
    assert!(computations.object.is_eq(&sphere));
    assert!(computations.point == Point::new(0.0, 0.0, -1.0));
    assert!(computations.eye_v == Vector::new(0.0, 0.0, -1.0));
    assert!(computations.normal == Vector::new(0.0, 0.0, -1.0));
  }

  #[test]
  fn new_computation_from_translated_position_correctly_calculates_normal() {
    let transform = Matrix4x4::translate(5.0, 0.0, 0.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(1, transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let mut intersection = Intersection::new(4.0, &sphere, Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    assert!(computations.inside == false);
  }

  #[test]
  fn new_computation_from_object_with_interpolated_normals_calculates_normal() {
    let transform = Matrix4x4::translate(0.0, 0.0, 0.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    
    let point_1 = Point::new(0.0, 1.0, 0.0);
    let point_2 = Point::new(-1.0, 0.0, 0.0);
    let point_3 = Point::new(1.0, 0.0, 0.0);
    let normal_1 = Vector::new(0.0, 1.0, 0.0);
    let normal_2 = Vector::new(-1.0, 0.0, 0.0);
    let normal_3 = Vector::new(1.0, 0.0, 0.0);
    let smooth_triangle = SmoothTriangle::new(1, point_1, point_2, point_3, normal_1, normal_2, normal_3, transform, material);

    let ray = Ray::new(&Point::new(-0.2, 0.3, -2.0), &Vector::new(0.0, 0.0, 1.0));

    let mut intersection = Intersection::new_with_uv(1.0, &smooth_triangle, Matrix4x4::identity(), Matrix4x4::identity(), 0.45, 0.25);
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    assert_eq!(computations.normal.x, -0.5547001962252291);
    assert_eq!(computations.normal.y, 0.8320502943378437);
    assert_eq!(computations.normal.z, 0.0);
  }

  #[test]
  fn computed_hit_from_inside_of_object() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(1, transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let world_to_object = Matrix4x4::translate(0.25, 0.25, 0.25).inverse();
    let normal_to_world = world_to_object.transpose();

    let mut intersection = Intersection::new(4.0, &sphere, world_to_object, normal_to_world);
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    assert_eq!(computations.normal.x, -0.19245008972987526);
    assert_eq!(computations.normal.y, -0.19245008972987526);
    assert_eq!(computations.normal.z, -0.9622504486493763);
  }

  #[test]
  fn computed_hit_from_outside_of_object() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(1, transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, 0.0), &Vector::new(0.0, 0.0, 1.0));

    let mut intersection = Intersection::new(1.0, &sphere, Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    assert!(computations.t == intersections[0].t);
    assert!(computations.object.is_eq(&sphere));
    assert!(computations.point == Point::new(0.0, 0.0, 1.0));
    assert!(computations.eye_v == Vector::new(0.0, 0.0, -1.0));
    assert!(computations.inside);
    assert!(computations.normal == Vector::new(0.0, 0.0, -1.0));
  }

  #[test]
  fn over_point_slightly_above_hit_is_computed() {
    let transform = Matrix4x4::translate(0.0, 0.0, 1.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(1, transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let mut intersection = Intersection::new(5.0, &sphere, Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    assert!(computations.over_point.z < 0.0001);
    assert!(computations.point.z > computations.over_point.z);
  }

  #[test]
  fn computations_do_not_compute_reflection_vector_for_non_reflective_objects() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let plane = Plane::new(1, transform, material);

    let ray = Ray::new(&Point::new(0.0, 1.0, -1.0), &Vector::new(0.0, -(2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0));

    let mut intersection = Intersection::new((2.0 as f64).sqrt(), &plane, Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    assert_eq!(computations.reflect_v.x, 0.0);
    assert_eq!(computations.reflect_v.y, 0.0);
    assert_eq!(computations.reflect_v.z, 0.0);
  }

  #[test]
  fn computations_compute_reflection_vector_for_reflective_objects() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 1.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let plane = Plane::new(1, transform, material);

    let ray = Ray::new(&Point::new(0.0, 1.0, -1.0), &Vector::new(0.0, -(2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0));

    let mut intersection = Intersection::new((2.0 as f64).sqrt(), &plane, Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    assert_eq!(computations.reflect_v.x, 0.0);
    assert_eq!(computations.reflect_v.y, (2.0 as f64).sqrt() / 2.0);
    assert_eq!(computations.reflect_v.z, (2.0 as f64).sqrt() / 2.0);
  }

  #[test]
  fn under_point_slightly_under_hit_is_computed() {
    let transform = Matrix4x4::translate(0.0, 0.0, 1.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(1, transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let mut intersection = Intersection::new(5.0, &sphere, Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    assert!(computations.under_point.z > (0.0001 / 2.0));
    assert!(computations.point.z < computations.under_point.z);
  }

  #[test]
  fn calculating_n1_and_n2_values_at_intersections() {
    let transform = Matrix4x4::scale(2.0, 2.0, 2.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 1.0, 1.5, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_1 = Sphere::new(1, transform, material);

    let transform = Matrix4x4::translate(0.0, 0.0, -0.25);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 1.0, 2.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = Sphere::new(2, transform, material);

    let transform = Matrix4x4::translate(0.0, 0.0, 0.25);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 1.0, 2.5, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_3 = Sphere::new(3, transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -4.0), &Vector::new(0.0, 0.0, 1.0));

    let mut intersection_1 = Intersection::new(2.0, &sphere_1, Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersection_2 = Intersection::new(2.75, &sphere_2, Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersection_3 = Intersection::new(3.25, &sphere_3, Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersection_4 = Intersection::new(4.75, &sphere_2, Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersection_5 = Intersection::new(5.25, &sphere_3, Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersection_6 = Intersection::new(6.0, &sphere_1, Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersections = Vec::new();
    intersections.push(intersection_1);
    intersections.push(intersection_2);
    intersections.push(intersection_3);
    intersections.push(intersection_4);
    intersections.push(intersection_5);
    intersections.push(intersection_6);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);
    assert_eq!(computations.n1, 1.0);
    assert_eq!(computations.n2, 1.5);

    let computations = Computations::new(&intersections[1], &ray, &intersections);
    assert_eq!(computations.n1, 1.5);
    assert_eq!(computations.n2, 2.0);

    let computations = Computations::new(&intersections[2], &ray, &intersections);
    assert_eq!(computations.n1, 2.0);
    assert_eq!(computations.n2, 2.5);

    let computations = Computations::new(&intersections[3], &ray, &intersections);
    assert_eq!(computations.n1, 2.5);
    assert_eq!(computations.n2, 2.5);

    let computations = Computations::new(&intersections[4], &ray, &intersections);
    assert_eq!(computations.n1, 2.5);
    assert_eq!(computations.n2, 1.5);

    let computations = Computations::new(&intersections[5], &ray, &intersections);
    assert_eq!(computations.n1, 1.5);
    assert_eq!(computations.n2, 1.0);
  }
}
