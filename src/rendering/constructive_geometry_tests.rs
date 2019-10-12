#[cfg(test)]
mod tests {
  use crate::rendering::math::Point;
  use crate::rendering::math::Vector;
  
  use crate::rendering::math::Color;

  use crate::rendering::math::Matrix4x4;

  use crate::rendering::shapes::shape::Shape;
  use crate::rendering::shapes::Sphere;
  use crate::rendering::shapes::Cube;

  use crate::rendering::ConstructiveGeometry;
  use crate::rendering::ConstructiveOperation;

  use crate::rendering::Ray;
  use crate::rendering::Intersection;
  
  use crate::rendering::Material;

  #[test]
  fn contructive_geometry_created_with_transform_and_material_and_left_and_right_shapes() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(Matrix4x4::identity(), material);
    
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cube = Cube::new(Matrix4x4::identity(), material);

    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let constructive_geometry = ConstructiveGeometry::new(Matrix4x4::identity(), material, &sphere, &cube, ConstructiveOperation::Union);

    assert!(constructive_geometry.transform == Matrix4x4::identity());
    assert!(constructive_geometry.material.ambient == 0.1);
    assert!(constructive_geometry.material.diffuse == 0.9);
    assert!(constructive_geometry.material.specular == 0.9);
    assert!(constructive_geometry.material.shininess == 200.0);
    assert_eq!(constructive_geometry.left_side.is_eq(&sphere), true);
    assert_eq!(constructive_geometry.right_side.is_eq(&cube), true);
    assert!(constructive_geometry.operation == ConstructiveOperation::Union);
  }

  #[test]
  fn checking_if_intersection_allowed_for_union() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(Matrix4x4::identity(), material);
    
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cube = Cube::new(Matrix4x4::identity(), material);

    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let constructive_geometry = ConstructiveGeometry::new(Matrix4x4::identity(), material, &sphere, &cube, ConstructiveOperation::Union);

    assert_eq!(constructive_geometry.intersection_allowed(true, true, true), false);
    assert_eq!(constructive_geometry.intersection_allowed(true, true, false), true);
    assert_eq!(constructive_geometry.intersection_allowed(true, false, true), false);
    assert_eq!(constructive_geometry.intersection_allowed(true, false, false), true);
    assert_eq!(constructive_geometry.intersection_allowed(false, true, true), false);
    assert_eq!(constructive_geometry.intersection_allowed(false, true, false), false);
    assert_eq!(constructive_geometry.intersection_allowed(false, false, true), true);
    assert_eq!(constructive_geometry.intersection_allowed(false, false, false), true);
  }

  #[test]
  fn checking_if_intersection_allowed_for_intersection() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(Matrix4x4::identity(), material);
    
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cube = Cube::new(Matrix4x4::identity(), material);

    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let constructive_geometry = ConstructiveGeometry::new(Matrix4x4::identity(), material, &sphere, &cube, ConstructiveOperation::Intersection);

    assert_eq!(constructive_geometry.intersection_allowed(true, true, true), true);
    assert_eq!(constructive_geometry.intersection_allowed(true, true, false), false);
    assert_eq!(constructive_geometry.intersection_allowed(true, false, true), true);
    assert_eq!(constructive_geometry.intersection_allowed(true, false, false), false);
    assert_eq!(constructive_geometry.intersection_allowed(false, true, true), true);
    assert_eq!(constructive_geometry.intersection_allowed(false, true, false), true);
    assert_eq!(constructive_geometry.intersection_allowed(false, false, true), false);
    assert_eq!(constructive_geometry.intersection_allowed(false, false, false), false);
  }

  #[test]
  fn checking_if_intersection_allowed_for_difference() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(Matrix4x4::identity(), material);
    
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cube = Cube::new(Matrix4x4::identity(), material);

    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let constructive_geometry = ConstructiveGeometry::new(Matrix4x4::identity(), material, &sphere, &cube, ConstructiveOperation::Difference);

    assert_eq!(constructive_geometry.intersection_allowed(true, true, true), false);
    assert_eq!(constructive_geometry.intersection_allowed(true, true, false), true);
    assert_eq!(constructive_geometry.intersection_allowed(true, false, true), false);
    assert_eq!(constructive_geometry.intersection_allowed(true, false, false), true);
    assert_eq!(constructive_geometry.intersection_allowed(false, true, true), true);
    assert_eq!(constructive_geometry.intersection_allowed(false, true, false), true);
    assert_eq!(constructive_geometry.intersection_allowed(false, false, true), false);
    assert_eq!(constructive_geometry.intersection_allowed(false, false, false), false);
  }

  #[test]
  fn filtering_union_intersections() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(Matrix4x4::identity(), material);
    
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cube = Cube::new(Matrix4x4::identity(), material);

    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let constructive_geometry = ConstructiveGeometry::new(Matrix4x4::identity(), material, &sphere, &cube, ConstructiveOperation::Union);

    let intersection_1 = Intersection::new(1.0, constructive_geometry.left_side, Matrix4x4::identity(), Matrix4x4::identity());
    let intersection_2 = Intersection::new(2.0, constructive_geometry.right_side, Matrix4x4::identity(), Matrix4x4::identity());
    let intersection_3 = Intersection::new(3.0, constructive_geometry.left_side, Matrix4x4::identity(), Matrix4x4::identity());
    let intersection_4 = Intersection::new(4.0, constructive_geometry.right_side, Matrix4x4::identity(), Matrix4x4::identity());
    let intersections = vec![intersection_1, intersection_2, intersection_3, intersection_4];

    let filtered_intersections = constructive_geometry.filter_intersections(intersections);

    assert_eq!(filtered_intersections.len(), 2);

    assert_eq!(filtered_intersections[0].t, 1.0);
    assert_eq!(filtered_intersections[1].t, 4.0);
  }

  #[test]
  fn filtering_intersection_intersections() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(Matrix4x4::identity(), material);
    
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cube = Cube::new(Matrix4x4::identity(), material);

    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let constructive_geometry = ConstructiveGeometry::new(Matrix4x4::identity(), material, &sphere, &cube, ConstructiveOperation::Intersection);

    let intersection_1 = Intersection::new(1.0, constructive_geometry.left_side, Matrix4x4::identity(), Matrix4x4::identity());
    let intersection_2 = Intersection::new(2.0, constructive_geometry.right_side, Matrix4x4::identity(), Matrix4x4::identity());
    let intersection_3 = Intersection::new(3.0, constructive_geometry.left_side, Matrix4x4::identity(), Matrix4x4::identity());
    let intersection_4 = Intersection::new(4.0, constructive_geometry.right_side, Matrix4x4::identity(), Matrix4x4::identity());
    let intersections = vec![intersection_1, intersection_2, intersection_3, intersection_4];

    let filtered_intersections = constructive_geometry.filter_intersections(intersections);

    assert_eq!(filtered_intersections.len(), 2);

    assert_eq!(filtered_intersections[0].t, 2.0);
    assert_eq!(filtered_intersections[1].t, 3.0);
  }

  #[test]
  fn filtering_difference_intersections() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(Matrix4x4::identity(), material);
    
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cube = Cube::new(Matrix4x4::identity(), material);

    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let constructive_geometry = ConstructiveGeometry::new(Matrix4x4::identity(), material, &sphere, &cube, ConstructiveOperation::Difference);

    let intersection_1 = Intersection::new(1.0, constructive_geometry.left_side, Matrix4x4::identity(), Matrix4x4::identity());
    let intersection_2 = Intersection::new(2.0, constructive_geometry.right_side, Matrix4x4::identity(), Matrix4x4::identity());
    let intersection_3 = Intersection::new(3.0, constructive_geometry.left_side, Matrix4x4::identity(), Matrix4x4::identity());
    let intersection_4 = Intersection::new(4.0, constructive_geometry.right_side, Matrix4x4::identity(), Matrix4x4::identity());
    let intersections = vec![intersection_1, intersection_2, intersection_3, intersection_4];

    let filtered_intersections = constructive_geometry.filter_intersections(intersections);

    assert_eq!(filtered_intersections.len(), 2);

    assert_eq!(filtered_intersections[0].t, 1.0);
    assert_eq!(filtered_intersections[1].t, 2.0);
  }

  #[test]
  fn ray_misses_constructive_geometry_object() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(Matrix4x4::identity(), material);
    
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cube = Cube::new(Matrix4x4::translate(0.0, 0.0, 0.5), material);

    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let constructive_geometry = ConstructiveGeometry::new(Matrix4x4::identity(), material, &sphere, &cube, ConstructiveOperation::Union);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = constructive_geometry.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert_eq!(intersections.len(), 2);

    assert_eq!(intersections[0].t, 4.0);
    assert!(intersections[0].object.is_eq(constructive_geometry.left_side));
    assert_eq!(intersections[1].t, 6.5);
    assert!(intersections[1].object.is_eq(constructive_geometry.right_side));
  }

  #[test]
  fn ray_hits_constructive_geometry_object() {
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = Sphere::new(Matrix4x4::identity(), material);
    
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let cube = Cube::new(Matrix4x4::identity(), material);

    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let constructive_geometry = ConstructiveGeometry::new(Matrix4x4::identity(), material, &sphere, &cube, ConstructiveOperation::Union);

    let ray = Ray::new(&Point::new(0.0, 2.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = constructive_geometry.intersections(&ray, Matrix4x4::identity(), Matrix4x4::identity());

    assert_eq!(intersections.len(), 0);
  }
}
