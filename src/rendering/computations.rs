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
  pub under_point: Point,
  pub n1: f64,
  pub n2: f64,
  pub reflect_v: Vector,
  pub object: &'a dyn Shape  
}

impl<'a> Computations<'a> {
  pub fn new<'b>(hit: &'b Intersection<'a>, ray: &Ray, intersections: &Vec<Intersection<'a>>) -> Computations<'a> {
    let t = hit.t;
    let object = hit.object;

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
    
    // Point slightly under surface of object, for refraction calculations
    let under_point = point.subtract_vector(&normal.multiply(0.0001)); // TODO: Use global epsilon

    // Refractive indices of a materials being transitioned to and from
    let (n1, n2) = calculate_refractive_indices(hit, intersections);

    // Only calculate reflection vector if object material is reflective
    let mut reflect_v = Vector::new(0.0, 0.0, 0.0);

    if object.get_material().reflectiveness > 0.0 {
      reflect_v = ray.direction.reflect(&normal);
    }

    Computations { 
      t: t, 
      point: point, 
      eye_v: eye_v, 
      normal: normal, 
      inside: inside,
      over_point: over_point,
      under_point: under_point,
      n1: n1,
      n2: n2, 
      reflect_v: reflect_v,
      object: object 
    } 
  }
}

fn calculate_refractive_indices<'a: 'b, 'b>(hit: &'b Intersection<'a>, intersections: &Vec<Intersection<'a>>) -> (f64, f64) {
  let mut objects: Vec<&dyn Shape> = Vec::new();
  
  let mut n1 = 1.0;
  let mut n2 = 1.0;

  for intersection in intersections {
    // Set leaving refractor index when hit intersection found
    if hit.t == intersection.t && hit.object.get_id() == intersection.object.get_id() {
      if objects.len() == 0 {
        n1 = 1.0;
      }
      else {
        let last_object = objects.last().unwrap(); 

        n1 = last_object.get_material().refractive_index;
      }
    }

    let entered_object = objects.iter().position(|&o| o.get_id() == intersection.object.get_id());     
    // Remove object if already encountered
    if let Some(i) = entered_object {
      objects.remove(i);  

    // Add object if first encountered
    } else {
      objects.push(intersection.object);
    }

    // Set Entering refractor index when hit intersection found
    if hit.t == intersection.t && hit.object.get_id() == intersection.object.get_id() {      
      if objects.len() == 0 {
        n2 = 1.0;
      }
      else {
        let last_object = objects.last().unwrap(); 

        n2 = last_object.get_material().refractive_index;
      }

      return (n1, n2);
    }
  } 

  (n1, n2)
}
