#[cfg(test)]
mod tests {
  use crate::rendering::math::tuple::Tuple;
  use crate::rendering::math::Point;
  use crate::rendering::math::Vector;
  
  use crate::rendering::math::Color;

  use crate::rendering::math::Matrix4x4;

  use crate::rendering::shapes::shape::Shape;
  use crate::rendering::shapes::Cylinder;

  use crate::rendering::Ray;
  
  use crate::rendering::Material;

  #[test]
  fn cylinder_created_with_transform_and_material() {
    let transform = Matrix4x4::translate(5.0, -3.0, 2.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cylinder = Cylinder::new(transform, false, 10.0, 20.0, material);

    assert!(cylinder.transform == Matrix4x4::translate(5.0, -3.0, 2.0));
    assert!(cylinder.capped == false);
    assert!(cylinder.minimum == 10.0);
    assert!(cylinder.maximum == 20.0);
    assert!(cylinder.material.ambient == 0.1);
    assert!(cylinder.material.diffuse == 0.9);
    assert!(cylinder.material.specular == 0.9);
    assert!(cylinder.material.shininess == 200.0);
  }

  #[test]
  fn ray_intersects_cylinder() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cylinder = Cylinder::new(transform, false, -9999999.9, 9999999.9, material);

    let ray = Ray::new(&Point::new(1.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 5.0);
    assert!(intersections[1].t == 5.0);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 4.0);
    assert!(intersections[1].t == 6.0);

    let ray = Ray::new(&Point::new(0.5, 0.0, -5.0), &Vector::new(0.1, 1.0, 1.0).normalize());

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 6.80798191702732);
    assert!(intersections[1].t == 7.088723439378861);
  }

  #[test]
  fn ray_intersects_constrained_cylinder() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cylinder = Cylinder::new(transform, false, 1.0, 2.0, material);

    let ray = Ray::new(&Point::new(0.0, 1.5, 0.0), &Vector::new(0.1, 1.0, 1.0).normalize());

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);

    let ray = Ray::new(&Point::new(0.0, 3.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);

    let ray = Ray::new(&Point::new(0.0, 2.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);

    let ray = Ray::new(&Point::new(0.0, 1.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);

    let ray = Ray::new(&Point::new(0.0, 1.5, -2.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
  }

  #[test]
  fn ray_intersects_cylinder_caps() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cylinder = Cylinder::new(transform, true, 1.0, 2.0, material);

    let ray = Ray::new(&Point::new(0.0, 3.0, 0.0), &Vector::new(0.0, -1.0, 0.0));

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);

    let ray = Ray::new(&Point::new(0.0, 3.0, -2.0), &Vector::new(0.0, -1.0, 2.0).normalize());

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);

    let ray = Ray::new(&Point::new(0.0, 4.0, -2.0), &Vector::new(0.0, -1.0, 1.0).normalize());

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);

    let ray = Ray::new(&Point::new(0.0, 0.0, -2.0), &Vector::new(0.0, 1.0, 2.0).normalize());

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);

    let ray = Ray::new(&Point::new(0.0, -1.0, -2.0), &Vector::new(0.0, 1.0, 1.0).normalize());

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
  }

  #[test]
  fn ray_misses_cylinder() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cylinder = Cylinder::new(transform, false, -9999999.9, 9999999.9, material);

    let ray = Ray::new(&Point::new(1.0, 0.0, 0.0), &Vector::new(0.0, 1.0, 0.0));

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);

    let ray = Ray::new(&Point::new(0.0, 0.0, 0.0), &Vector::new(0.0, 1.0, 0.0));

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(1.0, 1.0, 1.0).normalize());

    let intersections = cylinder.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);
  }

  #[test]
  fn normal_on_cylinder() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cylinder = Cylinder::new(transform, false, -9999999.9, 9999999.9, material);

    let normal = cylinder.normal_at(&Point::new(1.0, 0.0, 0.0));

    assert_eq!(normal.get_quad(), (1.0, 0.0, 0.0, 0.0));

    let normal = cylinder.normal_at(&Point::new(0.0, 5.0, -1.0));

    assert_eq!(normal.get_quad(), (0.0, 0.0, -1.0, 0.0));

    let normal = cylinder.normal_at(&Point::new(0.0, -2.0, 1.0));

    assert_eq!(normal.get_quad(), (0.0, 0.0, 1.0, 0.0));

    let normal = cylinder.normal_at(&Point::new(-1.0, 1.0, 0.0));

    assert_eq!(normal.get_quad(), (-1.0, 0.0, 0.0, 0.0));
  }

  #[test]
  fn normal_on_cylinder_caps() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cylinder = Cylinder::new(transform, false, 1.0, 2.0, material);

    let normal = cylinder.normal_at(&Point::new(0.0, 1.0, 0.0));

    assert_eq!(normal.get_quad(), (0.0, -1.0, 0.0, 0.0));

    let normal = cylinder.normal_at(&Point::new(0.5, 1.0, 0.0));

    assert_eq!(normal.get_quad(), (0.0, -1.0, 0.0, 0.0));

    let normal = cylinder.normal_at(&Point::new(0.0, 1.0, 0.5));

    assert_eq!(normal.get_quad(), (0.0, -1.0, 0.0, 0.0));

    let normal = cylinder.normal_at(&Point::new(0.0, 2.0, 0.0));

    assert_eq!(normal.get_quad(), (0.0, 1.0, 0.0, 0.0));

    let normal = cylinder.normal_at(&Point::new(0.5, 2.0, 0.0));

    assert_eq!(normal.get_quad(), (0.0, 1.0, 0.0, 0.0));

    let normal = cylinder.normal_at(&Point::new(0.0, 2.0, 0.5));

    assert_eq!(normal.get_quad(), (0.0, 1.0, 0.0, 0.0));
  }
}
