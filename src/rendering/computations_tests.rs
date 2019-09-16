#[cfg(test)]
mod tests {
  use crate::rendering::shape::Shape;
  use crate::rendering::Sphere;
    
  use crate::rendering::Material;

  use crate::rendering::Intersection;
  use crate::rendering::Ray;
  use crate::rendering::Computations;

  use crate::math::tuple::Tuple;
  use crate::math::Point;
  use crate::math::Vector;

  use crate::math::Matrix4x4;

  #[test]
  fn new_computation_holds_computed_state_of_intersection() {
    let sphere = Sphere::default();

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersection = Intersection::new(4.0, &sphere);

    let computations = Computations::new(&intersection, &ray);

    assert!(computations.t == intersection.t);
    assert!(computations.object.is_eq(&sphere));
    assert!(computations.point == Point::new(0.0, 0.0, -1.0));
    assert!(computations.eye_v == Vector::new(0.0, 0.0, -1.0));
    assert!(computations.normal == Vector::new(0.0, 0.0, -1.0));
  }

  #[test]
  fn computed_hit_from_inside_of_object() {
    let sphere = Sphere::default();

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersection = Intersection::new(4.0, &sphere);

    let computations = Computations::new(&intersection, &ray);

    assert!(computations.inside == false);
  }

  #[test]
  fn computed_hit_from_outside_of_object() {
    let sphere = Sphere::default();

    let ray = Ray::new(&Point::new(0.0, 0.0, 0.0), &Vector::new(0.0, 0.0, 1.0));

    let intersection = Intersection::new(1.0, &sphere);

    let computations = Computations::new(&intersection, &ray);

    assert!(computations.t == intersection.t);
    assert!(computations.object.is_eq(&sphere));
    assert!(computations.point == Point::new(0.0, 0.0, 1.0));
    assert!(computations.eye_v == Vector::new(0.0, 0.0, -1.0));
    assert!(computations.inside);
    assert!(computations.normal == Vector::new(0.0, 0.0, -1.0));
  }

  #[test]
  fn overpoint_slightly_above_hit_is_computed() {
    let sphere = Sphere::new(Matrix4x4::translate(0.0, 0.0, 1.0), Material::default());

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersection = Intersection::new(5.0, &sphere);

    let computations = Computations::new(&intersection, &ray);

    assert!(computations.over_point.z < 0.0001);
    assert!(computations.point.z > computations.over_point.z);
  }
}
