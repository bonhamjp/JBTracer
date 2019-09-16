#[cfg(test)]
mod tests {
  use crate::math::Matrix2x2;

  #[test]
  fn new_sets_values() {
    let matrix = Matrix2x2::new(1.0, 2.0, 3.0, 4.0);

    assert_eq!(matrix.elements[0], 1.0);
    assert_eq!(matrix.elements[1], 2.0);
    assert_eq!(matrix.elements[2], 3.0);
    assert_eq!(matrix.elements[3], 4.0);
  }

  #[test]
  fn identity_sets_identity_matrix() {
    let matrix = Matrix2x2::identity();

    assert_eq!(matrix.elements[0], 1.0);
    assert_eq!(matrix.elements[1], 0.0);
    assert_eq!(matrix.elements[2], 0.0);
    assert_eq!(matrix.elements[3], 1.0);
  }

  #[test]
  fn returns_element_at_row_column() {
    let matrix = Matrix2x2::new(1.0, 2.0, 3.0, 4.0);

    assert_eq!(matrix.element(0, 0), 1.0);
    assert_eq!(matrix.element(0, 1), 2.0);
    assert_eq!(matrix.element(1, 0), 3.0);
    assert_eq!(matrix.element(1, 1), 4.0);
  }

  #[test]
  fn returns_contents_of_row() {
    let matrix = Matrix2x2::new(1.0, 2.0, 3.0, 4.0);

    assert_eq!(matrix.row(0), (1.0, 2.0));
    assert_eq!(matrix.row(1), (3.0, 4.0));
  }

  #[test]
  fn recognizes_equal_matrices() {
    let matrix_1 = Matrix2x2::new(1.0, 2.0, 3.0, 4.0);
    let matrix_2 = Matrix2x2::new(1.0, 2.0, 3.0, 4.0);

    assert_eq!(matrix_1.is_eq(&matrix_2), true);
  }

  #[test]
  fn recognizes_non_equal_matrices() {
    let matrix_1 = Matrix2x2::new(1.0, 2.0, 3.0, 4.0);
    let matrix_2 = Matrix2x2::new(2.0, 2.0, 3.0, 4.0);

    assert_eq!(matrix_1.is_eq(&matrix_2), false);
  }

  #[test]
  fn transpose_swaps_matrix_rows_with_columns() {
    let matrix = Matrix2x2::new(1.0, 2.0, 3.0, 4.0);
    
    let transposed_matrix = matrix.transpose();

    assert_eq!(transposed_matrix.row(0), (1.0, 3.0));
    assert_eq!(transposed_matrix.row(1), (2.0, 4.0));
  }

  #[test]
  fn calculates_determinant() {
    let matrix = Matrix2x2::new(1.0, 5.0, -3.0, 2.0);
    
    assert_eq!(matrix.determinant(), 17.0);
  }
}
