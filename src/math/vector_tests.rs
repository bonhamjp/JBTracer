#[cfg(test)]
mod tests {
  #[test]
  fn new_sets_values() {
    use crate::math::Vector;

    let vector = Vector::new(1.1, 2.2, 3.3);

    assert_eq!(vector.x, 1.1);
    assert_eq!(vector.y, 2.2);
    assert_eq!(vector.z, 3.3);
  }

  #[test]
  fn empty_sets_0_values() {
    use crate::math::Vector;

    let vector = Vector::empty();

    assert_eq!(vector.x, 0.0);
    assert_eq!(vector.y, 0.0);
    assert_eq!(vector.z, 0.0);
  }

  #[test]
  fn x_axis_sets_normalized_x_axis() {
    use crate::math::Vector;

    let vector = Vector::x_axis();

    assert_eq!(vector.x, 1.0);
    assert_eq!(vector.y, 0.0);
    assert_eq!(vector.z, 0.0);
  }

  #[test]
  fn y_axis_sets_normalized_y_axis() {
    use crate::math::Vector;

    let vector = Vector::y_axis();

    assert_eq!(vector.x, 0.0);
    assert_eq!(vector.y, 1.0);
    assert_eq!(vector.z, 0.0);
  }

  #[test]
  fn z_axis_sets_normalized_z_axis() {
    use crate::math::Vector;

    let vector = Vector::z_axis();

    assert_eq!(vector.x, 0.0);
    assert_eq!(vector.y, 0.0);
    assert_eq!(vector.z, 1.0);
  }

  #[test]
  fn w_returns_0() {
    use crate::math::Vector;

    let vector_1 = Vector::new(0.1, 0.2, 0.3);

    assert_eq!(vector_1.w(), 0.0);

    let vector_2 = Vector::empty();

    assert_eq!(vector_2.w(), 0.0);
  }

  #[test]
  fn get_quad_returns_a_4_tuple_with_elements_and_w() {
    use crate::math::tuple::Tuple;
    use crate::math::Vector;

    let vector_1 = Vector::new(9.9, 8.8, 7.7);

    assert_eq!(vector_1.get_quad(), (9.9, 8.8, 7.7, 0.0));

    let vector_2 = Vector::empty();

    assert_eq!(vector_2.get_quad(), (0.0, 0.0, 0.0, 0.0));
  }

  #[test]
  fn adding_a_vector_adds_each_component_returning_vector() {
    use crate::math::tuple::Tuple;
    use crate::math::Vector;

    let vector_1 = Vector::new(1.1, 2.2, 3.3);
    let vector_2 = Vector::new(33.0, 33.0, 33.0);

    let sum = vector_1.add_vector(&vector_2);

    assert_eq!(sum.get_quad(), (34.1, 35.2, 36.3, 0.0));
  }

  #[test]
  fn adding_a_point_adds_each_component_returning_point() {
    use crate::math::tuple::Tuple;
    use crate::math::Vector;
    use crate::math::Point;

    let vector = Vector::new(1.1, 2.2, 3.3);
    let point = Point::new(2.0, 2.0, 2.0);

    let sum = vector.add_point(&point);

    assert_eq!(sum.get_quad(), (3.1, 4.2, 5.3, 1.0));
  }

    #[test]
  fn subtracting_a_vector_subtracts_each_component_returning_vector() {
    use crate::math::tuple::Tuple;
    use crate::math::Vector;

    let vector_1 = Vector::new(10.1, 20.2, 30.3);
    let vector_2 = Vector::new(1.1, 1.2, 1.3);

    let difference = vector_1.subtract_vector(&vector_2);

    assert_eq!(difference.get_quad(), (9.0, 19.0, 29.0, 0.0));
  }

  #[test]
  fn multiplying_by_number_multiplies_each_component() {
    use crate::math::tuple::Tuple;
    use crate::math::Vector;

    let vector = Vector::new(1.2, 2.2, 3.2);

    let product = vector.multiply(10.0);

    assert_eq!(product.get_quad(), (12.0, 22.0, 32.0, 0.0));
  }

  #[test]
  fn dividing_by_number_divides_each_component() {
    use crate::math::tuple::Tuple;
    use crate::math::Vector;

    let vector = Vector::new(10.0, 20.0, 30.0);

    let quotient = vector.divide(10.0);

    assert_eq!(quotient.get_quad(), (1.0, 2.0, 3.0, 0.0));
  }

  #[test]
  fn normalize_divides_by_magnitude() {
    use crate::math::tuple::Tuple;
    use crate::math::Vector;

    let vector = Vector::new(10.0, 20.0, 30.0);

    let normalized = vector.normalize();

    assert_eq!(normalized.get_quad(), (0.2672612419124244, 0.5345224838248488, 0.8017837257372731, 0.0));
  }

  #[test]
  fn cross_finds_cross_product_of_two_vectors() {
    use crate::math::tuple::Tuple;
    use crate::math::Vector;

    let vector_1 = Vector::new(1.0, 2.0, 3.0);
    let vector_2 = Vector::new(2.0, 3.0, 4.0);

    let cross_product = vector_1.cross(&vector_2);

    assert_eq!(cross_product.get_quad(), (-1.0, 2.0, -1.0, 0.0));
  }

  #[test]
  fn dot_with_vector_multiplies_elements() {
    use crate::math::tuple::Tuple;
    use crate::math::Vector;

    let vector_1 = Vector::new(1.2, 1.3, 1.4);
    let vector_2 = Vector::new(2.1, 3.1, 4.1);

    let dot_product = vector_1.dot(&vector_2);

    assert_eq!(dot_product, 12.29);
  }

  #[test]
  fn magnitude_uses_pythag_theorum() {
    use crate::math::tuple::Tuple;
    use crate::math::Vector;

    let vector = Vector::new(2.0, 2.0, 2.0);

    let magnitude = vector.magnitude();

    assert_eq!(magnitude, 3.4641016151377544);
  }

  #[test]
  fn dot_with_point_multiplies_elements() {
    use crate::math::tuple::Tuple;
    use crate::math::Vector;
    use crate::math::Point;

    let vector = Vector::new(1.2, 1.3, 1.4);
    let point = Point::new(20.0, 30.0, 40.0);

    let dot_product = vector.dot(&point);

    assert_eq!(dot_product, 119.0);
  }
}
