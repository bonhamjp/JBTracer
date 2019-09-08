#[cfg(test)]
mod tests {
  #[test]
  fn new_sets_values() {
    use crate::math::Matrix4x4;

    let matrix = Matrix4x4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);

    assert_eq!(matrix.elements[0], 1.0);
    assert_eq!(matrix.elements[1], 2.0);
    assert_eq!(matrix.elements[2], 3.0);
    assert_eq!(matrix.elements[3], 4.0);
    assert_eq!(matrix.elements[4], 5.0);
    assert_eq!(matrix.elements[5], 6.0);
    assert_eq!(matrix.elements[6], 7.0);
    assert_eq!(matrix.elements[7], 8.0);
    assert_eq!(matrix.elements[8], 9.0);
    assert_eq!(matrix.elements[9], 10.0);
    assert_eq!(matrix.elements[10], 11.0);
    assert_eq!(matrix.elements[11], 12.0);
    assert_eq!(matrix.elements[12], 13.0);
    assert_eq!(matrix.elements[13], 14.0);
    assert_eq!(matrix.elements[14], 15.0);
    assert_eq!(matrix.elements[15], 16.0);
  }

  #[test]
  fn identity_sets_identity_matrix() {
    use crate::math::Matrix4x4;

    let matrix = Matrix4x4::identity();

    assert_eq!(matrix.elements[0], 1.0);
    assert_eq!(matrix.elements[1], 0.0);
    assert_eq!(matrix.elements[2], 0.0);
    assert_eq!(matrix.elements[3], 0.0);
    assert_eq!(matrix.elements[4], 0.0);
    assert_eq!(matrix.elements[5], 1.0);
    assert_eq!(matrix.elements[6], 0.0);
    assert_eq!(matrix.elements[7], 0.0);
    assert_eq!(matrix.elements[8], 0.0);
    assert_eq!(matrix.elements[9], 0.0);
    assert_eq!(matrix.elements[10], 1.0);
    assert_eq!(matrix.elements[11], 0.0);
    assert_eq!(matrix.elements[12], 0.0);
    assert_eq!(matrix.elements[13], 0.0);
    assert_eq!(matrix.elements[14], 0.0);
    assert_eq!(matrix.elements[15], 1.0);
  }

  #[test]
  fn returns_element_at_row_column() {
    use crate::math::Matrix4x4;

    let matrix = Matrix4x4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);

    assert_eq!(matrix.element(0, 0), 1.0);
    assert_eq!(matrix.element(0, 1), 2.0);
    assert_eq!(matrix.element(0, 2), 3.0);
    assert_eq!(matrix.element(0, 3), 4.0);
    assert_eq!(matrix.element(1, 0), 5.0);
    assert_eq!(matrix.element(1, 1), 6.0);
    assert_eq!(matrix.element(1, 2), 7.0);
    assert_eq!(matrix.element(1, 3), 8.0);
    assert_eq!(matrix.element(2, 0), 9.0);
    assert_eq!(matrix.element(2, 1), 10.0);
    assert_eq!(matrix.element(2, 2), 11.0);
    assert_eq!(matrix.element(2, 3), 12.0);
    assert_eq!(matrix.element(3, 0), 13.0);
    assert_eq!(matrix.element(3, 1), 14.0);
    assert_eq!(matrix.element(3, 2), 15.0);
    assert_eq!(matrix.element(3, 3), 16.0);
  }

  #[test]
  fn returns_contents_of_row() {
    use crate::math::Matrix4x4;

    let matrix = Matrix4x4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);

    assert_eq!(matrix.row(0), (1.0, 2.0, 3.0, 4.0));
    assert_eq!(matrix.row(1), (5.0, 6.0, 7.0, 8.0));
    assert_eq!(matrix.row(2), (9.0, 10.0, 11.0, 12.0));
    assert_eq!(matrix.row(3), (13.0, 14.0, 15.0, 16.0));
  }

  #[test]
  fn recognizes_equal_matrices() {
    use crate::math::Matrix4x4;

    let matrix_1 = Matrix4x4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
    let matrix_2 = Matrix4x4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);

    assert_eq!(matrix_1.is_eq(&matrix_2), true);
  }

  #[test]
  fn recognizes_non_equal_matrices() {
    use crate::math::Matrix4x4;

    let matrix_1 = Matrix4x4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
    let matrix_2 = Matrix4x4::new(2.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);

    assert_eq!(matrix_1.is_eq(&matrix_2), false);
  }

  #[test]
  fn multiplies_two_matrices_together() {
    use crate::math::Matrix4x4;

    let matrix_1 = Matrix4x4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0);
    let matrix_2 = Matrix4x4::new(-2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0);

    let result_matrix = matrix_1.mult4x4(&matrix_2);

    assert_eq!(result_matrix.row(0), (20.0, 22.0, 50.0, 48.0));
    assert_eq!(result_matrix.row(1), (44.0, 54.0, 114.0, 108.0));
    assert_eq!(result_matrix.row(2), (40.0, 58.0, 110.0, 102.0));
    assert_eq!(result_matrix.row(3), (16.0, 26.0, 46.0, 42.0));    
  }

  #[test]
  fn matrix_multiplied_with_identity_matrix_equals_original_matrix() {
    use crate::math::Matrix4x4;

    let matrix_1 = Matrix4x4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
    let matrix_identity = Matrix4x4::identity();

    let result_matrix = matrix_1.mult4x4(&matrix_identity);

    assert_eq!(result_matrix.is_eq(&matrix_1), true);  
  }

  #[test]
  fn multiplies_matrix_with_point() {
    use crate::math::Matrix4x4;
    use crate::math::tuple::Tuple;
    use crate::math::Point;

    let matrix_1 = Matrix4x4::new(1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0);
    let point = Point::new(1.0, 2.0, 3.0);

    let (result_x, result_y, result_z, result_w) = matrix_1.mult4x1(&point);

    assert_eq!(result_x, 18.0);
    assert_eq!(result_y, 24.0);
    assert_eq!(result_z, 33.0);
    assert_eq!(result_w, 1.0);  
  }

  #[test]
  fn transpose_swaps_matrix_rows_with_columns() {
    use crate::math::Matrix4x4;

    let matrix = Matrix4x4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
    
    let transposed_matrix = matrix.transpose();

    assert_eq!(transposed_matrix.row(0), (1.0, 5.0, 9.0, 13.0));
    assert_eq!(transposed_matrix.row(1), (2.0, 6.0, 10.0, 14.0));
    assert_eq!(transposed_matrix.row(2), (3.0, 7.0, 11.0, 15.0));
    assert_eq!(transposed_matrix.row(3), (4.0, 8.0, 12.0, 16.0));
  }

  #[test]
  fn submatrix_returns_3x3_matrix_with_all_elements_not_from_row_and_column_args() {
    use crate::math::Matrix4x4;
    use crate::math::Matrix3x3;

    let matrix = Matrix4x4::new(-6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0);
    
    let submatrix = matrix.submatrix(2, 1);

    assert_eq!(submatrix.row(0), (-6.0, 1.0, 6.0));
    assert_eq!(submatrix.row(1), (-8.0, 8.0, 6.0));
    assert_eq!(submatrix.row(2), (-7.0, -1.0, 1.0));
  }

  #[test]
  fn minor_returns_the_determinant_of_the_submatrix_for_a_row_column() {
    use crate::math::Matrix4x4;
    
    let matrix = Matrix4x4::new(-2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0);

    let submatrix = matrix.submatrix(1, 0);

    assert_eq!(submatrix.determinant(), 253.0);
    assert_eq!(matrix.minor(1, 0), 253.0);
  }

  #[test]
  fn cofactor_returns_the_determinant_of_the_submatrix_for_a_row_column_negated_if_odd_summed() {
    use crate::math::Matrix4x4;
    
    let matrix = Matrix4x4::new(-2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0);

    assert_eq!(matrix.minor(0, 0), 690.0);
    assert_eq!(matrix.cofactor(0, 0), 690.0);
    assert_eq!(matrix.minor(1, 0), 253.0);
    assert_eq!(matrix.cofactor(1, 0), -253.0);
  }  

  #[test]
  fn determinant_is_calculated_using_cofactor_of_each_column_of_row_one_summed() {
    use crate::math::Matrix4x4;
    
    let matrix = Matrix4x4::new(-2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0);

    assert_eq!(matrix.cofactor(0, 0), 690.0);
    assert_eq!(matrix.cofactor(0, 1), 447.0);
    assert_eq!(matrix.cofactor(0, 2), 210.0);
    assert_eq!(matrix.cofactor(0, 3), 51.0);
    assert_eq!(matrix.determinant(), -4071.0);
  }

  #[test]
  fn checks_if_matrix_is_invertible_if_determinant_is_not_zero() {
    use crate::math::Matrix4x4;
    
    let matrix = Matrix4x4::new(6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0);

    assert_eq!(matrix.determinant(), -2120.0);
    assert_eq!(matrix.invertible(), true);
  }

  #[test]
  fn checks_if_matrix_is_not_invertible_if_determinant_is_zero() {
    use crate::math::Matrix4x4;

    let matrix = Matrix4x4::new(-4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 3.0, -7.0, 0.0, 0.0, 0.0, 0.0);

    assert_eq!(matrix.determinant(), 0.0);
    assert_eq!(matrix.invertible(), false);
  }

  #[test]
  fn inverse_creates_inversion_of_matrix() {
    use crate::math::Matrix4x4;

    let matrix_1 = Matrix4x4::new(-5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0);

    let inverted_1 = matrix_1.inverse();

    assert_eq!(matrix_1.determinant(), 532.0);
    assert_eq!(matrix_1.cofactor(2, 3), -160.0);
    assert_eq!(inverted_1.element(3, 2), -160.0 / 532.0);

    assert_eq!(inverted_1.row(0), (0.21804511278195488, 0.45112781954887216, 0.24060150375939848, -0.045112781954887216));
    assert_eq!(inverted_1.row(1), (-0.8082706766917294, -1.4567669172932332, -0.44360902255639095, 0.5206766917293233));
    assert_eq!(inverted_1.row(2), (-0.07894736842105263, -0.2236842105263158, -0.05263157894736842, 0.19736842105263158));
    assert_eq!(inverted_1.row(3), (-0.5225563909774437, -0.8139097744360902, -0.3007518796992481, 0.30639097744360905)); 
  
    let matrix_2 = Matrix4x4::new(8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0);

    let inverted_2 = matrix_2.inverse();

    assert_eq!(inverted_2.row(0), (-0.15384615384615385, -0.15384615384615385, -0.28205128205128205, -0.5384615384615384));
    assert_eq!(inverted_2.row(1), (-0.07692307692307693, 0.12307692307692308, 0.02564102564102564, 0.03076923076923077));
    assert_eq!(inverted_2.row(2), (0.358974358974359, 0.358974358974359, 0.4358974358974359, 0.9230769230769231));
    assert_eq!(inverted_2.row(3), (-0.6923076923076923, -0.6923076923076923, -0.7692307692307693, -1.9230769230769231));

    let matrix_3 = Matrix4x4::new(9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0, 2.0);

    let inverted_3 = matrix_3.inverse();

    assert_eq!(inverted_3.row(0), (-0.040740740740740744, -0.07777777777777778, 0.14444444444444443, -0.2222222222222222));
    assert_eq!(inverted_3.row(1), (-0.07777777777777778, 0.03333333333333333, 0.36666666666666664, -0.3333333333333333));
    assert_eq!(inverted_3.row(2), (-0.029012345679012345, -0.14629629629629629, -0.10925925925925926, 0.12962962962962962));
    assert_eq!(inverted_3.row(3), (0.17777777777777778, 0.06666666666666667, -0.26666666666666666, 0.3333333333333333));
  }

  #[test]
  fn matrix_multiplied_by_another_matrix_then_inversion_of_the_same_matrix_results_in_original_matrix() {
    use crate::math::Matrix4x4;

    let matrix_1 = Matrix4x4::new(3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0);
    let matrix_2 = Matrix4x4::new(8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0);

    let multiplied_matrix = matrix_1.mult4x4(&matrix_2);
    let inverse_multiplied_matrix = multiplied_matrix.mult4x4(&matrix_2.inverse());

    assert_eq!(matrix_1.is_eq(&inverse_multiplied_matrix), true);
  }

  #[test]
  fn matrix_translation_sets_up_identity_matrix_with_translations_in_last_row() {
    use crate::math::Matrix4x4;
    
    let translation = Matrix4x4::translate(10.0, 20.0, 30.0);

    assert_eq!(translation.row(0), (1.0, 0.0, 0.0, 10.0));
    assert_eq!(translation.row(1), (0.0, 1.0, 0.0, 20.0));
    assert_eq!(translation.row(2), (0.0, 0.0, 1.0, 30.0));
    assert_eq!(translation.row(3), (0.0, 0.0, 0.0, 1.0));
  }

  #[test]
  fn point_multipled_by_translation_matrix_moves_point_by_translation() {
    use crate::math::Matrix4x4;
    use crate::math::Point;

    let translation = Matrix4x4::translate(5.0, -3.0, 2.0);

    let point = Point::new(-3.0, 4.0, 5.0);

    let (translated_x, translated_y, translated_z, _) = translation.mult4x1(&point);

    assert_eq!(translated_x, 2.0);
    assert_eq!(translated_y, 1.0);
    assert_eq!(translated_z, 7.0);
  }

  #[test]
  fn point_multiplied_by_inverse_of_translation_matrix_moves_in_reverse() {
    use crate::math::Matrix4x4;
    use crate::math::Point;

    let translation = Matrix4x4::translate(5.0, -3.0, 2.0);

    let inverted = translation.inverse();

    let point = Point::new(-3.0, 4.0, 5.0);

    let (translated_x, translated_y, translated_z, _) = inverted.mult4x1(&point);

    assert_eq!(translated_x, -8.0);
    assert_eq!(translated_y, 7.0);
    assert_eq!(translated_z, 3.0);
  }

  #[test]
  fn vector_multipled_by_translation_matrix_does_not_alter_vector() {
    use crate::math::Matrix4x4;
    use crate::math::Vector;
    
    let translation = Matrix4x4::translate(5.0, -3.0, 2.0);

    let vector = Vector::new(-3.0, 4.0, 2.0);

    let (translated_x, translated_y, translated_z, _) = translation.mult4x1(&vector);

    assert_eq!(translated_x, -3.0);
    assert_eq!(translated_y, 4.0);
    assert_eq!(translated_z, 2.0);
  }

  #[test]
  fn matrix_scale_sets_up_identity_matrix_with_scale_in_diagonal() {
    use crate::math::Matrix4x4;
    
    let scale = Matrix4x4::scale(4.0, 5.0, 6.0);

    assert_eq!(scale.row(0), (4.0, 0.0, 0.0, 0.0));
    assert_eq!(scale.row(1), (0.0, 5.0, 0.0, 0.0));
    assert_eq!(scale.row(2), (0.0, 0.0, 6.0, 0.0));
    assert_eq!(scale.row(3), (0.0, 0.0, 0.0, 1.0));
  }

  #[test]
  fn point_multipled_by_scale_matrix_scales_point_by_factors() {
    use crate::math::Matrix4x4;
    use crate::math::Point;

    let scale = Matrix4x4::scale(2.0, 3.0, 4.0);

    let point = Point::new(-4.0, 6.0, 8.0);

    let (scaled_x, scaled_y, scaled_z, _) = scale.mult4x1(&point);

    assert_eq!(scaled_x, -8.0);
    assert_eq!(scaled_y, 18.0);
    assert_eq!(scaled_z, 32.0);
  }

  #[test]
  fn point_multiplied_by_inverse_of_scaling_matrix_scales_in_opposite_direction() {
    use crate::math::Matrix4x4;
    use crate::math::Point;

    let scale = Matrix4x4::scale(2.0, 3.0, 4.0);

    let inverted = scale.inverse();

    let point = Point::new(-4.0, 6.0, 8.0);

    let (scaled_x, scaled_y, scaled_z, _) = inverted.mult4x1(&point);

    assert_eq!(scaled_x, -2.0);
    assert_eq!(scaled_y, 2.0);
    assert_eq!(scaled_z, 2.0);
  }

  #[test]
  fn vector_multipled_by_scale_matrix_scales_vector_by_factors() {
    use crate::math::Matrix4x4;
    use crate::math::Vector;
    
    let scale = Matrix4x4::scale(2.0, 3.0, 4.0);

    let vector = Vector::new(-4.0, 6.0, 8.0);

    let (scaled_x, scaled_y, scaled_z, _) = scale.mult4x1(&vector);

    assert_eq!(scaled_x, -8.0);
    assert_eq!(scaled_y, 18.0);
    assert_eq!(scaled_z, 32.0);
  }

  #[test]
  fn reflection_is_scaling_by_negative_value() {
    use crate::math::Matrix4x4;
    use crate::math::Point;

    let reflection = Matrix4x4::scale(-1.0, 1.0, 1.0);

    let point = Point::new(2.0, 3.0, 4.0);

    let (reflected_x, reflected_y, reflected_z, _) = reflection.mult4x1(&point);

    assert_eq!(reflected_x, -2.0);
    assert_eq!(reflected_y, 3.0);
    assert_eq!(reflected_z, 4.0);
  }

  #[test]
  fn rotating_point_around_x_axis() {
    use std::f64;

    use crate::math::Matrix4x4;
    use crate::math::Point;

    let point = Point::new(0.0, 1.0, 0.0);
    
    let x_half_quarter_rotation = Matrix4x4::rotate_x(f64::consts::PI / 4.0);

    let (half_quarter_rotated_x, half_quarter_rotated_y, half_quarter_rotated_z, _) = x_half_quarter_rotation.mult4x1(&point);

    assert_eq!(half_quarter_rotated_x, 0.0);
    assert_eq!(half_quarter_rotated_y, ((2.0 as f64).sqrt()) / 2.0);
    assert_eq!(half_quarter_rotated_z, ((2.0 as f64).sqrt()) / 2.0);

    let x_full_quarter_rotation = Matrix4x4::rotate_x(f64::consts::PI / 2.0);

    let (full_quarter_rotated_x, full_quarter_rotated_y, full_quarter_rotated_z, _) = x_full_quarter_rotation.mult4x1(&point);

    // TODO: use global delta
    assert!((full_quarter_rotated_x - 0.0).abs() < 0.0001);
    assert!((full_quarter_rotated_y - 0.0).abs() < 0.0001);
    assert!((full_quarter_rotated_z - 1.0).abs() < 0.0001);
  }

  #[test]
  fn rotating_point_around_inverse_x_axis_rotates_in_opposite_direction() {
    use std::f64;

    use crate::math::Matrix4x4;
    use crate::math::Point;

    let point = Point::new(0.0, 1.0, 0.0);
    
    let x_half_quarter_rotation = Matrix4x4::rotate_x(f64::consts::PI / 4.0);

    let inverse = x_half_quarter_rotation.inverse();

    let (inverse_half_quarter_rotated_x, inverse_half_quarter_rotated_y, inverse_half_quarter_rotated_z, _) = inverse.mult4x1(&point);

    // TODO: use global delta
    assert!((inverse_half_quarter_rotated_x - 0.0).abs() < 0.0001);
    assert!((inverse_half_quarter_rotated_y - ((2.0 as f64).sqrt()) / 2.0).abs() < 0.0001);
    assert!((inverse_half_quarter_rotated_z - (-(2.0 as f64).sqrt()) / 2.0).abs() < 0.0001);
  }

  #[test]
  fn rotating_point_around_y_axis() {
    use std::f64;

    use crate::math::Matrix4x4;
    use crate::math::Point;

    let point = Point::new(0.0, 0.0, 1.0);
    
    let y_half_quarter_rotation = Matrix4x4::rotate_y(f64::consts::PI / 4.0);

    let (half_quarter_rotated_x, half_quarter_rotated_y, half_quarter_rotated_z, _) = y_half_quarter_rotation.mult4x1(&point);

    assert_eq!(half_quarter_rotated_x, ((2.0 as f64).sqrt()) / 2.0);
    assert_eq!(half_quarter_rotated_y, 0.0);
    assert_eq!(half_quarter_rotated_z, ((2.0 as f64).sqrt()) / 2.0);

    let y_full_quarter_rotation = Matrix4x4::rotate_y(f64::consts::PI / 2.0);

    let (full_quarter_rotated_x, full_quarter_rotated_y, full_quarter_rotated_z, _) = y_full_quarter_rotation.mult4x1(&point);

    // TODO: use global delta
    assert!((full_quarter_rotated_x - 1.0).abs() < 0.0001);
    assert!((full_quarter_rotated_y - 0.0).abs() < 0.0001);
    assert!((full_quarter_rotated_z - 0.0).abs() < 0.0001);
  }

  #[test]
  fn rotating_point_around_z_axis() {
    use std::f64;

    use crate::math::Matrix4x4;
    use crate::math::Point;

    let point = Point::new(0.0, 1.0, 0.0);
    
    let z_half_quarter_rotation = Matrix4x4::rotate_z(f64::consts::PI / 4.0);

    let (half_quarter_rotated_x, half_quarter_rotated_y, half_quarter_rotated_z, _) = z_half_quarter_rotation.mult4x1(&point);

    assert_eq!(half_quarter_rotated_x, (-(2.0 as f64).sqrt()) / 2.0);
    assert_eq!(half_quarter_rotated_y, ((2.0 as f64).sqrt()) / 2.0);
    assert_eq!(half_quarter_rotated_z, 0.0);

    let z_full_quarter_rotation = Matrix4x4::rotate_z(f64::consts::PI / 2.0);

    let (full_quarter_rotated_x, full_quarter_rotated_y, full_quarter_rotated_z, _) = z_full_quarter_rotation.mult4x1(&point);

    // TODO: use global delta
    assert!((full_quarter_rotated_x - -1.0).abs() < 0.0001);
    assert!((full_quarter_rotated_y - 0.0).abs() < 0.0001);
    assert!((full_quarter_rotated_z - 0.0).abs() < 0.0001);
  }

  #[test]
  fn shearing_x_in_proportion_to_y() {
    use crate::math::Matrix4x4;
    use crate::math::Point;
    
    let shearing = Matrix4x4::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    let point = Point::new(2.0, 3.0, 4.0);

    let (sheared_x, sheared_y, sheared_z, _) = shearing.mult4x1(&point);

    assert_eq!(sheared_x, 5.0);
    assert_eq!(sheared_y, 3.0);
    assert_eq!(sheared_z, 4.0);
  }

  #[test]
  fn shearing_x_in_proportion_to_z() {
    use crate::math::Matrix4x4;
    use crate::math::Point;
    
    let shearing = Matrix4x4::shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);

    let point = Point::new(2.0, 3.0, 4.0);

    let (sheared_x, sheared_y, sheared_z, _) = shearing.mult4x1(&point);

    assert_eq!(sheared_x, 6.0);
    assert_eq!(sheared_y, 3.0);
    assert_eq!(sheared_z, 4.0);
  }

  #[test]
  fn shearing_y_in_proportion_to_x() {
    use crate::math::Matrix4x4;
    use crate::math::Point;
    
    let shearing = Matrix4x4::shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

    let point = Point::new(2.0, 3.0, 4.0);

    let (sheared_x, sheared_y, sheared_z, _) = shearing.mult4x1(&point);

    assert_eq!(sheared_x, 2.0);
    assert_eq!(sheared_y, 5.0);
    assert_eq!(sheared_z, 4.0);
  }

  #[test]
  fn shearing_y_in_proportion_to_z() {
    use crate::math::Matrix4x4;
    use crate::math::Point;
    
    let shearing = Matrix4x4::shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

    let point = Point::new(2.0, 3.0, 4.0);

    let (sheared_x, sheared_y, sheared_z, _) = shearing.mult4x1(&point);

    assert_eq!(sheared_x, 2.0);
    assert_eq!(sheared_y, 7.0);
    assert_eq!(sheared_z, 4.0);
  }

  #[test]
  fn shearing_z_in_proportion_to_x() {
    use crate::math::Matrix4x4;
    use crate::math::Point;
    
    let shearing = Matrix4x4::shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);

    let point = Point::new(2.0, 3.0, 4.0);

    let (sheared_x, sheared_y, sheared_z, _) = shearing.mult4x1(&point);

    assert_eq!(sheared_x, 2.0);
    assert_eq!(sheared_y, 3.0);
    assert_eq!(sheared_z, 6.0);
  }

  #[test]
  fn shearing_z_in_proportion_to_y() {
    use crate::math::Matrix4x4;
    use crate::math::Point;
    
    let shearing = Matrix4x4::shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

    let point = Point::new(2.0, 3.0, 4.0);

    let (sheared_x, sheared_y, sheared_z, _) = shearing.mult4x1(&point);

    assert_eq!(sheared_x, 2.0);
    assert_eq!(sheared_y, 3.0);
    assert_eq!(sheared_z, 7.0);
  }

  #[test]
  fn transformation_applied_in_sequence() {
    use std::f64;

    use crate::math::Matrix4x4;
    use crate::math::Point;
    
    let x_rotation = Matrix4x4::rotate_x(f64::consts::PI / 2.0);
    let scaling = Matrix4x4::scale(5.0, 5.0, 5.0);
    let translation = Matrix4x4::translate(10.0, 5.0, 7.0);

    let point_1 = Point::new(1.0, 0.0, 1.0);

    let (rotated_x, rotated_y, rotated_z, _) = x_rotation.mult4x1(&point_1);

    // TODO: use global delta
    assert!((rotated_x - 1.0).abs() < 0.0001);
    assert!((rotated_y - -1.0).abs() < 0.0001);
    assert!((rotated_z - 0.0).abs() < 0.0001);

    let point_2 = Point::new(1.0, -1.0, 0.0);

    let (scaled_x, scaled_y, scaled_z, _) = scaling.mult4x1(&point_2);

    // TODO: use global delta
    assert!((scaled_x - 5.0).abs() < 0.0001);
    assert!((scaled_y - -5.0).abs() < 0.0001);
    assert!((scaled_z - 0.0).abs() < 0.0001);

    let point_3 = Point::new(5.0, -5.0, 0.0);

    let (translated_x, translated_y, translated_z, _) = translation.mult4x1(&point_3);

    // TODO: use global delta
    assert!((translated_x - 15.0).abs() < 0.0001);
    assert!((translated_y - 0.0).abs() < 0.0001);
    assert!((translated_z - 7.0).abs() < 0.0001);
  }

  #[test]
  fn chaining_transformation_together() {
    // must be done in reversed order
    
    use std::f64;

    use crate::math::Matrix4x4;
    use crate::math::Point;
    
    let x_rotation = Matrix4x4::rotate_x(f64::consts::PI / 2.0);
    let scaling = Matrix4x4::scale(5.0, 5.0, 5.0);
    let translation = Matrix4x4::translate(10.0, 5.0, 7.0);

    let chained_transformation = translation.mult4x4(&scaling).mult4x4(&x_rotation);

    let point = Point::new(1.0, 0.0, 1.0);

    let (transformed_x, transformed_y, transformed_z, _) = chained_transformation.mult4x1(&point);

    // TODO: use global delta
    assert!((transformed_x - 15.0).abs() < 0.0001);
    assert!((transformed_y - 0.0).abs() < 0.0001);
    assert!((transformed_z - 7.0).abs() < 0.0001);
  }
}
