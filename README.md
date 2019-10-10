# JBTracer

A Ray Tracer written in the Rust programming language.

## Deployment

This application depends on the crates:
  - `num = "0.2"`
  - `chrono = "0.4"`
  - `rand = "0.7.2"`
  
 ## Features
 
 JBTracer can draw six primitive shapes:
   - Plane
   - Cone
   - Cylinder
   - Sphere
   - Triangle
   - SmoothTriangle (triangle with interpolated normals)
These primitive types support fully customizable materials, which allow custom coloring, reflectiveness and translucency:
$$ ADD ONE OR TWO IMAGES OF ALL PRIMITIVE TYPES $$

Four patterns are available:
  - Checkered
  - Striped
  - Ringed
  - Gradient
These patterns can be drawn to the surface of any primitive:
$$ ADD IMAGE OF ALL PATTERNS $$

JBTracer can also draw constructive geometries by evaluating unions, intersection and differences of the primitive types.
 $$ ADD ONE OR TWO IMAGES OF CONSTRUCTIVE GEOMETRIES $$

An object file reader has been included as well. JBTracer can read basic object files, and create grouped objects (either with or without vertex normals) by parsing the data and generating Triangle or SmoothTriangle objects.
$$ ADD IMAGE OF TEAPOTS WITH AND WITHOUT NORMAL INTERPOLATION - WITH CAPTION $$
$$ ADD IMAGE COW - WITH CAPTION $$

The screensize of the output image can be adjusted by altering the dimensions of the `Canvas` object. The image is saved in the PPM format.
$$ ADD IMAGE OF SAME SHOT FROM DIFFERENT SIZED CANVASES $$

Anti-Aliasing support is also available. The image can be output with no Anti-Aliasing, or it can be output using Anti-Aliasing sample rates of either four or sixteen samples per pixel by adjusting which coloring function is called in the main rendering loop. 
$$ ADD IMAGES OF SAME SHAPE WITH AND WITHOUT ANTI-ALIASING $$

## Tests

A full test suite has been implemented, and is spread out across `_test.rs` files within the `rendering` module. The entire suite can be run by executing `cargo test`.
 
