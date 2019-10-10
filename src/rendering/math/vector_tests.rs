#[cfg(test)]
mod tests {
  use std::f64;

  use crate::rendering::math::tuple::Tuple;
  use crate::rendering::math::Point;
  use crate::rendering::math::Vector;

  #[test]
  fn new_sets_values() {
    let vector = Vector::new(1.1, 2.2, 3.3);

    assert_eq!(vector.x, 1.1);
    assert_eq!(vector.y, 2.2);
    assert_eq!(vector.z, 3.3);
    assert_eq!(vector.w, 0.0);
  }

  #[test]
  fn empty_sets_0_values() {
    let vector = Vector::empty();

    assert_eq!(vector.x, 0.0);
    assert_eq!(vector.y, 0.0);
    assert_eq!(vector.z, 0.0);
    assert_eq!(vector.w, 0.0);
  }

  #[test]
  fn x_axis_sets_normalized_x_axis() {
    let vector = Vector::x_axis();

    assert_eq!(vector.x, 1.0);
    assert_eq!(vector.y, 0.0);
    assert_eq!(vector.z, 0.0);
    assert_eq!(vector.w, 0.0);
  }

  #[test]
  fn y_axis_sets_normalized_y_axis() {
    let vector = Vector::y_axis();

    assert_eq!(vector.x, 0.0);
    assert_eq!(vector.y, 1.0);
    assert_eq!(vector.z, 0.0);
    assert_eq!(vector.w, 0.0);
  }

  #[test]
  fn z_axis_sets_normalized_z_axis() {
    let vector = Vector::z_axis();

    assert_eq!(vector.x, 0.0);
    assert_eq!(vector.y, 0.0);
    assert_eq!(vector.z, 1.0);
    assert_eq!(vector.w, 0.0);
  }

  #[test]
  fn get_quad_returns_a_4_tuple_with_elements_and_w() {
    let vector_1 = Vector::new(9.9, 8.8, 7.7);

    assert_eq!(vector_1.get_quad(), (9.9, 8.8, 7.7, 0.0));

    let vector_2 = Vector::empty();

    assert_eq!(vector_2.get_quad(), (0.0, 0.0, 0.0, 0.0));
  }

  #[test]
  fn adding_a_vector_adds_each_component_returning_vector() {
    let vector_1 = Vector::new(1.1, 2.2, 3.3);
    let vector_2 = Vector::new(33.0, 33.0, 33.0);

    let sum = vector_1.add_vector(&vector_2);

    assert_eq!(sum.get_quad(), (34.1, 35.2, 36.3, 0.0));
  }

  #[test]
  fn adding_a_point_adds_each_component_returning_point() {
    let vector = Vector::new(1.1, 2.2, 3.3);
    let point = Point::new(2.0, 2.0, 2.0);

    let sum = vector.add_point(&point);

    assert_eq!(sum.get_quad(), (3.1, 4.2, 5.3, 1.0));
  }

    #[test]
  fn subtracting_a_vector_subtracts_each_component_returning_vector() {
    let vector_1 = Vector::new(10.1, 20.2, 30.3);
    let vector_2 = Vector::new(1.1, 1.2, 1.3);

    let difference = vector_1.subtract_vector(&vector_2);

    assert_eq!(difference.get_quad(), (9.0, 19.0, 29.0, 0.0));
  }

  #[test]
  fn multiplying_by_number_multiplies_each_component() {
    let vector = Vector::new(1.2, 2.2, 3.2);

    let product = vector.multiply(10.0);

    assert_eq!(product.get_quad(), (12.0, 22.0, 32.0, 0.0));
  }

  #[test]
  fn dividing_by_number_divides_each_component() {
    let vector = Vector::new(10.0, 20.0, 30.0);

    let quotient = vector.divide(10.0);

    assert_eq!(quotient.get_quad(), (1.0, 2.0, 3.0, 0.0));
  }

  #[test]
  fn normalize_divides_by_magnitude() {
    let vector = Vector::new(10.0, 20.0, 30.0);

    let normalized = vector.normalize();

    assert_eq!(normalized.get_quad(), (0.2672612419124244, 0.5345224838248488, 0.8017837257372731, 0.0));
  }

  #[test]
  fn cross_finds_cross_product_of_two_vectors() {
    let vector_1 = Vector::new(1.0, 2.0, 3.0);
    let vector_2 = Vector::new(2.0, 3.0, 4.0);

    let cross_product = vector_1.cross(&vector_2);

    assert_eq!(cross_product.get_quad(), (-1.0, 2.0, -1.0, 0.0));
  }

  #[test]
  fn dot_with_vector_multiplies_elements() {
    let vector_1 = Vector::new(1.2, 1.3, 1.4);
    let vector_2 = Vector::new(2.1, 3.1, 4.1);

    let dot_product = vector_1.dot(&vector_2);

    assert_eq!(dot_product, 12.29);
  }

  #[test]
  fn magnitude_uses_pythag_theorum() {
    let vector = Vector::new(2.0, 2.0, 2.0);

    let magnitude = vector.magnitude();

    assert_eq!(magnitude, 3.4641016151377544);
  }

  #[test]
  fn dot_with_point_multiplies_elements() {
    let vector = Vector::new(1.2, 1.3, 1.4);
    let point = Point::new(20.0, 30.0, 40.0);

    let dot_product = vector.dot(&point);

    assert_eq!(dot_product, 119.0);
  }

  #[test]
  fn reflecting_vector_45_degrees() {
    let vector = Vector::new(1.0, -1.0, 0.0);
    let normal = Vector::new(0.0, 1.0, 0.0);

    let reflection = vector.reflect(&normal);

    assert_eq!(reflection.get_quad(), (1.0, 1.0, 0.0, 0.0));
  }

  #[test]
  fn reflecting_vector_off_slant() {
    let vector = Vector::new(0.0, -1.0, 0.0);
    let normal = Vector::new((2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0, 0.0);

    let reflection = vector.reflect(&normal);

    assert!((reflection.x - 1.0).abs() < 0.0001);
    assert!((reflection.y - 0.0).abs() < 0.0001);
    assert!((reflection.z - 0.0).abs() < 0.0001);
  }
}
