#[cfg(test)]
mod tests {
  use crate::rendering::math::Matrix3x3;

  #[test]
  fn new_sets_values() {
    let matrix = Matrix3x3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);

    assert_eq!(matrix.elements[0], 1.0);
    assert_eq!(matrix.elements[1], 2.0);
    assert_eq!(matrix.elements[2], 3.0);
    assert_eq!(matrix.elements[3], 4.0);
    assert_eq!(matrix.elements[4], 5.0);
    assert_eq!(matrix.elements[5], 6.0);
    assert_eq!(matrix.elements[6], 7.0);
    assert_eq!(matrix.elements[7], 8.0);
    assert_eq!(matrix.elements[8], 9.0);
  }

  #[test]
  fn identity_sets_identity_matrix() {
    let matrix = Matrix3x3::identity();

    assert_eq!(matrix.elements[0], 1.0);
    assert_eq!(matrix.elements[1], 0.0);
    assert_eq!(matrix.elements[2], 0.0);
    assert_eq!(matrix.elements[3], 0.0);
    assert_eq!(matrix.elements[4], 1.0);
    assert_eq!(matrix.elements[5], 0.0);
    assert_eq!(matrix.elements[6], 0.0);
    assert_eq!(matrix.elements[7], 0.0);
    assert_eq!(matrix.elements[8], 1.0);
  }

  #[test]
  fn returns_element_at_row_column() {
    let matrix = Matrix3x3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);

    assert_eq!(matrix.element(0, 0), 1.0);
    assert_eq!(matrix.element(0, 1), 2.0);
    assert_eq!(matrix.element(0, 2), 3.0);
    assert_eq!(matrix.element(1, 0), 4.0);
    assert_eq!(matrix.element(1, 1), 5.0);
    assert_eq!(matrix.element(1, 2), 6.0);
    assert_eq!(matrix.element(2, 0), 7.0);
    assert_eq!(matrix.element(2, 1), 8.0);
    assert_eq!(matrix.element(2, 2), 9.0);
  }

  #[test]
  fn returns_contents_of_row() {
    let matrix = Matrix3x3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);

    assert_eq!(matrix.row(0), (1.0, 2.0, 3.0));
    assert_eq!(matrix.row(1), (4.0, 5.0, 6.0));
    assert_eq!(matrix.row(2), (7.0, 8.0, 9.0));
  }

  #[test]
  fn recognizes_equal_matrices() {
    let matrix_1 = Matrix3x3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let matrix_2 = Matrix3x3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);

    assert_eq!(matrix_1.is_eq(&matrix_2), true);
  }

  #[test]
  fn recognizes_non_equal_matrices() {
    let matrix_1 = Matrix3x3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let matrix_2 = Matrix3x3::new(2.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);

    assert_eq!(matrix_1.is_eq(&matrix_2), false);
  }

  #[test]
  fn transpose_swaps_matrix_rows_with_columns() {
    let matrix = Matrix3x3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    
    let transposed_matrix = matrix.transpose();

    assert_eq!(transposed_matrix.row(0), (1.0, 4.0, 7.0));
    assert_eq!(transposed_matrix.row(1), (2.0, 5.0, 8.0));
    assert_eq!(transposed_matrix.row(3), (3.0, 6.0, 9.0));
  }

  #[test]
  fn submatrix_returns_2x2_matrix_with_all_elements_not_from_row_and_column_args() {
    let matrix = Matrix3x3::new(1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0);
    
    let submatrix = matrix.submatrix(0, 2);

    assert_eq!(submatrix.row(0), (-3.0, 2.0));
    assert_eq!(submatrix.row(1), (0.0, 6.0));
  }

  #[test]
  fn minor_returns_the_determinant_of_the_submatrix_for_a_row_column() {
    let matrix = Matrix3x3::new(3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0);
    
    let submatrix = matrix.submatrix(1, 0);

    assert_eq!(submatrix.determinant(), 25.0);
    assert_eq!(matrix.minor(1, 0), 25.0);
  }

  #[test]
  fn cofactor_returns_the_determinant_of_the_submatrix_for_a_row_column_negated_if_odd_summed() {
    let matrix = Matrix3x3::new(3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0);

    assert_eq!(matrix.minor(0, 0), -12.0);
    assert_eq!(matrix.cofactor(0, 0), -12.0);
    assert_eq!(matrix.minor(1, 0), 25.0);
    assert_eq!(matrix.cofactor(1, 0), -25.0);
  }

  #[test]
  fn determinant_is_calculated_using_cofactor_of_each_column_of_row_one_summed() {
    let matrix = Matrix3x3::new(1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0);

    assert_eq!(matrix.cofactor(0, 0), 56.0);
    assert_eq!(matrix.cofactor(0, 1), 12.0);
    assert_eq!(matrix.cofactor(0, 2), -46.0);
    assert_eq!(matrix.determinant(), -196.0);
  }
}
