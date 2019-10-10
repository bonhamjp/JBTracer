#[cfg(test)]
mod tests {
  use crate::rendering::math::Point;
  use crate::rendering::math::Vector;
  
  use crate::rendering::math::Color;

  use crate::rendering::shapes::Triangle;
  use crate::rendering::shapes::SmoothTriangle;

  use crate::rendering::ObjLineType;
  use crate::rendering::ObjFileParser;

  #[test]
  fn ignores_lines_from_obj_file_that_do_not_allowed_pattern() {
    let gibberish_lines = vec![
      "```".to_string(), 
      "Invalid 1".to_string(), 
      "x Another Invalid line".to_string(), 
      "This one is also invalid".to_string(), 
      "```".to_string()
    ];

    assert!(ObjFileParser::obj_line_type(&gibberish_lines[0]) == ObjLineType::Invalid);
    assert!(ObjFileParser::obj_line_type(&gibberish_lines[1]) == ObjLineType::Invalid);
    assert!(ObjFileParser::obj_line_type(&gibberish_lines[2]) == ObjLineType::Invalid);
    assert!(ObjFileParser::obj_line_type(&gibberish_lines[3]) == ObjLineType::Invalid);
    assert!(ObjFileParser::obj_line_type(&gibberish_lines[4]) == ObjLineType::Invalid);
  }

  #[test]
  fn parses_vertex_lines_from_obj_file_into_points() {
    let vertex_lines = vec![
      "v 0.1 0.2 0.3".to_string(), 
      "v 1.1 -1.2 1.3".to_string(), 
      "v -2.1 2.2 -2.3".to_string()
    ];

    assert!(ObjFileParser::obj_line_type(&vertex_lines[0]) == ObjLineType::Vertex);
    assert!(ObjFileParser::parse_vertex(&vertex_lines[0]) == Point::new(0.1, 0.2, 0.3));

    assert!(ObjFileParser::obj_line_type(&vertex_lines[1]) == ObjLineType::Vertex);
    assert!(ObjFileParser::parse_vertex(&vertex_lines[1]) == Point::new(1.1, -1.2, 1.3));

    assert!(ObjFileParser::obj_line_type(&vertex_lines[2]) == ObjLineType::Vertex);
    assert!(ObjFileParser::parse_vertex(&vertex_lines[2]) == Point::new(-2.1, 2.2, -2.3));
  }

  #[test]
  fn parses_vertex_normal_lines_from_obj_file_into_vectors() {
    let vertex_normal_lines = vec![
      "vn 3.1 3.2 3.3".to_string(), 
      "vn 4.1 -4.2 4.3".to_string(), 
      "vn -5.1 5.2 -5.3".to_string()
    ];

    assert!(ObjFileParser::obj_line_type(&vertex_normal_lines[0]) == ObjLineType::VertexNormal);
    assert!(ObjFileParser::parse_vertex_normal(&vertex_normal_lines[0]) == Vector::new(3.1, 3.2, 3.3));

    assert!(ObjFileParser::obj_line_type(&vertex_normal_lines[1]) == ObjLineType::VertexNormal);
    assert!(ObjFileParser::parse_vertex_normal(&vertex_normal_lines[1]) == Vector::new(4.1, -4.2, 4.3));

    assert!(ObjFileParser::obj_line_type(&vertex_normal_lines[2]) == ObjLineType::VertexNormal);
    assert!(ObjFileParser::parse_vertex_normal(&vertex_normal_lines[2]) == Vector::new(-5.1, 5.2, -5.3));
  }

  #[test]
  fn creates_triangles_from_face_lines_in_obj_file() {
    let mut triangles: Vec<Triangle> = Vec::new();
    let mut vertices: Vec<Point> = Vec::new();

    let obj_lines = vec![
      "v 0.1 0.2 0.3".to_string(), 
      "v 1.1 -1.2 1.3".to_string(), 
      "v -2.1 2.2 -2.3".to_string(),
      "".to_string(),
      "f 2 1 3".to_string()
    ];

    vertices.push(ObjFileParser::parse_vertex(&obj_lines[0]));
    vertices.push(ObjFileParser::parse_vertex(&obj_lines[1]));
    vertices.push(ObjFileParser::parse_vertex(&obj_lines[2]));

    ObjFileParser::parse_face(&obj_lines[4], Color::new(1.0, 1.0, 1.0, 1.0), &mut vertices, &mut triangles);
  
    let triangle = &triangles[0];

    assert!(triangle.point_1 == Point::new(1.1, -1.2, 1.3));
    assert!(triangle.point_2 == Point::new(0.1, 0.2, 0.3));
    assert!(triangle.point_3 == Point::new(-2.1, 2.2, -2.3));
  }

  #[test]
  fn creates_multiple_triangles_from_complex_polygon() {
    let mut triangles: Vec<Triangle> = Vec::new();
    let mut vertices: Vec<Point> = Vec::new();

    let obj_lines = vec![
      "v -1.0 1.0 0.0".to_string(), 
      "v -1.0 0.0 0.0".to_string(), 
      "v 1.0 0.0 0.0".to_string(),
      "v 1.0 1.0 0.0".to_string(),
      "v 0.0 2.0 0.0".to_string(),
      "".to_string(),
      "f 1 2 3 4 5".to_string()
    ];

    vertices.push(ObjFileParser::parse_vertex(&obj_lines[0]));
    vertices.push(ObjFileParser::parse_vertex(&obj_lines[1]));
    vertices.push(ObjFileParser::parse_vertex(&obj_lines[2]));
    vertices.push(ObjFileParser::parse_vertex(&obj_lines[3]));
    vertices.push(ObjFileParser::parse_vertex(&obj_lines[4]));

    ObjFileParser::parse_face(&obj_lines[6], Color::new(1.0, 1.0, 1.0, 1.0), &mut vertices, &mut triangles);
  
    assert_eq!(triangles.len(), 3);

    let triangle_1 = &triangles[0];

    assert!(triangle_1.point_1 == Point::new(-1.0, 1.0, 0.0));
    assert!(triangle_1.point_2 == Point::new(-1.0, 0.0, 0.0));
    assert!(triangle_1.point_3 == Point::new(1.0, 0.0, 0.0));

    let triangle_2 = &triangles[1];

    assert!(triangle_2.point_1 == Point::new(-1.0, 1.0, 0.0));
    assert!(triangle_2.point_2 == Point::new(1.0, 0.0, 0.0));
    assert!(triangle_2.point_3 == Point::new(1.0, 1.0, 0.0));

    let triangle_3 = &triangles[2];

    assert!(triangle_3.point_1 == Point::new(-1.0, 1.0, 0.0));
    assert!(triangle_3.point_2 == Point::new(1.0, 1.0, 0.0));
    assert!(triangle_3.point_3 == Point::new(0.0, 2.0, 0.0));
  }

  #[test]
  fn creates_smooth_triangles_from_obj_lines_with_normals() {
    let mut smooth_triangles: Vec<SmoothTriangle> = Vec::new();
    let mut vertices: Vec<Point> = Vec::new();
    let mut normals: Vec<Vector> = Vec::new();

    let obj_lines = vec![
      "v -1.0 1.0 0.0".to_string(), 
      "v -1.0 0.0 0.0".to_string(), 
      "v 1.0 0.0 0.0".to_string(),
      "v 1.0 1.0 0.0".to_string(),
      "v 0.0 2.0 0.0".to_string(),
      "".to_string(),
      "vn 0.0 0.0 1.0".to_string(),
      "vn 1.0 0.0 0.0".to_string(),
      "vn 0.0 1.0 0.0".to_string(),
      "f 1/999/3 2/999/2 3/999/1 4/999/2 5/999/3".to_string()
    ];

    vertices.push(ObjFileParser::parse_vertex(&obj_lines[0]));
    vertices.push(ObjFileParser::parse_vertex(&obj_lines[1]));
    vertices.push(ObjFileParser::parse_vertex(&obj_lines[2]));
    vertices.push(ObjFileParser::parse_vertex(&obj_lines[3]));
    vertices.push(ObjFileParser::parse_vertex(&obj_lines[4]));

    normals.push(ObjFileParser::parse_vertex_normal(&obj_lines[6]));
    normals.push(ObjFileParser::parse_vertex_normal(&obj_lines[7]));
    normals.push(ObjFileParser::parse_vertex_normal(&obj_lines[8]));

    ObjFileParser::parse_vertex_normal_face(&obj_lines[9], Color::new(1.0, 1.0, 1.0, 1.0), &mut vertices, &mut normals, &mut smooth_triangles);
  
    assert_eq!(smooth_triangles.len(), 3);

    let smooth_triangle_1 = &smooth_triangles[0];

    assert!(smooth_triangle_1.point_1 == Point::new(-1.0, 1.0, 0.0));
    assert!(smooth_triangle_1.point_2 == Point::new(-1.0, 0.0, 0.0));
    assert!(smooth_triangle_1.point_3 == Point::new(1.0, 0.0, 0.0));
    assert!(smooth_triangle_1.normal_1 == Vector::new(0.0, 1.0, 0.0));
    assert!(smooth_triangle_1.normal_2 == Vector::new(1.0, 0.0, 0.0));
    assert!(smooth_triangle_1.normal_3 == Vector::new(0.0, 0.0, 1.0));

    let smooth_triangle_2 = &smooth_triangles[1];

    assert!(smooth_triangle_2.point_1 == Point::new(-1.0, 1.0, 0.0));
    assert!(smooth_triangle_2.point_2 == Point::new(1.0, 0.0, 0.0));
    assert!(smooth_triangle_2.point_3 == Point::new(1.0, 1.0, 0.0));
    assert!(smooth_triangle_2.normal_1 == Vector::new(0.0, 1.0, 0.0));
    assert!(smooth_triangle_2.normal_2 == Vector::new(0.0, 0.0, 1.0));
    assert!(smooth_triangle_2.normal_3 == Vector::new(1.0, 0.0, 0.0));

    let smooth_triangle_3 = &smooth_triangles[2];

    assert!(smooth_triangle_3.point_1 == Point::new(-1.0, 1.0, 0.0));
    assert!(smooth_triangle_3.point_2 == Point::new(1.0, 1.0, 0.0));
    assert!(smooth_triangle_3.point_3 == Point::new(0.0, 2.0, 0.0));
    assert!(smooth_triangle_3.normal_1 == Vector::new(0.0, 1.0, 0.0));
    assert!(smooth_triangle_3.normal_2 == Vector::new(1.0, 0.0, 0.0));
    assert!(smooth_triangle_3.normal_3 == Vector::new(0.0, 1.0, 0.0));
  }
}
