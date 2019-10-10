#[cfg(test)]
mod tests {
  use crate::rendering::math::Point;
  use crate::rendering::math::Vector;
  
  use crate::rendering::math::Color;

  use crate::rendering::math::Matrix4x4;

  use crate::rendering::shapes::shape::Shape;
  use crate::rendering::shapes::Plane;

  use crate::rendering::Ray;
  
  use crate::rendering::Material;

  #[test]
  fn plane_created_with_transform_and_material() {
    let transform = Matrix4x4::translate(5.0, -3.0, 2.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let plane = Plane::new(transform, material);

    assert!(plane.transform == Matrix4x4::translate(5.0, -3.0, 2.0));
    assert!(plane.material.ambient == 0.1);
    assert!(plane.material.diffuse == 0.9);
    assert!(plane.material.specular == 0.9);
    assert!(plane.material.shininess == 200.0);
  }

  #[test]
  fn normal_of_plane_is_constant() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let plane = Plane::new(transform, material);

    let normal_1 = plane.normal_at(&Point::new(0.0, 0.0, 0.0));
    let normal_2 = plane.normal_at(&Point::new(10.0, 0.0, -10.0));
    let normal_3 = plane.normal_at(&Point::new(-5.0, 0.0, 150.0));

    let up_normal = Vector::new(0.0, 1.0, 0.0);

    assert!(normal_1 == up_normal);
    assert!(normal_2 == up_normal);
    assert!(normal_3 == up_normal);
  }

  #[test]
  fn ray_does_not_intersect_plane_if_parallel() {
   let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let plane = Plane::new(transform, material);

    let ray = Ray::new(&Point::new(0.0, 10.0, 10.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = plane.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert_eq!(intersections.len(), 0);
  }

  #[test]
  fn ray_does_not_intersect_plane_if_coplanar() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let plane = Plane::new(transform, material);

    let ray = Ray::new(&Point::new(0.0, 0.0, 0.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = plane.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert_eq!(intersections.len(), 0);
  }

  #[test]
  fn ray_intersects_plane_from_above() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let plane = Plane::new(transform, material);

    let ray = Ray::new(&Point::new(0.0, 1.0, 0.0), &Vector::new(0.0, -1.0, 0.0));

    let intersections = plane.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert_eq!(intersections.len(), 1);
    assert_eq!(intersections[0].t, 1.0);
    assert!(plane.is_eq(intersections[0].object));
  }

  #[test]
  fn ray_intersects_plane_from_below() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let plane = Plane::new(transform, material);

    let ray = Ray::new(&Point::new(0.0, -1.0, 0.0), &Vector::new(0.0, 1.0, 0.0));

    let intersections = plane.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert_eq!(intersections.len(), 1);
    assert_eq!(intersections[0].t, 1.0);
    assert!(plane.is_eq(intersections[0].object));
  }
}
