#[cfg(test)]
mod tests {
  use crate::rendering::math::Color;

  use crate::rendering::Canvas;
  
  #[test]
  fn new_canvas_sets_width_height_and_correct_sized_buffer() {
    let canvas = Canvas::new(4, 2);

    assert_eq!(canvas.width, 4);
    assert_eq!(canvas.height, 2);
    assert_eq!(canvas.color_buffer.len(), 8);
  }

  #[test]
  fn new_colors_are_set_to_default_value() {
    let canvas = Canvas::new(2, 2);

    assert_eq!(canvas.color_buffer[0].display_values(), (255, 255, 255, 255));
    assert_eq!(canvas.color_buffer[1].display_values(), (255, 255, 255, 255));
    assert_eq!(canvas.color_buffer[2].display_values(), (255, 255, 255, 255));
    assert_eq!(canvas.color_buffer[3].display_values(), (255, 255, 255, 255));
  }

  #[test]
  fn outputs_data_in_ppm_v3_format() {
    let mut canvas = Canvas::new(2, 2);

    canvas.color_pixel(0, 0, Color::new(1.0, 0.0, 0.0, 1.0));
    canvas.color_pixel(1, 1, Color::new(0.0, 0.0, 1.0, 1.0));

    let image_output = canvas.image_output(); 

    assert_eq!(image_output[0], "P3\n");
    assert_eq!(image_output[1], "2 2\n");
    assert_eq!(image_output[2], "255\n");
    assert_eq!(image_output[3], "255 0 0 255 255 255 \n");
    assert_eq!(image_output[4], "255 255 255 0 0 255 \n");
    assert_eq!(image_output[5], "");
  }

  #[test]
  fn outputs_data_in_ppm_v3_format_with_max_70_char_lines() {
    let mut canvas = Canvas::new(8, 2);

    canvas.color_pixel(0, 0, Color::new(1.0, 0.0, 0.0, 1.0));
    canvas.color_pixel(0, 1, Color::new(0.0, 1.0, 0.0, 1.0));
    canvas.color_pixel(0, 2, Color::new(0.0, 0.0, 1.0, 1.0));
    canvas.color_pixel(0, 3, Color::new(0.0, 1.0, 1.0, 1.0));
    canvas.color_pixel(0, 4, Color::new(1.0, 0.0, 1.0, 1.0));
    canvas.color_pixel(0, 5, Color::new(1.0, 1.0, 0.0, 1.0));
    canvas.color_pixel(0, 6, Color::new(1.0, 1.0, 1.0, 1.0));
    canvas.color_pixel(0, 7, Color::new(1.0, 1.0, 1.0, 1.0));

    let image_output = canvas.image_output(); 

    assert_eq!(image_output[0], "P3\n");
    assert_eq!(image_output[1], "8 2\n");
    assert_eq!(image_output[2], "255\n");
    assert_eq!(image_output[3], "255 0 0 0 255 0 0 0 255 0 255 255 255 0 255 255 255 0 255 255 255 \n");
    assert_eq!(image_output[4], "255 255 255 \n");
    assert_eq!(image_output[5], "255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 \n");
    assert_eq!(image_output[6], "255 255 255 255 255 255 255 \n");
    assert_eq!(image_output[7], "");
  }

  #[test]
  fn colors_pixel_at_location_with_provided_color() {
    let mut canvas = Canvas::new(4, 4);

    let red = Color::new(1.0, 0.0, 0.0, 1.0);
    let green = Color::new(0.0, 1.0, 0.0, 1.0);
    let blue = Color::new(0.0, 0.0, 1.0, 1.0);

    canvas.color_pixel(0, 0, red);
    assert_eq!(canvas.color_buffer[0].r, 1.0);
    assert_eq!(canvas.color_buffer[0].g, 0.0);
    assert_eq!(canvas.color_buffer[0].b, 0.0);
    assert_eq!(canvas.color_buffer[0].a, 1.0); 

    canvas.color_pixel(1, 1, green);
    assert_eq!(canvas.color_buffer[5].r, 0.0);
    assert_eq!(canvas.color_buffer[5].g, 1.0);
    assert_eq!(canvas.color_buffer[5].b, 0.0);
    assert_eq!(canvas.color_buffer[5].a, 1.0);

    canvas.color_pixel(2, 2, blue);
    assert_eq!(canvas.color_buffer[10].r, 0.0);
    assert_eq!(canvas.color_buffer[10].g, 0.0);
    assert_eq!(canvas.color_buffer[10].b, 1.0);
    assert_eq!(canvas.color_buffer[10].a, 1.0);
  }

  #[test]
  fn get_pixel_color_at_canvas_location() {
    let mut canvas = Canvas::new(4, 4);

    let red = Color::new(1.0, 0.0, 0.0, 1.0);
    let green = Color::new(0.0, 1.0, 0.0, 1.0);
    let blue = Color::new(0.0, 0.0, 1.0, 1.0);

    canvas.color_buffer[0] = red;
    let canvas_red = canvas.pixel_color(0, 0);
    assert_eq!(canvas_red.r, 1.0);
    assert_eq!(canvas_red.g, 0.0);
    assert_eq!(canvas_red.b, 0.0);
    assert_eq!(canvas_red.a, 1.0);
    
    canvas.color_buffer[5] = green;
    let canvas_green = canvas.pixel_color(1, 1);
    assert_eq!(canvas_green.r, 0.0);
    assert_eq!(canvas_green.g, 1.0);
    assert_eq!(canvas_green.b, 0.0);
    assert_eq!(canvas_green.a, 1.0);

    canvas.color_buffer[10] = blue;
    let canvas_blue = canvas.pixel_color(2, 2);
    assert_eq!(canvas_blue.r, 0.0);
    assert_eq!(canvas_blue.g, 0.0);
    assert_eq!(canvas_blue.b, 1.0);
    assert_eq!(canvas_blue.a, 1.0);
  }
}
