use crate::math::matrix3x3::Matrix3x3;

use crate::math::tuple::Tuple;
use crate::math::Point;
use crate::math::Vector;

// TODO: Declare this delta value somewhere global
const ROUNDING_DELTA: f64 = 0.0001;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Matrix4x4 {
  pub elements: [f64; 16]
}

impl Matrix4x4 {
  pub fn new(r1c1: f64, r1c2: f64, r1c3: f64, r1c4: f64, r2c1: f64, r2c2: f64, r2c3: f64, r2c4: f64, r3c1: f64, r3c2: f64, r3c3: f64, r3c4: f64, r4c1: f64, r4c2: f64, r4c3: f64, r4c4: f64) -> Matrix4x4 {
    let elements: [f64; 16] = [r1c1, r1c2, r1c3, r1c4, r2c1, r2c2, r2c3, r2c4, r3c1, r3c2, r3c3, r3c4, r4c1, r4c2, r4c3, r4c4];
    
    Matrix4x4 { elements: elements }
  }

  pub fn identity() -> Matrix4x4 {
    let elements: [f64; 16] = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    
    Matrix4x4 { elements: elements }
  }

  pub fn element(&self, row: usize, column: usize) -> f64 {
    self.elements[(row * 4) + column]
  }

  pub fn row(&self, row: usize) -> (f64, f64, f64, f64) {
    if row == 0 {
      return (self.elements[0], self.elements[1], self.elements[2], self.elements[3]);
    } else if row == 1 {
      return (self.elements[4], self.elements[5], self.elements[6], self.elements[7]);
    } else if row == 2 {
      return (self.elements[8], self.elements[9], self.elements[10], self.elements[11]);
    } else {
      return (self.elements[12], self.elements[13], self.elements[14], self.elements[15]);
    }
  }

  pub fn is_eq(&self, r_hand: &Matrix4x4) -> bool {
    (self.elements[0] - r_hand.elements[0]).abs() < ROUNDING_DELTA &&
    (self.elements[1] - r_hand.elements[1]).abs() < ROUNDING_DELTA &&
    (self.elements[2] - r_hand.elements[2]).abs() < ROUNDING_DELTA &&
    (self.elements[3] - r_hand.elements[3]).abs() < ROUNDING_DELTA &&
    (self.elements[4] - r_hand.elements[4]).abs() < ROUNDING_DELTA &&
    (self.elements[5] - r_hand.elements[5]).abs() < ROUNDING_DELTA &&
    (self.elements[6] - r_hand.elements[6]).abs() < ROUNDING_DELTA &&
    (self.elements[7] - r_hand.elements[7]).abs() < ROUNDING_DELTA &&
    (self.elements[8] - r_hand.elements[8]).abs() < ROUNDING_DELTA &&
    (self.elements[9] - r_hand.elements[9]).abs() < ROUNDING_DELTA &&
    (self.elements[10] - r_hand.elements[10]).abs() < ROUNDING_DELTA &&
    (self.elements[11] - r_hand.elements[11]).abs() < ROUNDING_DELTA &&
    (self.elements[12] - r_hand.elements[12]).abs() < ROUNDING_DELTA &&
    (self.elements[13] - r_hand.elements[13]).abs() < ROUNDING_DELTA &&
    (self.elements[14] - r_hand.elements[14]).abs() < ROUNDING_DELTA &&
    (self.elements[15] - r_hand.elements[15]).abs() < ROUNDING_DELTA
  }

  pub fn mult4x4(&self, r_hand: &Matrix4x4) -> Matrix4x4 {
    let r1c1 = self.element(0, 0) * r_hand.element(0, 0) +
               self.element(0, 1) * r_hand.element(1, 0) +
               self.element(0, 2) * r_hand.element(2, 0) +
               self.element(0, 3) * r_hand.element(3, 0);
    let r1c2 = self.element(0, 0) * r_hand.element(0, 1) +
               self.element(0, 1) * r_hand.element(1, 1) +
               self.element(0, 2) * r_hand.element(2, 1) +
               self.element(0, 3) * r_hand.element(3, 1);
    let r1c3 = self.element(0, 0) * r_hand.element(0, 2) +
               self.element(0, 1) * r_hand.element(1, 2) +
               self.element(0, 2) * r_hand.element(2, 2) +
               self.element(0, 3) * r_hand.element(3, 2);
    let r1c4 = self.element(0, 0) * r_hand.element(0, 3) +
               self.element(0, 1) * r_hand.element(1, 3) +
               self.element(0, 2) * r_hand.element(2, 3) +
               self.element(0, 3) * r_hand.element(3, 3);
    let r2c1 = self.element(1, 0) * r_hand.element(0, 0) +
               self.element(1, 1) * r_hand.element(1, 0) +
               self.element(1, 2) * r_hand.element(2, 0) +
               self.element(1, 3) * r_hand.element(3, 0);
    let r2c2 = self.element(1, 0) * r_hand.element(0, 1) +
               self.element(1, 1) * r_hand.element(1, 1) +
               self.element(1, 2) * r_hand.element(2, 1) +
               self.element(1, 3) * r_hand.element(3, 1);
    let r2c3 = self.element(1, 0) * r_hand.element(0, 2) +
               self.element(1, 1) * r_hand.element(1, 2) +
               self.element(1, 2) * r_hand.element(2, 2) +
               self.element(1, 3) * r_hand.element(3, 2);
    let r2c4 = self.element(1, 0) * r_hand.element(0, 3) +
               self.element(1, 1) * r_hand.element(1, 3) +
               self.element(1, 2) * r_hand.element(2, 3) +
               self.element(1, 3) * r_hand.element(3, 3);
    let r3c1 = self.element(2, 0) * r_hand.element(0, 0) +
               self.element(2, 1) * r_hand.element(1, 0) +
               self.element(2, 2) * r_hand.element(2, 0) +
               self.element(2, 3) * r_hand.element(3, 0);
    let r3c2 = self.element(2, 0) * r_hand.element(0, 1) +
               self.element(2, 1) * r_hand.element(1, 1) +
               self.element(2, 2) * r_hand.element(2, 1) +
               self.element(2, 3) * r_hand.element(3, 1);
    let r3c3 = self.element(2, 0) * r_hand.element(0, 2) +
               self.element(2, 1) * r_hand.element(1, 2) +
               self.element(2, 2) * r_hand.element(2, 2) +
               self.element(2, 3) * r_hand.element(3, 2);
    let r3c4 = self.element(2, 0) * r_hand.element(0, 3) +
               self.element(2, 1) * r_hand.element(1, 3) +
               self.element(2, 2) * r_hand.element(2, 3) +
               self.element(2, 3) * r_hand.element(3, 3);
    let r4c1 = self.element(3, 0) * r_hand.element(0, 0) +
               self.element(3, 1) * r_hand.element(1, 0) +
               self.element(3, 2) * r_hand.element(2, 0) +
               self.element(3, 3) * r_hand.element(3, 0);
    let r4c2 = self.element(3, 0) * r_hand.element(0, 1) +
               self.element(3, 1) * r_hand.element(1, 1) +
               self.element(3, 2) * r_hand.element(2, 1) +
               self.element(3, 3) * r_hand.element(3, 1);
    let r4c3 = self.element(3, 0) * r_hand.element(0, 2) +
               self.element(3, 1) * r_hand.element(1, 2) +
               self.element(3, 2) * r_hand.element(2, 2) +
               self.element(3, 3) * r_hand.element(3, 2);
    let r4c4 = self.element(3, 0) * r_hand.element(0, 3) +
               self.element(3, 1) * r_hand.element(1, 3) +
               self.element(3, 2) * r_hand.element(2, 3) +
               self.element(3, 3) * r_hand.element(3, 3);

    let elements: [f64; 16] = [r1c1, r1c2, r1c3, r1c4, r2c1, r2c2, r2c3, r2c4, r3c1, r3c2, r3c3, r3c4, r4c1, r4c2, r4c3, r4c4];
    
    Matrix4x4 { elements: elements }
  }

  pub fn mult4x1(&self, r_hand: &Tuple) -> (f64, f64, f64, f64) {
    let (r_hand_x, r_hand_y, r_hand_z, r_hand_w) = r_hand.get_quad();  

    let x = self.element(0, 0) * r_hand_x +
            self.element(0, 1) * r_hand_y +
            self.element(0, 2) * r_hand_z +
            self.element(0, 3) * r_hand_w;
    let y = self.element(1, 0) * r_hand_x +
            self.element(1, 1) * r_hand_y +
            self.element(1, 2) * r_hand_z +
            self.element(1, 3) * r_hand_w;
    let z = self.element(2, 0) * r_hand_x +
            self.element(2, 1) * r_hand_y +
            self.element(2, 2) * r_hand_z +
            self.element(2, 3) * r_hand_w;
    let w = self.element(3, 0) * r_hand_x +
            self.element(3, 1) * r_hand_y +
            self.element(3, 2) * r_hand_z +
            self.element(3, 3) * r_hand_w;

    (x, y, z, w)
  }

  // convenience methods
  pub fn mult_point(&self, r_hand: &Point) -> Point {
    let (new_x, new_y, new_z, new_w) = self.mult4x1(r_hand);

    Point::new(new_x, new_y, new_z)
  }

  pub fn mult_vector(&self, r_hand: &Vector) -> Vector {
    let (new_x, new_y, new_z, new_w) = self.mult4x1(r_hand);

    Vector::new(new_x, new_y, new_z)
  }

  pub fn transpose(&self) -> Matrix4x4 {
    let r1c1 = self.element(0, 0);
    let r1c2 = self.element(1, 0);
    let r1c3 = self.element(2, 0);
    let r1c4 = self.element(3, 0);
    let r2c1 = self.element(0, 1);
    let r2c2 = self.element(1, 1);
    let r2c3 = self.element(2, 1);
    let r2c4 = self.element(3, 1);
    let r3c1 = self.element(0, 2);
    let r3c2 = self.element(1, 2);
    let r3c3 = self.element(2, 2);
    let r3c4 = self.element(3, 2);
    let r4c1 = self.element(0, 3);
    let r4c2 = self.element(1, 3);
    let r4c3 = self.element(2, 3);
    let r4c4 = self.element(3, 3);

    let elements: [f64; 16] = [r1c1, r1c2, r1c3, r1c4, r2c1, r2c2, r2c3, r2c4, r3c1, r3c2, r3c3, r3c4, r4c1, r4c2, r4c3, r4c4];
    
    Matrix4x4 { elements: elements }
  }

  pub fn submatrix(&self, row: usize, column: usize) -> Matrix3x3 {
    let mut submatrix_elements: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

    let mut current_element_index = 0;

    for i in 0..4 {
      for j in 0..4 {
        // gather elements from everywhere but row and column from arguments
        if (i != row && j != column) {
          submatrix_elements[current_element_index] = self.element(i, j);

          current_element_index += 1;
        }
      }
    }

    Matrix3x3 { elements: submatrix_elements }
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
    self.elements[0] * self.cofactor(0, 0) + self.elements[1] * self.cofactor(0, 1) + self.elements[2] * self.cofactor(0, 2) + self.elements[3] * self.cofactor(0, 3)
  }

  pub fn invertible(&self) -> bool {
    self.determinant() != 0.0
  }

  pub fn inverse(&self) -> Matrix4x4 {
    let mut inverted_elements: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

    let determinant = self.determinant();

    for i in 0..4 {
      for j in 0..4 {
        // adding to col row accomplishes transpose
        inverted_elements[(j * 4) + i] = self.cofactor(i, j) / determinant;
      }
    }

    Matrix4x4 { elements: inverted_elements }
  }

  // TODO: Move these somewhere else?
  
  pub fn translate(x_translate: f64, y_translate: f64, z_translate: f64) -> Matrix4x4 {
    let mut matrix = Matrix4x4::identity();

    // top three rows of last column hold translation
    matrix.elements[3] = x_translate;
    matrix.elements[7] = y_translate;
    matrix.elements[11] = z_translate;

    matrix
  }

  pub fn scale(x_scale: f64, y_scale: f64, z_scale: f64) -> Matrix4x4 {
    let mut matrix = Matrix4x4::identity();

    // diagonal holds scale factors
    matrix.elements[0] = x_scale;
    matrix.elements[5] = y_scale;
    matrix.elements[10] = z_scale;

    matrix
  }

  // TODO: Implement rotation around arbitrary axis

  pub fn rotate_x(radians: f64) -> Matrix4x4 {
    let mut matrix = Matrix4x4::identity();

    // rotate y and z components around x
    matrix.elements[5] = radians.cos();
    matrix.elements[6] = -radians.sin();
    matrix.elements[9] = radians.sin();
    matrix.elements[10] = radians.cos();

    matrix
  }

  pub fn rotate_y(radians: f64) -> Matrix4x4 {
    let mut matrix = Matrix4x4::identity();

    // rotate x and z components around y
    matrix.elements[0] = radians.cos();
    matrix.elements[2] = radians.sin();
    matrix.elements[8] = -radians.sin();
    matrix.elements[10] = radians.cos();

    matrix
  }

  pub fn rotate_z(radians: f64) -> Matrix4x4 {
    let mut matrix = Matrix4x4::identity();

    // rotate x and y components around z
    matrix.elements[0] = radians.cos();
    matrix.elements[1] = -radians.sin();
    matrix.elements[4] = radians.sin();
    matrix.elements[5] = radians.cos();

    matrix
  }

  pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4x4 {
    let mut matrix = Matrix4x4::identity();

    // moves elements in proportion to others
    matrix.elements[1] = xy;
    matrix.elements[2] = xz;
    matrix.elements[4] = yx;
    matrix.elements[6] = yz;
    matrix.elements[8] = zx;
    matrix.elements[9] = zy;

    matrix
  }

  pub fn view_transform(from: &Point, to: &Point, up: &Vector) -> Matrix4x4 {
    let forward = to.subtract_point(&from).normalize();
    let up_n = up.normalize();
    let left = forward.cross(&up_n);
    let true_up = left.cross(&forward);

    let orientation = Matrix4x4::new(
      left.x, left.y, left.z, 0.0,
      true_up.x, true_up.y, true_up.z, 0.0,
      -forward.x, -forward.y, -forward.z, 0.0,
      0.0, 0.0, 0.0, 1.0
    );

    orientation.mult4x4(&Matrix4x4::translate(-from.x, -from.y, -from.z))
  }
}
