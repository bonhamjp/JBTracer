#[cfg(test)]
mod tests {
//   #[test]
//   fn new_sets_values() {
//     use crate::math::Point;

//     let point = Point::new(10.10, 20.20, 30.30);

//     assert_eq!(point.x, 10.10);
//     assert_eq!(point.y, 20.20);
//     assert_eq!(point.z, 30.30);
//   }

  #[test]
  fn ray_intersects_sphere_at_two_point() {
    use crate::rendering::shape::Shape;
    use crate::rendering::Sphere;
    use crate::rendering::Ray;
    use crate::rendering::Intersection;
    
    use crate::math::Point;
    use crate::math::Vector;
  
    let sphere = Sphere::new();

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray);

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 4.0);
    assert!(intersections[1].t == 6.0);
  }

  #[test]
  fn ray_intersects_sphere_at_tangent() {
    use crate::rendering::shape::Shape;
    use crate::rendering::Sphere;
    use crate::rendering::Ray;
    use crate::rendering::Intersection;
    
    use crate::math::Point;
    use crate::math::Vector;
  
    let sphere = Sphere::new();

    let ray = Ray::new(&Point::new(0.0, 1.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray);

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 5.0);
    assert!(intersections[1].t == 5.0);
  }

  #[test]
  fn ray_misses_sphere() {
    use crate::rendering::shape::Shape;
    use crate::rendering::Sphere;
    use crate::rendering::Ray;
    use crate::rendering::Intersection;
    
    use crate::math::Point;
    use crate::math::Vector;
  
    let sphere = Sphere::new();

    let ray = Ray::new(&Point::new(0.0, 2.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray);

    assert!(intersections.len() == 0);
  }

  #[test]
  fn ray_originates_in_sphere() {
    use crate::rendering::shape::Shape;
    use crate::rendering::Sphere;
    use crate::rendering::Ray;
    use crate::rendering::Intersection;
    
    use crate::math::Point;
    use crate::math::Vector;
  
    let sphere = Sphere::new();

    let ray = Ray::new(&Point::new(0.0, 0.0, 0.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray);
    
    assert!(intersections.len() == 2);
    assert!(intersections[0].t == -1.0);
    assert!(intersections[1].t == 1.0);
  }

  #[test]
  fn ray_points_away_from_sphere() {
    use crate::rendering::shape::Shape;
    use crate::rendering::Sphere;
    use crate::rendering::Ray;
    use crate::rendering::Intersection;
    
    use crate::math::Point;
    use crate::math::Vector;
  
    let sphere = Sphere::new();

    let ray = Ray::new(&Point::new(0.0, 0.0, 5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = sphere.intersections(&ray);
    
    assert!(intersections.len() == 2);
    assert!(intersections[0].t == -6.0);
    assert!(intersections[1].t == -4.0);
  }
}
