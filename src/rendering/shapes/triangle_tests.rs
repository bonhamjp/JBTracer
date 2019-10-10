#[cfg(test)]
mod tests {
  use crate::rendering::math::tuple::Tuple;
  use crate::rendering::math::Point;
  use crate::rendering::math::Vector;
  
  use crate::rendering::math::Color;

  use crate::rendering::math::Matrix4x4;

  use crate::rendering::shapes::shape::Shape;
  use crate::rendering::shapes::Triangle;

  use crate::rendering::Ray;
  
  use crate::rendering::Material;

  #[test]
  fn triangle_created_with_points_transform_and_material() {
    let transform = Matrix4x4::translate(5.0, -3.0, 2.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    
    let point_1 = Point::new(0.0, 1.0, 0.0);
    let point_2 = Point::new(-1.0, 0.0, 0.0);
    let point_3 = Point::new(1.0, 0.0, 0.0);
    let triangle = Triangle::new(point_1, point_2, point_3, transform, material);

    assert!(triangle.transform == Matrix4x4::translate(5.0, -3.0, 2.0));
    assert!(triangle.point_1.get_quad() == Point::new(0.0, 1.0, 0.0).get_quad());
    assert!(triangle.point_2.get_quad() == Point::new(-1.0, 0.0, 0.0).get_quad());
    assert!(triangle.point_3.get_quad() == Point::new(1.0, 0.0, 0.0).get_quad());
    assert!(triangle.edge_1.get_quad() == Vector::new(-1.0, -1.0, 0.0).get_quad());
    assert!(triangle.edge_2.get_quad() == Vector::new(1.0, -1.0, 0.0).get_quad());
    assert!(triangle.normal.get_quad() == Vector::new(0.0, 0.0, -1.0).get_quad());
    assert!(triangle.material.ambient == 0.1);
    assert!(triangle.material.diffuse == 0.9);
    assert!(triangle.material.specular == 0.9);
    assert!(triangle.material.shininess == 200.0);
  }

  #[test]
  fn normal_on_triangle() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    
    let point_1 = Point::new(0.0, 1.0, 0.0);
    let point_2 = Point::new(-1.0, 0.0, 0.0);
    let point_3 = Point::new(1.0, 0.0, 0.0);
    let triangle = Triangle::new(point_1, point_2, point_3, transform, material);

    let normal = triangle.normal_at(&Point::new(0.0, 0.5, 0.0));

    assert_eq!(normal.get_quad(), triangle.normal.get_quad());

    let normal = triangle.normal_at(&Point::new(-0.5, 0.75, 0.0));

    assert_eq!(normal.get_quad(), triangle.normal.get_quad());

    let normal = triangle.normal_at(&Point::new(0.5, 0.25, 0.0));

    assert_eq!(normal.get_quad(), triangle.normal.get_quad());
  }

  #[test]
  fn ray_is_parallel_to_triangle() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    
    let point_1 = Point::new(0.0, 1.0, 0.0);
    let point_2 = Point::new(-1.0, 0.0, 0.0);
    let point_3 = Point::new(1.0, 0.0, 0.0);
    let triangle = Triangle::new(point_1, point_2, point_3, transform, material);

    let ray = Ray::new(&Point::new(0.0, -1.0, -2.0), &Vector::new(0.0, 1.0, 0.0));

    let intersections = triangle.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);
  }

  #[test]
  fn ray_misses_point_1_to_point_3_triangle_edge() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    
    let point_1 = Point::new(0.0, 1.0, 0.0);
    let point_2 = Point::new(-1.0, 0.0, 0.0);
    let point_3 = Point::new(1.0, 0.0, 0.0);
    let triangle = Triangle::new(point_1, point_2, point_3, transform, material);

    let ray = Ray::new(&Point::new(1.0, 1.0, -2.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = triangle.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);
  }

  #[test]
  fn ray_misses_point_1_to_point_2_triangle_edge() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    
    let point_1 = Point::new(0.0, 1.0, 0.0);
    let point_2 = Point::new(-1.0, 0.0, 0.0);
    let point_3 = Point::new(1.0, 0.0, 0.0);
    let triangle = Triangle::new(point_1, point_2, point_3, transform, material);

    let ray = Ray::new(&Point::new(-1.0, 1.0, -2.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = triangle.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);      
  }

  #[test]
  fn ray_misses_point_2_to_point_3_triangle_edge() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    
    let point_1 = Point::new(0.0, 1.0, 0.0);
    let point_2 = Point::new(-1.0, 0.0, 0.0);
    let point_3 = Point::new(1.0, 0.0, 0.0);
    let triangle = Triangle::new(point_1, point_2, point_3, transform, material);

    let ray = Ray::new(&Point::new(0.0, -1.0, -2.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = triangle.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);         
  }

  #[test]
  fn ray_hits_triangle() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    
    let point_1 = Point::new(0.0, 1.0, 0.0);
    let point_2 = Point::new(-1.0, 0.0, 0.0);
    let point_3 = Point::new(1.0, 0.0, 0.0);
    let triangle = Triangle::new(point_1, point_2, point_3, transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.5, -2.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = triangle.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 1); 
    assert!(intersections[0].t == 2.0);
  }
}
