use std::rc::Rc;
use std::rc::Weak;

use crate::rendering::shape::Shape;

pub struct Intersection {
  pub t: f64 // ,
//   pub object: Weak<Shape>
}

impl Intersection {
  pub fn new(t: f64) -> Intersection {
    // Intersection { t: t, object: Weak::new() }
    Intersection { t: t }
  }
}
