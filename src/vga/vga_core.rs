use super::vga_color::VGAColor;

pub trait Clearable {
  fn clear(&mut self, color: &VGAColor);
}

pub trait PlainDrawable {
  fn draw_point(&mut self, x: usize, y: usize, color: &VGAColor);
  fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: &VGAColor);
  fn draw_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize, color: &VGAColor);
  fn fill_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize, color: &VGAColor);
}

pub trait TextDrawable {
  fn draw_string(&mut self, x: usize, y: usize, color: &VGAColor, text: &str, reset_x: usize) -> (usize, usize);
  fn measure_string(&self, x: usize, y: usize, text: &str, reset_x: usize) -> (usize, usize);
}