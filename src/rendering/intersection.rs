use crate::rendering::shape::Shape;

use crate::math::Matrix4x4;

pub struct Intersection<'a> {
  pub t: f64,
  pub object: &'a dyn Shape,
  pub world_to_container: Matrix4x4,
  pub normal_to_world: Matrix4x4,
  pub u: f64,
  pub v: f64
}

impl<'a> Intersection<'a> {
  pub fn insert_intersection(intersection_list: &mut Vec<Intersection<'a>>, new_intersections: &mut Vec<Intersection<'a>>) -> Vec<Intersection<'a>> {
    let mut combined_intersections: Vec<Intersection> = Vec::new();
    
    combined_intersections.append(intersection_list);
    combined_intersections.append(new_intersections);
    
    combined_intersections.sort_by(|intersection_a, intersection_b| intersection_a.t.partial_cmp(&intersection_b.t).unwrap() );

    combined_intersections
  }

  pub fn get_hit<'c>(intersection_list: &'c Vec<Intersection<'a>>) -> Option<&'c Intersection<'a>> {
    for intersection in intersection_list {
      if intersection.t > 0.0 {
        return Some(&intersection);
      }
    }

    None
  }

  pub fn new(t: f64, object: &'a dyn Shape, world_to_container: Matrix4x4, normal_to_world: Matrix4x4) -> Intersection<'a> {
    Intersection { 
      t: t, 
      object: object,
      world_to_container: world_to_container, 
      normal_to_world: normal_to_world,
      u: 0.0,
      v: 0.0
    }
  }

  pub fn new_with_uv(t: f64, object: &'a dyn Shape, world_to_container: Matrix4x4, normal_to_world: Matrix4x4, u: f64, v: f64) -> Intersection<'a> {
    Intersection { 
      t: t, 
      object: object,
      world_to_container: world_to_container, 
      normal_to_world: normal_to_world,
      u: u,
      v: v
    }
  }
}
