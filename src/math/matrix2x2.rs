pub struct Matrix2x2 {
  pub elements: [f64; 4]
}

impl Matrix2x2 {
  pub fn new(r1c1: f64, r1c2: f64, r2c1: f64, r2c2: f64) -> Matrix2x2 {
    let elements: [f64; 4] = [r1c1, r1c2, r2c1, r2c2];
    
    Matrix2x2 { elements: elements }
  }

  pub fn identity() -> Matrix2x2 {
    let elements: [f64; 4] = [1.0, 0.0, 0.0, 1.0];
    
    Matrix2x2 { elements: elements }
  }

  pub fn element(&self, row: usize, column: usize) -> f64 {
    self.elements[(row * 2) + column]
  }

  pub fn row(&self, row: usize) -> (f64, f64) {
    if row == 0 {
      return (self.elements[0], self.elements[1]);
    } else {
      return (self.elements[2], self.elements[3]);
    }
  }

  pub fn is_eq(&self, r_hand: &Matrix2x2) -> bool {
    self.elements[0] == r_hand.elements[0] &&
    self.elements[1] == r_hand.elements[1] &&
    self.elements[2] == r_hand.elements[2] &&
    self.elements[3] == r_hand.elements[3]
  }

  pub fn transpose(&self) -> Matrix2x2 {
    let r1c1 = self.element(0, 0);
    let r1c2 = self.element(1, 0);
    let r2c1 = self.element(0, 1);
    let r2c2 = self.element(1, 1);

    let elements: [f64; 4] = [r1c1, r1c2, r2c1, r2c2];
    
    Matrix2x2 { elements: elements }
  }

  pub fn determinant(&self) -> f64 {
    self.element(0, 0) * self.element(1, 1) - self.element(0, 1) * self.element(1, 0)
  }
}
