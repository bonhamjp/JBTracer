#[cfg(test)]
mod tests {
  use std::f64;

  use crate::rendering::math::Point;
  use crate::rendering::math::Vector;

  use crate::rendering::math::Color;

  use crate::rendering::math::Matrix4x4;

  use crate::rendering::Scene;

  use crate::rendering::Camera;

  use crate::rendering::PointLight;

  use crate::rendering::shapes::shape::Shape;
  use crate::rendering::shapes::Sphere;
  use crate::rendering::shapes::Plane;

  use crate::rendering::Container;

  use crate::rendering::Material;

  use crate::rendering::Ray;
  use crate::rendering::Intersection;
  use crate::rendering::Computations;

  #[test]
  fn creating_scene_with_lights_and_objects() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());
    
    let transform = Matrix4x4::translate(5.0, -3.0, 2.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = &Sphere::new(transform, material);

    let container_objects = vec![sphere as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![PointLight::default()], vec![container]);

    assert!(scene.lights.len() == 1);
    assert!(scene.lights[0] == PointLight::default());
    assert!(scene.containers.len() == 1);
    assert!((scene.containers[0].shapes[0] as &dyn Shape).is_eq(sphere as &dyn Shape));
  }

  #[test]
  fn intersecting_a_scene_with_a_ray() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

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

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersection = Intersection::new(4.0, scene.containers[0].shapes[0], Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    let shaded_color = scene.shade_hit(&computations, 4);
    
    assert_eq!(shaded_color.r, 0.38066119308103435);
    assert_eq!(shaded_color.g, 0.47582649135129296);
    assert_eq!(shaded_color.b, 0.28549589481077575);
    assert_eq!(shaded_color.a, 1.0);
  }

  #[test]
  fn shading_an_intersection_that_occurs_on_inside_of_object() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(0.0, 0.25, 0.0)); 

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.0, 0.0), &Vector::new(0.0, 0.0, 1.0));

    let intersection = Intersection::new(0.5, scene.containers[0].shapes[1], Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    let shaded_color = scene.shade_hit(&computations, 4);
    
    assert_eq!(shaded_color.r, 0.9049844720832575);
    assert_eq!(shaded_color.g, 0.9049844720832575);
    assert_eq!(shaded_color.b, 0.9049844720832575);
    assert_eq!(shaded_color.a, 1.0);
  }

  #[test]
  fn shading_when_intersection_misses() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());
    
    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 1.0, 0.0));

    let color = scene.color_at(&ray, 4);

    assert_eq!(color.r, 0.0);
    assert_eq!(color.g, 0.0);
    assert_eq!(color.b, 0.0);
    assert_eq!(color.a, 1.0);
  }

  #[test]
  fn shading_when_intersection_hits() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());
    
    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let color = scene.color_at(&ray, 4);

    assert_eq!(color.r, 0.38066119308103435);
    assert_eq!(color.g, 0.47582649135129296);
    assert_eq!(color.b, 0.28549589481077575);
    assert_eq!(color.a, 1.0);
  }

  #[test]
  fn shading_when_intersection_behind_ray() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(1.0, 0.1, 0.1, 100.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.0, 0.75), &Vector::new(0.0, 0.0, -1.0));

    let color = scene.color_at(&ray, 4);

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

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

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

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let point = Point::new(0.0, 10.0, 0.0);

    assert!(!scene.is_shadowed(&point, &Point::new(-10.0, 10.0, -10.0)));
  }

  #[test]
  fn shadow_when_object_is_between_point_and_light() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let point = Point::new(10.0, -10.0, 10.0);

    assert!(scene.is_shadowed(&point, &Point::new(-10.0, 10.0, -10.0)));
  }

  #[test]
  fn now_shadow_when_object_is_behind_light() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let point = Point::new(-20.0, 20.0, -20.0);

    assert!(!scene.is_shadowed(&point, &Point::new(-10.0, 10.0, -10.0)));
  }

  #[test]
  fn now_shadow_when_object_is_behind_point() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)); 

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let point = Point::new(-2.0, 2.0, -2.0);

    assert!(!scene.is_shadowed(&point, &Point::new(-10.0, 10.0, -10.0)));
  }

  #[test]
  fn shade_hit_only_calculates_ambient_contribution_when_in_shadow() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.0, 5.0), &Vector::new(0.0, 0.0, 1.0));

    let intersection = Intersection::new(4.0, scene.containers[0].shapes[1], Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    let shaded_color = scene.shade_hit(&computations, 4);
    
    assert_eq!(shaded_color.r, 0.1);
    assert_eq!(shaded_color.g, 0.1);
    assert_eq!(shaded_color.b, 0.1);
    assert_eq!(shaded_color.a, 1.0);
  }

  #[test]
  fn reflected_color_of_a_non_reflective_material() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);
    
    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.0, 0.0), &Vector::new(0.0, 0.0, 1.0));

    let intersection = Intersection::new(1.0, scene.containers[0].shapes[0], Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    let reflected_color = scene.reflected_color(&computations, 4);
    
    assert_eq!(reflected_color.r, 0.0);
    assert_eq!(reflected_color.g, 0.0);
    assert_eq!(reflected_color.b, 0.0);
    assert_eq!(reflected_color.a, 1.0);
  }

  #[test]
  fn reflected_color_of_a_reflective_material() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let transform = Matrix4x4::translate(0.0, -1.0, 0.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.5, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let plane = &Plane::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape, plane as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.0, -3.0), &Vector::new(0.0, -(2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0));

    let intersection = Intersection::new((2.0 as f64).sqrt(), scene.containers[0].shapes[2], Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    let reflected_color = scene.reflected_color(&computations, 4);
    
    assert_eq!(reflected_color.r, 0.19050298661014994);
    assert_eq!(reflected_color.g, 0.2381287332626874);
    assert_eq!(reflected_color.b, 0.14287723995761248);
    assert_eq!(reflected_color.a, 1.0);
  }

  #[test]
  fn shade_hit_factors_in_reflected_color() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let transform = Matrix4x4::translate(0.0, -1.0, 0.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.5, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let plane = &Plane::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape, plane as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.0, -3.0), &Vector::new(0.0, -(2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0));

    let intersection = Intersection::new((2.0 as f64).sqrt(), scene.containers[0].shapes[2], Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);

    let shaded_color = scene.shade_hit(&computations, 4);
    
    assert_eq!(shaded_color.r, 0.8769283755916514);
    assert_eq!(shaded_color.g, 0.9245541222441889);
    assert_eq!(shaded_color.b, 0.8293026289391139);
    assert_eq!(shaded_color.a, 1.0);
  }

  #[test]
  fn infinite_reflection_ends_after_maximum_reflective_rays_cast() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(0.0, 0.0, 0.0));

    let transform = Matrix4x4::translate(0.0, -1.0, 0.0);
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 1.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let plane_1 = &Plane::new(transform, material);

    let transform = Matrix4x4::translate(0.0, 1.0, 0.0);
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 1.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let plane_2 = &Plane::new(transform, material);

    let container_objects = vec![plane_1 as &dyn Shape, plane_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.0, 0.0), &Vector::new(0.0, 1.0, 0.0));

    let _bottom_out_color = scene.color_at(&ray, 4);
  }

  #[test]
  fn reflected_color_returns_black_if_maximum_depth_reached() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(0.0, 0.0, 0.0));

    let transform = Matrix4x4::translate(0.0, -1.0, 0.0);
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.5, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let plane = &Plane::new(transform, material);

    let container_objects = vec![plane as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.0, -3.0), &Vector::new(0.0, -(2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0));

    let intersection = Intersection::new((2.0 as f64).sqrt(), scene.containers[0].shapes[0], Matrix4x4::identity(), Matrix4x4::identity());
    let mut intersections = Vec::new();
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);
    
    let reflected_color = scene.reflected_color(&computations, 0);
    
    assert_eq!(reflected_color.r, 0.0);
    assert_eq!(reflected_color.g, 0.0);
    assert_eq!(reflected_color.b, 0.0);
    assert_eq!(reflected_color.a, 1.0);
  }

  #[test]
  fn refracted_color_of_an_opaque_material() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.8, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);
    
    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let mut intersections = Vec::new();
    let intersection_1 = Intersection::new(4.0, scene.containers[0].shapes[0], Matrix4x4::identity(), Matrix4x4::identity());
    let intersection_2 = Intersection::new(6.0, scene.containers[0].shapes[0], Matrix4x4::identity(), Matrix4x4::identity());
    intersections.push(intersection_1);
    intersections.push(intersection_2);

    let computations = Computations::new(&intersections[0], &ray, &intersections);

    let refracted_color = scene.reflected_color(&computations, 4);
    
    assert_eq!(refracted_color.r, 0.0);
    assert_eq!(refracted_color.g, 0.0);
    assert_eq!(refracted_color.b, 0.0);
    assert_eq!(refracted_color.a, 1.0);
  }

  #[test]
  fn refracted_color_returns_black_if_maximum_depth_reached() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 1.0, 1.5, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.8, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.0, -5.0), &Vector::new(0.0, 0.0, 1.0));

    let mut intersections = Vec::new();
    let intersection_1 = Intersection::new(4.0, scene.containers[0].shapes[0], Matrix4x4::identity(), Matrix4x4::identity());
    let intersection_2 = Intersection::new(6.0, scene.containers[0].shapes[0], Matrix4x4::identity(), Matrix4x4::identity());
    intersections.push(intersection_1);
    intersections.push(intersection_2);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);
    
    let reflected_color = scene.refracted_color(&computations, 0);
    
    assert_eq!(reflected_color.r, 0.0);
    assert_eq!(reflected_color.g, 0.0);
    assert_eq!(reflected_color.b, 0.0);
    assert_eq!(reflected_color.a, 1.0);
  }

  #[test]
  fn refracted_color_with_total_internal_reflection() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));

    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 1.0, 1.5, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(0.1, 0.8, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.0, (2.0 as f64).sqrt() / 2.0), &Vector::new(0.0, 1.0, 0.0));

    let mut intersections = Vec::new();
    let intersection_1 = Intersection::new(-(2.0 as f64).sqrt() / 2.0, scene.containers[0].shapes[0], Matrix4x4::identity(), Matrix4x4::identity());
    let intersection_2 = Intersection::new((2.0 as f64).sqrt() / 2.0, scene.containers[0].shapes[0], Matrix4x4::identity(), Matrix4x4::identity());
    intersections.push(intersection_1);
    intersections.push(intersection_2);
    
    let computations = Computations::new(&intersections[1], &ray, &intersections);
    
    let reflected_color = scene.refracted_color(&computations, 4);

    assert_eq!(reflected_color.r, 0.0);
    assert_eq!(reflected_color.g, 0.0);
    assert_eq!(reflected_color.b, 0.0);
    assert_eq!(reflected_color.a, 1.0);
  }

  #[test]
  fn schlick_calculation_with_total_internal_reflection() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));

    let transform = Matrix4x4::identity();
    let material = Material::solid(1.0, 0.7, 0.2, 200.0, 0.0, 1.0, 1.5, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = &Sphere::new(transform, material);

    let container_objects = vec![sphere as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.0, (2.0 as f64).sqrt() / 2.0), &Vector::new(0.0, 1.0, 0.0));

    let mut intersections = Vec::new();
    let intersection_1 = Intersection::new(-(2.0 as f64).sqrt() / 2.0, scene.containers[0].shapes[0], Matrix4x4::identity(), Matrix4x4::identity());
    let intersection_2 = Intersection::new((2.0 as f64).sqrt() / 2.0, scene.containers[0].shapes[0], Matrix4x4::identity(), Matrix4x4::identity());
    intersections.push(intersection_1);
    intersections.push(intersection_2);
    
    let computations = Computations::new(&intersections[1], &ray, &intersections);
    
    let reflectance = scene.schlick(&computations);

    assert_eq!(reflectance, 1.0);
  }

  #[test]
  fn schlick_calculation_with_perpendicular_angle() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));

    let transform = Matrix4x4::identity();
    let material = Material::solid(1.0, 0.7, 0.2, 200.0, 0.0, 1.0, 1.5, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = &Sphere::new(transform, material);

    let container_objects = vec![sphere as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.0, 0.0), &Vector::new(0.0, 1.0, 0.0));

    let mut intersections = Vec::new();
    let intersection_1 = Intersection::new(-1.0, scene.containers[0].shapes[0], Matrix4x4::identity(), Matrix4x4::identity());
    let intersection_2 = Intersection::new(1.0, scene.containers[0].shapes[0], Matrix4x4::identity(), Matrix4x4::identity());
    intersections.push(intersection_1);
    intersections.push(intersection_2);
    
    let computations = Computations::new(&intersections[1], &ray, &intersections);
    
    let reflectance = scene.schlick(&computations);

    assert_eq!(reflectance, 0.04000000000000001);
  }

  #[test]
  fn schlick_calculation_with_small_angle_and_n2_larger_than_n1() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));

    let transform = Matrix4x4::identity();
    let material = Material::solid(1.0, 0.7, 0.2, 200.0, 0.0, 1.0, 1.5, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere = &Sphere::new(transform, material);

    let container_objects = vec![sphere as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(&Point::new(0.0, 0.99, -2.0), &Vector::new(0.0, 0.0, 1.0));

    let mut intersections = Vec::new();
    let intersection = Intersection::new(1.8589, scene.containers[0].shapes[0], Matrix4x4::identity(), Matrix4x4::identity());
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);
    
    let reflectance = scene.schlick(&computations);

    assert_eq!(reflectance, 0.4887308101221217);
  }

  #[test]
  fn shades_hit_with_transparent_reflective_material() {
    let camera = Camera::new(200, 100, f64::consts::PI / 2.0, Matrix4x4::identity());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));
    
    let transform = Matrix4x4::identity();
    let material = Material::solid(0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0, Color::new(0.8, 1.0, 0.6, 1.0), Matrix4x4::identity());
    let sphere_1 = &Sphere::new(transform, material);

    let transform = Matrix4x4::scale(0.5, 0.5, 0.5);
    let material = Material::solid(1.0, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let sphere_2 = &Sphere::new(transform, material);

    let transform = Matrix4x4::translate(0.0, -3.5, -0.5);
    let material = Material::solid(0.5, 0.9, 0.9, 200.0, 0.0, 0.0, 1.0, Color::new(1.0, 0.0, 0.0, 1.0), Matrix4x4::identity());
    let sphere_3 = &Sphere::new(transform, material);

    let transform = Matrix4x4::translate(0.0, -1.0, 0.0);
    let material = Material::solid(0.1, 0.9, 0.9, 200.0, 0.5, 0.5, 1.5, Color::new(1.0, 1.0, 1.0, 1.0), Matrix4x4::identity());
    let plane = &Plane::new(transform, material);

    let container_objects = vec![sphere_1 as &dyn Shape, sphere_2 as &dyn Shape, sphere_3 as &dyn Shape, plane as &dyn Shape];
    let container = Container::new(Matrix4x4::identity(), container_objects);

    let scene = Scene::new(camera, vec![light], vec![container]);

    let ray = Ray::new(
      &Point::new(0.0, 0.0, -3.0), 
      &Vector::new(0.0, -(2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0)
    );

    let mut intersections = Vec::new();
    let intersection = Intersection::new((2.0 as f64).sqrt(), scene.containers[0].shapes[3], Matrix4x4::identity(), Matrix4x4::identity());
    intersections.push(intersection);
    
    let computations = Computations::new(&intersections[0], &ray, &intersections);
    
    let shaded_color = scene.shade_hit(&computations, 4);

    assert_eq!(shaded_color.r, 0.9339223928751195);
    assert_eq!(shaded_color.g, 0.6964432916998902);
    assert_eq!(shaded_color.b, 0.6924361306125347);
    assert_eq!(shaded_color.a, 1.0);
  }
}
