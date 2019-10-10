#[cfg(test)]
mod tests {
  use crate::rendering::math::Point;
  use crate::rendering::math::Vector;
  
  use crate::rendering::math::Color;

  use crate::rendering::math::Matrix4x4;

  use crate::rendering::shapes::shape::Shape;
  use crate::rendering::shapes::Cube;
  use crate::rendering::shapes::Sphere;

  use crate::rendering::Container;

  use crate::rendering::Ray;
  
  use crate::rendering::Material;

  #[test]
  fn container_created_with_transform() {
    let transform = Matrix4x4::translate(5.0, -3.0, 2.0);
    let container = Container::new(transform, Vec::new());

    assert!(container.transform == Matrix4x4::translate(5.0, -3.0, 2.0));
  }

  #[test]
  fn container_stores_references_to_child_shapes() {
    let transform = Matrix4x4::translate(-1.0, -1.0, -1.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = &Sphere::new(transform, material);

    let transform = Matrix4x4::translate(1.0, 1.0, 1.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cube = &Cube::new(transform, material);

    let transform = Matrix4x4::identity();
    let container = Container::new(transform, vec![sphere as &dyn Shape, cube as &dyn Shape]);

    assert!((container.shapes[0] as &dyn Shape).is_eq(sphere as &dyn Shape));
    assert!((container.shapes[1] as &dyn Shape).is_eq(cube as &dyn Shape));
  }

  #[test]
  fn intersecting_empty_container_with_a_ray() {
    let transform = Matrix4x4::identity();
    let container = Container::new(transform, Vec::new());

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = container.intersect(&ray);

    assert!(intersections.len() == 0);
  }

  #[test]
  fn intersecting_non_empty_container_at_origin() {
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::translate(0.0, 0.0, -3.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let transform = Matrix4x4::translate(5.0, 0.0, 0.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_3 = &Sphere::new(transform, material);

    let transform = Matrix4x4::identity();
    let shapes = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape, sphere_3 as &dyn Shape];
    let container = Container::new(transform, shapes);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = container.intersect(&ray);

    assert!(intersections.len() == 4);
    assert!((intersections[0].object).is_eq(sphere_2 as &dyn Shape));
    assert!((intersections[1].object).is_eq(sphere_2 as &dyn Shape));
    assert!((intersections[2].object).is_eq(sphere_1 as &dyn Shape));
    assert!((intersections[3].object).is_eq(sphere_1 as &dyn Shape));
  }

  #[test]
  fn intersecting_non_empty_transformed_container() {
    let transform = Matrix4x4::translate(5.0, 0.0, 0.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = &Sphere::new(transform, material);

    let transform_2 = Matrix4x4::scale(2.0, 2.0, 2.0);
    let shapes = vec![sphere as &dyn Shape];
    let container = Container::new(transform_2, shapes);

    let ray = Ray::new(&Point::new(10.0, 0.0, -10.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = container.intersect(&ray);

    assert!(intersections.len() == 2);
  }
}
