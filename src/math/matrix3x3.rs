use crate::math::matrix2x2::Matrix2x2;

pub struct Matrix3x3 {
  pub elements: [f64; 9]
}

impl Matrix3x3 {
  pub fn new(r1c1: f64, r1c2: f64, r1c3: f64, r2c1: f64, r2c2: f64, r2c3: f64, r3c1: f64, r3c2: f64, r3c3: f64) -> Matrix3x3 {
    let elements: [f64; 9] = [r1c1, r1c2, r1c3, r2c1, r2c2, r2c3, r3c1, r3c2, r3c3];
    
    Matrix3x3 { elements: elements }
  }

  pub fn identity() -> Matrix3x3 {
    let elements: [f64; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
    
    Matrix3x3 { elements: elements }
  }

  pub fn element(&self, row: usize, column: usize) -> f64 {
    self.elements[(row * 3) + column]
  }

  pub fn row(&self, row: usize) -> (f64, f64, f64) {
    if row == 0 {
      return (self.elements[0], self.elements[1], self.elements[2]);
    } else if row == 1 {
      return (self.elements[3], self.elements[4], self.elements[5]);
    } else {
      return (self.elements[6], self.elements[7], self.elements[8]);
    }
  }

  pub fn is_eq(&self, r_hand: &Matrix3x3) -> bool {
    self.elements[0] == r_hand.elements[0] &&
    self.elements[1] == r_hand.elements[1] &&
    self.elements[2] == r_hand.elements[2] &&
    self.elements[3] == r_hand.elements[3] &&
    self.elements[4] == r_hand.elements[4] &&
    self.elements[5] == r_hand.elements[5] &&
    self.elements[6] == r_hand.elements[6] &&
    self.elements[7] == r_hand.elements[7] &&
    self.elements[8] == r_hand.elements[8]
  }

  pub fn transpose(&self) -> Matrix3x3 {
    let r1c1 = self.element(0, 0);
    let r1c2 = self.element(1, 0);
    let r1c3 = self.element(2, 0);
    let r2c1 = self.element(0, 1);
    let r2c2 = self.element(1, 1);
    let r2c3 = self.element(2, 1);
    let r3c1 = self.element(0, 2);
    let r3c2 = self.element(1, 2);
    let r3c3 = self.element(2, 2);

    let elements: [f64; 9] = [r1c1, r1c2, r1c3, r2c1, r2c2, r2c3, r3c1, r3c2, r3c3];
    
    Matrix3x3 { elements: elements }
  }

  pub fn submatrix(&self, row: usize, column: usize) -> Matrix2x2 {
    let mut submatrix_elements: [f64; 4] = [0.0, 0.0, 0.0, 0.0];

    let mut current_element_index = 0;

    for i in 0..3 {
      for j in 0..3 {
        // gather elements from everywhere but row and column from arguments
        if (i != row && j != column) {
          submatrix_elements[current_element_index] = self.element(i, j);

          current_element_index += 1;
        }
      }
    }

    Matrix2x2 { elements: submatrix_elements }
  }

  pub fn minor(&self, row: usize, column: usize) -> f64 {
    self.submatrix(row, column).determinant()
  }

  pub fn cofactor(&self, row: usize, column: usize) -> f64 {
    if (row + column) % 2 == 0 {
      return self.minor(row, column);
    } else {
      return self.minor(row, column) * -1.0;
    }
  }

  pub fn determinant(&self) -> f64 {
    // first row multiplied by cofactors, then summed
    self.elements[0] * self.cofactor(0, 0) + self.elements[1] * self.cofactor(0, 1) + self.elements[2] * self.cofactor(0, 2)
  }
}
