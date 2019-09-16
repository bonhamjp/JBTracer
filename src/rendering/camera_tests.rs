#[cfg(test)]
mod tests {
  use std::f64;

  use crate::rendering::Camera;

  use crate::math::tuple::Tuple;
  use crate::math::Point;
  use crate::math::Vector;

  use crate::math::Matrix4x4;

  #[test]
  fn camera_is_created_with_specified_dimensions_and_fov() {
    let camera = Camera::new(160, 120, f64::consts::PI / 2.0, Matrix4x4::identity());

    assert_eq!(camera.horizontal_size, 160);
    assert_eq!(camera.vertical_size, 120);
    assert_eq!(camera.field_of_view, f64::consts::PI / 2.0);
  }

  #[test]
  fn calculates_pixel_size_for_wider_dimensions() {
    let camera = Camera::new(200, 125, f64::consts::PI / 2.0, Matrix4x4::identity());

    assert_eq!(camera.pixel_size, 0.009999999999999998);
  }

  #[test]
  fn calculates_pixel_size_for_tall_dimensions() {
    let camera = Camera::new(125, 200, f64::consts::PI / 2.0, Matrix4x4::identity());

    assert_eq!(camera.pixel_size, 0.009999999999999998);
  }

  #[test]
  fn ray_through_the_center_of_a_canvas() {
    let camera = Camera::new(201, 101, f64::consts::PI / 2.0, Matrix4x4::identity());

    let ray = camera.ray_for_pixel(100, 50);

    assert!((ray.origin.x - 0.0).abs() < 0.0001);
    assert!((ray.origin.y - 0.0).abs() < 0.0001);
    assert!((ray.origin.z - 0.0).abs() < 0.0001);
    assert!((ray.direction.x - 0.0).abs() < 0.0001);
    assert!((ray.direction.y - 0.0).abs() < 0.0001);
    assert!((ray.direction.z - -1.0).abs() < 0.0001);
  }

  #[test]
  fn ray_through_corner_of_a_canvas() {
    let camera = Camera::new(201, 101, f64::consts::PI / 2.0, Matrix4x4::identity());

    let ray = camera.ray_for_pixel(0, 0);

    assert!((ray.origin.x - 0.0).abs() < 0.0001);
    assert!((ray.origin.y - 0.0).abs() < 0.0001);
    assert!((ray.origin.z - 0.0).abs() < 0.0001);
    assert!((ray.direction.x - 0.66519).abs() < 0.0001);
    assert!((ray.direction.y - 0.33259).abs() < 0.0001);
    assert!((ray.direction.z - -0.66851).abs() < 0.0001);
  }

  #[test]
  fn ray_for_transformed_camera() {
    let camera_transform = Matrix4x4::rotate_y(f64::consts::PI / 4.0).mult4x4(&Matrix4x4::translate(0.0, -2.0, 5.0));
    let camera = Camera::new(201, 101, f64::consts::PI / 2.0, camera_transform);

    let ray = camera.ray_for_pixel(100, 50);

    assert!((ray.origin.x - 0.0).abs() < 0.0001);
    assert!((ray.origin.y - 2.0).abs() < 0.0001);
    assert!((ray.origin.z - -5.0).abs() < 0.0001);
    assert!((ray.direction.x - ((2.0 as f64).sqrt() / 2.0)).abs() < 0.0001);
    assert!((ray.direction.y - 0.0).abs() < 0.0001);
    assert!((ray.direction.z - (-(2.0 as f64).sqrt() / 2.0)).abs() < 0.0001);
  }
}
