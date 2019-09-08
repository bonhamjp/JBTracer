#[cfg(test)]
mod tests {
  #[test]
  fn new_sets_values() {
    use crate::math::Point;

    let point = Point::new(10.10, 20.20, 30.30);

    assert_eq!(point.x, 10.10);
    assert_eq!(point.y, 20.20);
    assert_eq!(point.z, 30.30);
  }

  #[test]
  fn empty_sets_0_values() {
    use crate::math::Point;

    let point = Point::empty();

    assert_eq!(point.x, 0.0);
    assert_eq!(point.y, 0.0);
    assert_eq!(point.z, 0.0);
  }

  #[test]
  fn w_returns_1() {
    use crate::math::Point;

    let point_1 = Point::new(4.2, 2.4, 6.6);

    assert_eq!(point_1.w(), 1.0);

    let point_2 = Point::empty();

    assert_eq!(point_2.w(), 1.0);
  }

  #[test]
  fn get_quad_returns_a_4_tuple_with_elements_and_w() {
    use crate::math::tuple::Tuple;
    use crate::math::Point;

    let point_1 = Point::new(1.9, 1.8, 1.7);

    assert_eq!(point_1.get_quad(), (1.9, 1.8, 1.7, 1.0));

    let point_2 = Point::empty();

    assert_eq!(point_2.get_quad(), (0.0, 0.0, 0.0, 1.0));
  }

  #[test]
  fn adding_a_vector_adds_each_component_returning_point() {
    use crate::math::tuple::Tuple;
    use crate::math::Point;
    use crate::math::Vector;

    let point = Point::new(5.1, 4.1, 3.1);
    let vector = Vector::new(1.0, 2.0, 3.0);

    let sum = point.add_vector(&vector);

    assert_eq!(sum.get_quad(), (6.1, 6.1, 6.1, 1.0));
  }

  #[test]
  fn subtracting_a_point_subtracts_each_component_returning_vector() {
    use crate::math::tuple::Tuple;
    use crate::math::Point;

    let point_1 = Point::new(20.1, 20.2, 20.3);
    let point_2 = Point::new(2.0, 2.0, 2.0);

    let difference = point_1.subtract_point(&point_2);

    assert_eq!(difference.get_quad(), (18.1, 18.2, 18.3, 0.0));
  }

    #[test]
  fn subtracting_a_vector_subtracts_each_component_returning_point() {
    use crate::math::tuple::Tuple;
    use crate::math::Point;
    use crate::math::Vector;
    
    let point = Vector::new(10.1, 20.2, 30.3);
    let vector = Vector::new(1.1, 1.2, 1.3);

    let difference = point.subtract_vector(&vector);

    assert_eq!(difference.get_quad(), (9.0, 19.0, 29.0, 0.0));
  }

  #[test]
  fn multiplying_by_number_multiplies_each_component() {
    use crate::math::tuple::Tuple;
    use crate::math::Point;

    let point = Point::new(1.2, 2.2, 3.2);

    let product = point.multiply(10.0);

    assert_eq!(product.get_quad(), (12.0, 22.0, 32.0, 1.0));
  }

  #[test]
  fn dividing_by_number_divides_each_component() {
    use crate::math::tuple::Tuple;
    use crate::math::Point;

    let point = Point::new(10.0, 20.0, 30.0);

    let quotient = point.divide(10.0);

    assert_eq!(quotient.get_quad(), (1.0, 2.0, 3.0, 1.0));
  }

  #[test]
  fn normalize_divides_by_magnitude() {
    use crate::math::tuple::Tuple;
    use crate::math::Point;

    let point = Point::new(1.0, 2.0, 3.0);

    let normalized = point.normalize();

    assert_eq!(normalized.get_quad(), (0.2581988897471611, 0.5163977794943222, 0.7745966692414834, 1.0));
  }

  #[test]
  fn dot_with_point_vector_multiplies_elements() {
    use crate::math::tuple::Tuple;
    use crate::math::Point;

    let point_1 = Point::new(1.2, 1.3, 1.4);
    let point_2 = Point::new(2.1, 3.1, 4.1);

    let dot_product = point_1.dot(&point_2);

    assert_eq!(dot_product, 13.29);
  }

  #[test]
  fn magnitude_uses_pythag_theorum() {
    use crate::math::tuple::Tuple;
    use crate::math::Point;

    let point = Point::new(5.0, 4.0, 3.0);

    let magnitude = point.magnitude();

    assert_eq!(magnitude, 7.14142842854285);
  }

  #[test]
  fn dot_with_point_multiplies_elements() {
    use crate::math::tuple::Tuple;
    use crate::math::Point;
    use crate::math::Vector;

    let point = Point::new(1.2, 1.3, 1.4);
    let vector = Vector::new(20.0, 30.0, 40.0);

    let dot_product = point.dot(&vector);

    assert_eq!(dot_product, 119.0);
  }
}
