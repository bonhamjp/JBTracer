#[cfg(test)]
mod tests {
  use std::f64;

  use crate::rendering::Canvas;

  use crate::rendering::Scene;

  use crate::rendering::Camera;

  use crate::rendering::PointLight;

  use crate::rendering::shape::Shape;
  use crate::rendering::Sphere;

  use crate::rendering::Material;

  use crate::rendering::Ray;
  use crate::rendering::Intersection;
  use crate::rendering::Computations;

  use crate::math::tuple::Tuple;
  use crate::math::Point;
  use crate::math::Vector;

  use crate::math::Color;

  use crate::math::Matrix4x4;

  #[test]
  fn empty_scene_contains_no_lights_or_objects() {
    let scene = Scene::empty();

    assert!(scene.lights.len() == 0);
    assert!(scene.objects.len() == 0);
  }

  #[test]
  fn creating_scene_with_lights_and_objects() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());
    
    let sphere = Sphere::default();

    let scene = Scene::new(camera, vec![PointLight::default()], vec![&sphere]);

    assert!(scene.lights.len() == 1);
    assert!(scene.lights[0] == PointLight::default());
    assert!(scene.objects.len() == 1);
    assert!(scene.objects[0].is_eq(&sphere));
  }

  #[test]
  fn intersecting_a_scene_with_a_ray() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let sphere_1 = &Sphere::new(Matrix4x4::identity(), Material::new(Color::new(0.8, 1.0, 0.6, 1.0), 0.1, 0.7, 0.2, 200.0));
    let sphere_2 = &Sphere::new(Matrix4x4::scale(0.5, 0.5, 0.5), Material::default());

    let scene = Scene::new(camera, vec![light], vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape]);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersections = scene.intersect(&ray);

    assert!(intersections.len() == 4);
    assert!(intersections[0].t == 4.0);
    assert!(intersections[1].t == 4.5);
    assert!(intersections[2].t == 5.5);
    assert!(intersections[3].t == 6.0);
  }

  #[test]
  fn shading_an_intersection_that_occurs_on_outside_of_object() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let sphere_1 = &Sphere::new(Matrix4x4::identity(), Material::new(Color::new(0.8, 1.0, 0.6, 1.0), 0.1, 0.7, 0.2, 200.0));
    let sphere_2 = &Sphere::new(Matrix4x4::scale(0.5, 0.5, 0.5), Material::default());

    let scene = Scene::new(camera, vec![light], vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape]);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersection = Intersection::new(4.0, scene.objects[0]);

    let computations = Computations::new(&intersection, &ray);

    let shaded_color = scene.shade_hit(&computations);
    
    assert_eq!(shaded_color.r, 0.38066119308103435);
    assert_eq!(shaded_color.g, 0.47582649135129296);
    assert_eq!(shaded_color.b, 0.28549589481077575);
    assert_eq!(shaded_color.a, 1.0);
  }

  #[test]
  fn shading_an_intersection_that_occurs_on_inside_of_object() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(0.0, 0.25, 0.0)); 

    let sphere_1 = &Sphere::new(Matrix4x4::identity(), Material::new(Color::new(0.8, 1.0, 0.6, 1.0), 0.1, 0.7, 0.2, 200.0));
    let sphere_2 = &Sphere::new(Matrix4x4::scale(0.5, 0.5, 0.5), Material::default());

    let scene = Scene::new(camera, vec![light], vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape]);

    let ray = Ray::new(&Point::new(0.0, 0.0, 0.0), &Vector::new(0.0, 0.0, 1.0));

    let intersection = Intersection::new(0.5, scene.objects[1]);

    let computations = Computations::new(&intersection, &ray);

    let shaded_color = scene.shade_hit(&computations);
    
    assert_eq!(shaded_color.r, 0.9049844720832575);
    assert_eq!(shaded_color.g, 0.9049844720832575);
    assert_eq!(shaded_color.b, 0.9049844720832575);
    assert_eq!(shaded_color.a, 1.0);
  }

  #[test]
  fn shading_when_intersection_misses() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());
    
    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let sphere_1 = &Sphere::new(Matrix4x4::identity(), Material::new(Color::new(0.8, 1.0, 0.6, 1.0), 0.1, 0.7, 0.2, 200.0));
    let sphere_2 = &Sphere::new(Matrix4x4::scale(0.5, 0.5, 0.5), Material::default());

    let scene = Scene::new(camera, vec![light], vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape]);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 1.0, 0.0));

    let color = scene.color_at(&ray);

    assert_eq!(color.r, 0.0);
    assert_eq!(color.g, 0.0);
    assert_eq!(color.b, 0.0);
    assert_eq!(color.a, 1.0);
  }

  #[test]
  fn shading_when_intersection_hits() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());
    
    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let sphere_1 = &Sphere::new(Matrix4x4::identity(), Material::new(Color::new(0.8, 1.0, 0.6, 1.0), 0.1, 0.7, 0.2, 200.0));
    let sphere_2 = &Sphere::new(Matrix4x4::scale(0.5, 0.5, 0.5), Material::default());

    let scene = Scene::new(camera, vec![light], vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape]);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let color = scene.color_at(&ray);

    assert_eq!(color.r, 0.38066119308103435);
    assert_eq!(color.g, 0.47582649135129296);
    assert_eq!(color.b, 0.28549589481077575);
    assert_eq!(color.a, 1.0);
  }

  #[test]
  fn shading_when_intersection_behind_ray() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let sphere_1 = &Sphere::new(Matrix4x4::identity(), Material::new(Color::new(0.8, 1.0, 0.6, 1.0), 1.0, 0.7, 0.2, 200.0));
    let sphere_2 = &Sphere::new(Matrix4x4::scale(0.5, 0.5, 0.5), Material::new(Color::new(0.8, 1.0, 0.6, 1.0), 1.0, 0.1, 0.1, 100.0));

    let scene = Scene::new(camera, vec![light], vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape]);

    let ray = Ray::new(&Point::new(0.0, 0.0, 0.75), &Vector::new(0.0, 0.0, -1.0));

    let color = scene.color_at(&ray);

    assert_eq!(color.r, 0.8);
    assert_eq!(color.g, 1.0);
    assert_eq!(color.b, 0.6);
    assert_eq!(color.a, 1.0);
  }

  #[test]
  fn rendering_scene_through_camera() {
    let from = Point::new(0.0, 0.0, -5.0);
    let to = Point::new(0.0, 0.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);
    let camera_transform = Matrix4x4::view_transform(&from, &to, &up);
    let camera = Camera::new(11, 11, f64::consts::PI / 2.0, camera_transform);

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let sphere_1 = &Sphere::new(Matrix4x4::identity(), Material::new(Color::new(0.8, 1.0, 0.6, 1.0), 0.1, 0.7, 0.2, 200.0));
    let sphere_2 = &Sphere::new(Matrix4x4::scale(0.5, 0.5, 0.5), Material::default());

    let scene = Scene::new(camera, vec![light], vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape]);

    let canvas = scene.render();

    let buffer_color = canvas.pixel_color(5, 5);

    assert_eq!(buffer_color.r, 0.38066119308103435);
    assert_eq!(buffer_color.g, 0.47582649135129296);
    assert_eq!(buffer_color.b, 0.28549589481077575);
    assert_eq!(buffer_color.a, 1.0);
  }

  #[test]
  fn no_shadow_when_nothing_collinear_with_point_and_light() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let sphere_1 = &Sphere::new(Matrix4x4::identity(), Material::new(Color::new(0.8, 1.0, 0.6, 1.0), 0.1, 0.7, 0.2, 200.0));
    let sphere_2 = &Sphere::new(Matrix4x4::scale(0.5, 0.5, 0.5), Material::default());

    let scene = Scene::new(camera, vec![light], vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape]);

    let point = Point::new(0.0, 10.0, 0.0);

    assert!(!scene.is_shadowed(&point, &Point::new(-10.0, 10.0, -10.0)));
  }

  #[test]
  fn shadow_when_object_is_between_point_and_light() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let sphere_1 = &Sphere::new(Matrix4x4::identity(), Material::new(Color::new(0.8, 1.0, 0.6, 1.0), 0.1, 0.7, 0.2, 200.0));
    let sphere_2 = &Sphere::new(Matrix4x4::scale(0.5, 0.5, 0.5), Material::default());

    let scene = Scene::new(camera, vec![light], vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape]);

    let point = Point::new(10.0, -10.0, 10.0);

    assert!(scene.is_shadowed(&point, &Point::new(-10.0, 10.0, -10.0)));
  }

  #[test]
  fn now_shadow_when_object_is_behind_light() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let sphere_1 = &Sphere::new(Matrix4x4::identity(), Material::new(Color::new(0.8, 1.0, 0.6, 1.0), 0.1, 0.7, 0.2, 200.0));
    let sphere_2 = &Sphere::new(Matrix4x4::scale(0.5, 0.5, 0.5), Material::default());

    let scene = Scene::new(camera, vec![light], vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape]);

    let point = Point::new(-20.0, 20.0, -20.0);

    assert!(!scene.is_shadowed(&point, &Point::new(-10.0, 10.0, -10.0)));
  }

  #[test]
  fn now_shadow_when_object_is_behind_point() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let sphere_1 = &Sphere::new(Matrix4x4::identity(), Material::new(Color::new(0.8, 1.0, 0.6, 1.0), 0.1, 0.7, 0.2, 200.0));
    let sphere_2 = &Sphere::new(Matrix4x4::scale(0.5, 0.5, 0.5), Material::default());

    let scene = Scene::new(camera, vec![light], vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape]);

    let point = Point::new(-2.0, 2.0, -2.0);

    assert!(!scene.is_shadowed(&point, &Point::new(-10.0, 10.0, -10.0)));
  }

  #[test]
  fn shade_hit_only_calculates_ambient_contribution_when_in_shadow() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0)); 

    let sphere_1 = &Sphere::new(Matrix4x4::identity(), Material::new(Color::new(0.8, 1.0, 0.6, 1.0), 0.1, 0.7, 0.2, 200.0));
    let sphere_2 = &Sphere::new(Matrix4x4::translate(0.0, 0.0, 10.0), Material::default());

    let scene = Scene::new(camera, vec![light], vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape]);

    let ray = Ray::new(&Point::new(0.0, 0.0, 5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersection = Intersection::new(4.0, scene.objects[1]);

    let computations = Computations::new(&intersection, &ray);

    let shaded_color = scene.shade_hit(&computations);
    
    assert_eq!(shaded_color.r, 0.1);
    assert_eq!(shaded_color.g, 0.1);
    assert_eq!(shaded_color.b, 0.1);
    assert_eq!(shaded_color.a, 1.0);
  }
}
