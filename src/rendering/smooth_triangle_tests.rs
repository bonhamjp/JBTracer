#[cfg(test)]
mod tests {
  use std::f64;

  use crate::rendering::shape::Shape;
  use crate::rendering::SmoothTriangle;

  use crate::rendering::Ray;
  use crate::rendering::Intersection;
  
  use crate::rendering::Material;

  use crate::math::tuple::Tuple;
  use crate::math::Point;
  use crate::math::Vector;
  
  use crate::math::Color;

  use crate::math::Matrix4x4;

  #[test]
  fn smooth_triangle_created_with_points_transform_and_material() {
    let transform = Matrix4x4::translate(0.0, 0.0, 0.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    
    let point_1 = Point::new(0.0, 1.0, 0.0);
    let point_2 = Point::new(-1.0, 0.0, 0.0);
    let point_3 = Point::new(1.0, 0.0, 0.0);
    let normal_1 = Vector::new(0.0, 1.0, 0.0);
    let normal_2 = Vector::new(-1.0, 0.0, 0.0);
    let normal_3 = Vector::new(1.0, 0.0, 0.0);
    let smooth_triangle = SmoothTriangle::new(1, point_1, point_2, point_3, normal_1, normal_2, normal_3, transform, material);

    assert!(smooth_triangle.transform == Matrix4x4::translate(0.0, 0.0, 0.0));
    assert!(smooth_triangle.point_1.get_quad() == Point::new(0.0, 1.0, 0.0).get_quad());
    assert!(smooth_triangle.point_2.get_quad() == Point::new(-1.0, 0.0, 0.0).get_quad());
    assert!(smooth_triangle.point_3.get_quad() == Point::new(1.0, 0.0, 0.0).get_quad());
    assert!(smooth_triangle.normal_1.get_quad() == Vector::new(0.0, 1.0, 0.0).get_quad());
    assert!(smooth_triangle.normal_2.get_quad() == Vector::new(-1.0, 0.0, 0.0).get_quad());
    assert!(smooth_triangle.normal_3.get_quad() == Vector::new(1.0, 0.0, 0.0).get_quad());
    assert!(smooth_triangle.edge_1.get_quad() == Vector::new(-1.0, -1.0, 0.0).get_quad());
    assert!(smooth_triangle.edge_2.get_quad() == Vector::new(1.0, -1.0, 0.0).get_quad());
    assert!(smooth_triangle.material.ambient == 0.1);
    assert!(smooth_triangle.material.diffuse == 0.9);
    assert!(smooth_triangle.material.specular == 0.9);
    assert!(smooth_triangle.material.shininess == 200.0);
  }

  #[test]
  fn smooth_triangle_uses_u_and_v_to_interpolate_normal() {
    let transform = Matrix4x4::translate(0.0, 0.0, 0.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    
    let point_1 = Point::new(0.0, 1.0, 0.0);
    let point_2 = Point::new(-1.0, 0.0, 0.0);
    let point_3 = Point::new(1.0, 0.0, 0.0);
    let normal_1 = Vector::new(0.0, 1.0, 0.0);
    let normal_2 = Vector::new(-1.0, 0.0, 0.0);
    let normal_3 = Vector::new(1.0, 0.0, 0.0);
    let smooth_triangle = SmoothTriangle::new(1, point_1, point_2, point_3, normal_1, normal_2, normal_3, transform, material);

    let normal = smooth_triangle.normal_at_with_uv(&Point::new(0.0, 0.0, 0.0), 0.45, 0.25);

    assert_eq!(normal.x, -0.5547001962252291);
    assert_eq!(normal.y, 0.8320502943378437);
    assert_eq!(normal.z, 0.0);
  }
}
