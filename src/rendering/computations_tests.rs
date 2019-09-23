#[cfg(test)]
mod tests {
  use crate::rendering::shape::Shape;
  use crate::rendering::Sphere;
  use crate::rendering::Plane;

  use crate::rendering::Material;

  use crate::rendering::pattern::Pattern;
  use crate::rendering::SolidPattern;

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
    let pattern = &SolidPattern::new(Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let material = Material::new(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, pattern);
    let sphere = Sphere::new(1, transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let mut intersection = Intersection::new(4.0, &sphere);
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
  fn computed_hit_from_inside_of_object() {
    let transform = Matrix4x4::identity();
    let pattern = &SolidPattern::new(Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let material = Material::new(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, pattern);
    let sphere = Sphere::new(1, transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    
    let mut intersection = Intersection::new(4.0, &sphere);
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    assert!(computations.inside == false);
  }

  #[test]
  fn computed_hit_from_outside_of_object() {
    let transform = Matrix4x4::identity();
    let pattern = &SolidPattern::new(Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let material = Material::new(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, pattern);
    let sphere = Sphere::new(1, transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, 0.0), &Vector::new(0.0, 0.0, 1.0));

    let mut intersection = Intersection::new(1.0, &sphere);
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
    let pattern = &SolidPattern::new(Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let material = Material::new(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, pattern);
    let sphere = Sphere::new(1, transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let mut intersection = Intersection::new(5.0, &sphere);
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    assert!(computations.over_point.z < 0.0001);
    assert!(computations.point.z > computations.over_point.z);
  }

  #[test]
  fn computations_do_not_compute_reflection_vector_for_non_reflective_objects() {
    let transform = Matrix4x4::identity();
    let pattern = &SolidPattern::new(Color::new(1.0, 1.0, 1.0, 0.0), Matrix4x4::identity());
    let material = Material::new(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, pattern);
    let plane = Plane::new(1, transform, material);

    let ray = Ray::new(&Point::new(0.0, 1.0, -1.0), &Vector::new(0.0, -(2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0));

    let mut intersection = Intersection::new((2.0 as f64).sqrt(), &plane);
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
    let pattern = &SolidPattern::new(Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let material = Material::new(0.1, 0.9, 0.9, 200.0, 1.0, 0.0, 1.0, pattern);
    let plane = Plane::new(1, transform, material);

    let ray = Ray::new(&Point::new(0.0, 1.0, -1.0), &Vector::new(0.0, -(2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0));

    let mut intersection = Intersection::new((2.0 as f64).sqrt(), &plane);
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
    let pattern = &SolidPattern::new(Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let material = Material::new(0.1, 0.9, 0.9, 200.0, 0.0, 1.0, 1.5, pattern);
    let sphere = Sphere::new(1, transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let mut intersection = Intersection::new(5.0, &sphere);
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    assert!(computations.under_point.z > (0.0001 / 2.0));
    assert!(computations.point.z < computations.under_point.z);
  }

  #[test]
  fn calculating_n1_and_n2_values_at_intersections() {
    let transform = Matrix4x4::scale(2.0, 2.0, 2.0);
    let pattern = &SolidPattern::new(Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let material = Material::new(0.1, 0.9, 0.9, 200.0, 0.0, 1.0, 1.5, pattern);
    let sphere_1 = Sphere::new(1, transform, material);

    let transform = Matrix4x4::translate(0.0, 0.0, -0.25);
    let pattern = &SolidPattern::new(Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let material = Material::new(0.1, 0.9, 0.9, 200.0, 0.0, 1.0, 2.0, pattern);
    let sphere_2 = Sphere::new(2, transform, material);

    let transform = Matrix4x4::translate(0.0, 0.0, 0.25);
    let pattern = &SolidPattern::new(Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let material = Material::new(0.1, 0.9, 0.9, 200.0, 0.0, 1.0, 2.5, pattern);
    let sphere_3 = Sphere::new(3, transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -4.0), &Vector::new(0.0, 0.0, 1.0));

    let mut intersection_1 = Intersection::new(2.0, &sphere_1);
    let mut intersection_2 = Intersection::new(2.75, &sphere_2);
    let mut intersection_3 = Intersection::new(3.25, &sphere_3);
    let mut intersection_4 = Intersection::new(4.75, &sphere_2);
    let mut intersection_5 = Intersection::new(5.25, &sphere_3);
    let mut intersection_6 = Intersection::new(6.0, &sphere_1);
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
