use crate::rendering::shape::Shape;

use crate::rendering::Intersection;
use crate::rendering::Ray;

use crate::math::tuple::Tuple;
use crate::math::Point;
use crate::math::Vector;

use crate::math::Matrix4x4;

pub struct Computations<'a> {
  pub t: f64,
  pub point: Point,
  pub eye_v: Vector,
  pub normal: Vector,
  pub inside: bool,
  pub over_point: Point,
  pub object: &'a dyn Shape  
}

impl<'a> Computations<'a> {
  pub fn new<'b>(intersection: &'b Intersection<'a>, ray: &Ray) -> Computations<'a> {
    let t = intersection.t;
    let object = intersection.object;

    let point = ray.position(t);
    let eye_v = ray.direction.multiply(-1.0);
    let mut normal = object.normal_at(&point);

    let mut inside = false;

    // Check if eye vector is pointing away from normal, to test if inside object
    if normal.dot(&eye_v) < 0.0 {
      inside = true;
      normal = normal.multiply(-1.0);
    }

    // Point slightly off of object surface prevents shadow precision error
    let over_point = point.add_vector(&normal.multiply(0.0001)); // TODO: Use global epsilon 
    
    Computations { 
      t: t, 
      point: point, 
      eye_v: eye_v, 
      normal: normal, 
      inside: inside,
      over_point: over_point, 
      object: object 
    } 
  }
}
