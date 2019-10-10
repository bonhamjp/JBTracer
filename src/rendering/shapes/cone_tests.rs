#[cfg(test)]
mod tests {
  use std::f64;

  use crate::rendering::math::tuple::Tuple;
  use crate::rendering::math::Point;
  use crate::rendering::math::Vector;
  
  use crate::rendering::math::Color;

  use crate::rendering::math::Matrix4x4;

  use crate::rendering::shapes::shape::Shape;
  use crate::rendering::shapes::Cone;

  use crate::rendering::Ray;
  
  use crate::rendering::Material;

  #[test]
  fn cone_created_with_transform_and_material() {
    let transform = Matrix4x4::translate(5.0, -3.0, 2.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cone = Cone::new(transform, false, 10.0, 20.0, material);

    assert!(cone.transform == Matrix4x4::translate(5.0, -3.0, 2.0));
    assert!(cone.capped == false);
    assert!(cone.minimum == 10.0);
    assert!(cone.maximum == 20.0);
    assert!(cone.material.ambient == 0.1);
    assert!(cone.material.diffuse == 0.9);
    assert!(cone.material.specular == 0.9);
    assert!(cone.material.shininess == 200.0);
  }

  #[test]
  fn ray_intersects_cone() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cone = Cone::new(transform, false, -9999999.9, 9999999.9, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = cone.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
    assert_eq!(intersections[0].t, 5.0);
    assert_eq!(intersections[1].t, 5.0);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(1.0, 1.0, 1.0).normalize());

    let intersections = cone.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
    assert_eq!(intersections[0].t, 8.660254037844386);
    assert_eq!(intersections[1].t, 8.660254037844386);

    let ray = Ray::new(&Point::new(1.0, 1.0, -5.0), &Vector::new(-0.5, -1.0, 1.0).normalize());

    let intersections = cone.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
    assert_eq!(intersections[0].t, 4.550055679356349);
    assert_eq!(intersections[1].t, 49.449944320643645);
  }

  #[test]
  fn ray_intersects_cone_parallel_to_a_half() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cone = Cone::new(transform, false, -9999999.9, 9999999.9, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -1.0), &Vector::new(0.0, 1.0, 1.0).normalize());

    let intersections = cone.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 1);
    assert_eq!(intersections[0].t, 0.3535533905932738);
  }

  #[test]
  fn ray_intersects_cone_caps() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cone = Cone::new(transform, true, -0.5, 0.5, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 1.0, 0.0));

    let intersections = cone.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);

    let ray = Ray::new(&Point::new(0.0, 0.0, -0.25), &Vector::new(0.0, 1.0, 1.0).normalize());

    let intersections = cone.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);

    let ray = Ray::new(&Point::new(0.0, 0.0, -0.25), &Vector::new(0.0, 1.0, 0.0));

    let intersections = cone.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 4);
  }

  #[test]
  fn normal_on_cone() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cone = Cone::new(transform, true, -1.0, 1.0, material);

    let normal = cone.normal_at(&Point::new(0.0, 0.0, 0.0));

    assert_eq!(normal.get_quad(), (0.0, 0.0, 0.0, 0.0));

    let normal = cone.normal_at(&Point::new(1.0, 1.0, 1.0));

    assert_eq!(normal.get_quad(), Vector::new(1.0, -(2.0 as f64).sqrt(), 1.0).normalize().get_quad());

    let normal = cone.normal_at(&Point::new(-1.0, -1.0, 0.0));

    assert_eq!(normal.x, -0.7071067811865476);
    assert_eq!(normal.y, 0.7071067811865476);
    assert_eq!(normal.z, 0.0);
  }
}
