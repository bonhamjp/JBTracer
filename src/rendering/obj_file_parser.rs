use std::f64;

use std::io::prelude::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use crate::rendering::shape::Shape;
use crate::rendering::Triangle;

use crate::rendering::Container;

use crate::rendering::Material;

use crate::math::tuple::Tuple;
use crate::math::Point;
use crate::math::Vector;

use crate::math::Color;

use crate::math::Matrix4x4;

#[derive(PartialEq)]
pub enum ObjLineType {
  Vertex,
  Face,
  Group,
  Invalid
}

pub struct ObjFileParser {
  pub vertex_count: u64
}

impl ObjFileParser {
  pub fn new(file_name: String) -> ObjFileParser {
    ObjFileParser { 
      vertex_count: 929
    }
  }

  pub fn load_file<'a>(file_name: String, transform: Matrix4x4, color: Color, triangles: &'a mut Vec<Triangle>) -> Container<'a> {
    let mut vertices = Vec::new();

    let obj_file = File::open(file_name).unwrap();
    let reader = BufReader::new(obj_file);

    for line in reader.lines() {    
      let obj_line = line.unwrap();
      
      if (ObjFileParser::obj_line_type(&obj_line) == ObjLineType::Group) {
        
      } else if (ObjFileParser::obj_line_type(&obj_line) == ObjLineType::Vertex) {
        vertices.push(ObjFileParser::parse_vertex(&obj_line));

      } else if (ObjFileParser::obj_line_type(&obj_line) == ObjLineType::Face) {
        ObjFileParser::parse_face(&obj_line, color, &mut vertices, triangles);
      
      }
    }

    let mut container_shape_references = Vec::new();
    for triangle in triangles {    
      container_shape_references.push(triangle as &dyn Shape);
    }

    Container::new(transform, container_shape_references)
  }

  // TODO: Add load file with normal data

  pub fn obj_line_type(obj_line: &String) -> ObjLineType {
    match obj_line.chars().next() {
      Some('v') => {
        return ObjLineType::Vertex;
      },
      Some('f') => {
        return ObjLineType::Face;
      },
      Some('g') => {
        return ObjLineType::Group;
      },
      Some(_) => {
        return ObjLineType::Invalid;
      },
      None => {
        return ObjLineType::Invalid;
      },
    }
  }

  pub fn parse_vertex(obj_line: &String) -> Point {
    let mut vertex_line = obj_line.clone();

    let vertex_parts = vertex_line.split(" ").collect::<Vec<&str>>();
    
    // Must have exactly four parts
    Point::new(
      vertex_parts[1].parse::<f64>().unwrap(),
      vertex_parts[2].parse::<f64>().unwrap(),
      vertex_parts[3].parse::<f64>().unwrap()
    )
  }

  // TODO: Add parse vertex normal function

  pub fn parse_face(obj_line: &String, color: Color, vertices: &mut Vec::<Point>, triangles: &mut Vec::<Triangle>) {
    let mut face_line = obj_line.clone();

    let face_indices = face_line.split(" ").collect::<Vec<&str>>();
    
    // Split into triangles if face contains more than three vertices
    for i in 2..(face_indices.len() - 1) {
      let point_1 = vertices[face_indices[1].parse::<usize>().unwrap() - 1];
      let point_2 = vertices[face_indices[i].parse::<usize>().unwrap() - 1];
      let point_3 = vertices[face_indices[i + 1].parse::<usize>().unwrap() - 1];
      
      let triangle = Triangle::new(
        999, 
        point_1,
        point_2,
        point_3,
        Matrix4x4::identity(), 
        Material::solid(0.1, 0.3, 0.3, 4.0, 0.0, 0.0, 1.0, color, Matrix4x4::identity())
      );

      triangles.push(triangle);
    }
  }

  // TODO: Add parse face with normal data function

}
