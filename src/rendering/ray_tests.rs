#[cfg(test)]
mod tests {
  use crate::rendering::Ray;

  use crate::math::tuple::Tuple;
  use crate::math::Point;
  use crate::math::Vector;

  use crate::math::Matrix4x4;

  #[test]
  fn new_sets_values() {
    let ray = Ray::new(&Point::new(1.0, 2.0, 3.0), &Vector::new(4.0, 5.0, 6.0));

    let (ray_origin_x, ray_origin_y, ray_origin_z, _) = ray.origin.get_quad();

    assert_eq!(ray_origin_x, 1.0);
    assert_eq!(ray_origin_y, 2.0);
    assert_eq!(ray_origin_z, 3.0);

    let (ray_direction_x, ray_direction_y, ray_direction_z, _) = ray.direction.get_quad();

    assert_eq!(ray_direction_x, 4.0);
    assert_eq!(ray_direction_y, 5.0);
    assert_eq!(ray_direction_z, 6.0);
  }

  #[test]
  fn computes_position_from_ray_and_distance() {
    let ray = Ray::new(&Point::new(2.0, 3.0, 4.0), &Vector::new(1.0, 0.0, 0.0));

    let position_1 = ray.position(0.0);

    let (position_1_x, position_1_y, position_1_z, _) = position_1.get_quad();

    assert_eq!(position_1_x, 2.0);
    assert_eq!(position_1_y, 3.0);
    assert_eq!(position_1_z, 4.0);

    let position_2 = ray.position(1.0);

    let (position_2_x, position_2_y, position_2_z, _) = position_2.get_quad();

    assert_eq!(position_2_x, 3.0);
    assert_eq!(position_2_y, 3.0);
    assert_eq!(position_2_z, 4.0);

    let position_3 = ray.position(-1.0);

    let (position_3_x, position_3_y, position_3_z, _) = position_3.get_quad();

    assert_eq!(position_3_x, 1.0);
    assert_eq!(position_3_y, 3.0);
    assert_eq!(position_3_z, 4.0);

    let position_4 = ray.position(2.5);

    let (position_4_x, position_4_y, position_4_z, _) = position_4.get_quad();

    assert_eq!(position_4_x, 4.5);
    assert_eq!(position_4_y, 3.0);
    assert_eq!(position_4_z, 4.0);
  }

  #[test]
  fn translating_ray() {
    let mut ray = Ray::new(&Point::new(1.0, 2.0, 3.0), &Vector::new(0.0, 1.0, 0.0));

    let translation = Matrix4x4::translate(3.0, 4.0, 5.0);

    let transformed_ray = ray.transform(&translation);

    assert!(transformed_ray.origin.get_quad() == (4.0, 6.0, 8.0, 1.0));
    assert!(transformed_ray.direction.get_quad() == (0.0, 1.0, 0.0, 0.0));
  }

  #[test]
  fn scaling_ray() {
    let mut ray = Ray::new(&Point::new(1.0, 2.0, 3.0), &Vector::new(0.0, 1.0, 0.0));

    let scale = Matrix4x4::scale(2.0, 3.0, 4.0);

    let transformed_ray = ray.transform(&scale);

    assert!(transformed_ray.origin.get_quad() == (2.0, 6.0, 12.0, 1.0));
    assert!(transformed_ray.direction.get_quad() == (0.0, 3.0, 0.0, 0.0));
  }
}
