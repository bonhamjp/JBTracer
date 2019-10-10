#[cfg(test)]
mod tests {
  use crate::rendering::math::tuple::Tuple;
  use crate::rendering::math::Point;
  use crate::rendering::math::Vector;
  
  use crate::rendering::math::Color;

  use crate::rendering::math::Matrix4x4;

  use crate::rendering::shapes::shape::Shape;
  use crate::rendering::shapes::Cube;

  use crate::rendering::Ray;
  
  use crate::rendering::Material;

  #[test]
  fn cube_created_with_transform_and_material() {
    let transform = Matrix4x4::translate(5.0, -3.0, 2.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cube = Cube::new(transform, material);

    assert!(cube.transform == Matrix4x4::translate(5.0, -3.0, 2.0));
    assert!(cube.material.ambient == 0.1);
    assert!(cube.material.diffuse == 0.9);
    assert!(cube.material.specular == 0.9);
    assert!(cube.material.shininess == 200.0);
  }

  #[test]
  fn ray_misses_cube() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cube = Cube::new(transform, material);

    let ray = Ray::new(&Point::new(-2.0, 0.0, 0.0), &Vector::new(0.2673, 0.5345, 0.8018));

    let intersections = cube.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);

    let ray = Ray::new(&Point::new(0.0, -2.0, 0.0), &Vector::new(0.8018, 0.2673, 0.5345));

    let intersections = cube.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);

    let ray = Ray::new(&Point::new(0.0, 0.0, -2.0), &Vector::new(0.5345, 0.8018, 0.2673));

    let intersections = cube.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);

    let ray = Ray::new(&Point::new(2.0, 0.0, 2.0), &Vector::new(0.0, 0.0, -1.0));

    let intersections = cube.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);

    let ray = Ray::new(&Point::new(0.0, 2.0, 2.0), &Vector::new(0.0, -1.0, 0.0));

    let intersections = cube.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);

    let ray = Ray::new(&Point::new(2.0, 2.0, 0.0), &Vector::new(-1.0, 0.0, 0.0));

    let intersections = cube.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 0);
  }

  #[test]
  fn ray_intersects_cube() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cube = Cube::new(transform, material);

    let ray = Ray::new(&Point::new(5.0, 0.5, 0.0), &Vector::new(-1.0, 0.0, 0.0));

    let intersections = cube.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 4.0);
    assert!(intersections[1].t == 6.0);

    let ray = Ray::new(&Point::new(-5.0, 0.5, 0.0), &Vector::new(1.0, 0.0, 0.0));

    let intersections = cube.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 4.0);
    assert!(intersections[1].t == 6.0);

    let ray = Ray::new(&Point::new(0.5, 5.0, 0.0), &Vector::new(0.0, -1.0, 0.0));

    let intersections = cube.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 4.0);
    assert!(intersections[1].t == 6.0);

    let ray = Ray::new(&Point::new(0.5, -5.0, 0.0), &Vector::new(0.0, 1.0, 0.0));

    let intersections = cube.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 4.0);
    assert!(intersections[1].t == 6.0);

    let ray = Ray::new(&Point::new(0.5, 0.0, 5.0), &Vector::new(0.0, 0.0, -1.0));

    let intersections = cube.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 4.0);
    assert!(intersections[1].t == 6.0);

    let ray = Ray::new(&Point::new(0.5, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = cube.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == 4.0);
    assert!(intersections[1].t == 6.0);

    let ray = Ray::new(&Point::new(0.0, 0.5, 0.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = cube.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert!(intersections.len() == 2);
    assert!(intersections[0].t == -1.0);
    assert!(intersections[1].t == 1.0);
  }

  #[test]
  fn normal_on_cube() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cube = Cube::new(transform, material);

    let normal = cube.normal_at(&Point::new(1.0, 0.5, -0.8));

    assert_eq!(normal.get_quad(), (1.0, 0.0, 0.0, 0.0));

    let normal = cube.normal_at(&Point::new(-1.0, -0.2, -0.9));

    assert_eq!(normal.get_quad(), (-1.0, 0.0, 0.0, 0.0));

    let normal = cube.normal_at(&Point::new(-0.4, 1.0, -0.1));

    assert_eq!(normal.get_quad(), (0.0, 1.0, 0.0, 0.0));

    let normal = cube.normal_at(&Point::new(0.3, -1.0, -0.7));

    assert_eq!(normal.get_quad(), (0.0, -1.0, 0.0, 0.0));

    let normal = cube.normal_at(&Point::new(-0.6, 0.3, 1.0));

    assert_eq!(normal.get_quad(), (0.0, 0.0, 1.0, 0.0));

    let normal = cube.normal_at(&Point::new(0.4, 0.4, -1.0));

    assert_eq!(normal.get_quad(), (0.0, 0.0, -1.0, 0.0));

    let normal = cube.normal_at(&Point::new(1.0, 1.0, 1.0));

    assert_eq!(normal.get_quad(), (1.0, 0.0, 0.0, 0.0));

    let normal = cube.normal_at(&Point::new(-1.0, -1.0, -1.0));

    assert_eq!(normal.get_quad(), (-1.0, 0.0, 0.0, 0.0));
  }
}
