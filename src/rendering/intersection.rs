use crate::rendering::shape::Shape;

pub struct Intersection<'a> {
  pub t: f64,
  pub object: &'a dyn Shape  
}

impl<'a> Intersection<'a> {
  pub fn insert_intersection(intersection_list: &mut Vec<Intersection<'a>>, new_intersections: &mut Vec<Intersection<'a>>) -> Vec<Intersection<'a>> {
    let mut combined_intersections: Vec<Intersection> = Vec::new();
    
    combined_intersections.append(intersection_list);
    combined_intersections.append(new_intersections);
    
    combined_intersections.sort_by(|intersection_a, intersection_b| intersection_a.t.partial_cmp(&intersection_b.t).unwrap() );

    combined_intersections
  }

  pub fn get_hit<'b>(intersection_list: &'b Vec<Intersection<'a>>) -> Option<&'b Intersection<'a>> {
    for intersection in intersection_list {
      if intersection.t > 0.0 {
        return Some(&intersection);
      }
    }

    None
  }

  pub fn new(t: f64, object: &'a dyn Shape) -> Intersection {
    Intersection { t: t, object: object }
  }
}
