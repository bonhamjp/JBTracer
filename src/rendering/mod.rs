pub mod shape;

pub mod canvas;
pub mod canvas_tests;
pub use self::canvas::Canvas;

pub mod ray;
pub mod ray_tests;
pub use self::ray::Ray;

pub mod intersection;
pub mod intersection_tests;
pub use self::intersection::Intersection;

pub mod sphere;
pub mod sphere_tests;
pub use self::sphere::Sphere;
