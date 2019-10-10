pub trait Tuple {
  fn get_quad(&self) -> (f64, f64, f64, f64);

  fn internal_add(&self, r_hand: &Tuple) -> (f64, f64, f64) {
    let (l_hand_x, l_hand_y, l_hand_z, _) = self.get_quad();
    let (r_hand_x, r_hand_y, r_hand_z, _) = r_hand.get_quad();

    (l_hand_x + r_hand_x, l_hand_y + r_hand_y, l_hand_z + r_hand_z)
  }

  fn internal_subtract(&self, r_hand: &Tuple) -> (f64, f64, f64) {
    let (l_hand_x, l_hand_y, l_hand_z, _) = self.get_quad();
    let (r_hand_x, r_hand_y, r_hand_z, _) = r_hand.get_quad();

    (l_hand_x - r_hand_x, l_hand_y - r_hand_y, l_hand_z - r_hand_z)
  }

  fn internal_multiply(&self, r_hand: f64) -> (f64, f64, f64) {
    let (l_hand_x, l_hand_y, l_hand_z, _) = self.get_quad();

    (l_hand_x * r_hand, l_hand_y * r_hand, l_hand_z * r_hand)
  }

  fn internal_divide(&self, r_hand: f64) -> (f64, f64, f64) {
    let (l_hand_x, l_hand_y, l_hand_z, _) = self.get_quad();
    
    (l_hand_x / r_hand, l_hand_y / r_hand, l_hand_z / r_hand)
  }

  fn magnitude(&self) -> f64 {
    let (x, y, z, w) = self.get_quad();

    ((x * x) + (y * y) + (z * z) + (w * w)).sqrt()
  }

  fn dot(&self, r_hand: &Tuple) -> f64 {
    let (l_hand_x, l_hand_y, l_hand_z, l_hand_w) = self.get_quad();
    let (r_hand_x, r_hand_y, r_hand_z, r_hand_w) = r_hand.get_quad();
    
    l_hand_x * r_hand_x + l_hand_y * r_hand_y + l_hand_z * r_hand_z + l_hand_w * r_hand_w
  }
}
