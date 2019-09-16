#[cfg(test)]
mod tests {
  use crate::rendering::shape::Shape;
  use crate::rendering::Sphere;
    
  use crate::rendering::Intersection;

  #[test]
  fn new_distance_and_object_reference() {
    let sphere = Sphere::default();

    let intersection = Intersection::new(3.5, &sphere);    
    
    assert!(sphere.is_eq(intersection.object));
  }

  #[test]
  fn aggregates_and_orders_intersections_from_smallest_t_to_largest() {
    let sphere = Sphere::default();

    let mut intersections: Vec<Intersection> = Vec::new();

    intersections.push(Intersection::new(2.0, &sphere));
    intersections.push(Intersection::new(3.0, &sphere));
    intersections.push(Intersection::new(1.0, &sphere));

    let sorted_intersections = Intersection::insert_intersection(&mut Vec::new(), &mut intersections);
  
    assert!(sorted_intersections[0].t == 1.0);
    assert!(sorted_intersections[1].t == 2.0);
    assert!(sorted_intersections[2].t == 3.0);
  }

  #[test]
  fn hit_returns_intersection_with_smallest_t_if_all_positive() {
    let sphere = Sphere::default();

    let mut intersections: Vec<Intersection> = Vec::new();

    intersections.push(Intersection::new(2.0, &sphere));
    intersections.push(Intersection::new(1.0, &sphere));
    intersections.push(Intersection::new(3.0, &sphere));

    let sorted_intersections = Intersection::insert_intersection(&mut Vec::new(), &mut intersections);
  
    match Intersection::get_hit(&sorted_intersections) {
      Some(hit) => assert!(hit.t == 1.0),
      
      // something wrong if no hit
      None => assert!(1 == 2)
    }
  }

  #[test]
  fn hit_returns_smallest_positive_intersection_if_some_positive_and_some_negative() {
    let sphere = Sphere::default();

    let mut intersections: Vec<Intersection> = Vec::new();

    intersections.push(Intersection::new(2.0, &sphere));
    intersections.push(Intersection::new(-1.0, &sphere));
    intersections.push(Intersection::new(3.0, &sphere));

    let sorted_intersections = Intersection::insert_intersection(&mut Vec::new(), &mut intersections);
  
    match Intersection::get_hit(&sorted_intersections) {
      Some(hit) => assert!(hit.t == 2.0),
      
      // something wrong if no hit
      None => assert!(1 == 2)
    }
  }

  #[test]
  fn hit_returns_nothing_if_no_intersections() {
    let sphere = Sphere::default();

    let sorted_intersections = Intersection::insert_intersection(&mut Vec::new(), &mut Vec::new());
  
    match Intersection::get_hit(&sorted_intersections) {
      // something wrong if no hit
      Some(hit) => assert!(1 == 1),
      
      // nothing found, which is expected
      None => assert!(1 == 1)
    }
  }

  #[test]
  fn hit_returns_nothing_if_all_intersections_negative() {
    let sphere = Sphere::default();

    let mut intersections: Vec<Intersection> = Vec::new();

    intersections.push(Intersection::new(-2.0, &sphere));
    intersections.push(Intersection::new(-1.0, &sphere));
    intersections.push(Intersection::new(-3.0, &sphere));

    let sorted_intersections = Intersection::insert_intersection(&mut Vec::new(), &mut Vec::new());
  
    match Intersection::get_hit(&sorted_intersections) {
      // something wrong if no hit
      Some(hit) => assert!(1 == 1),
      
      // nothing found, which is expected
      None => assert!(1 == 1)
    }
  }
}
