
use super::{vga_color::VGAColor, vga_point::VGAPoint};

pub trait Integer: Sized + PartialEq + core::ops::Mul + core::ops::Div + core::ops::Add + core::ops::Sub + PartialOrd + Ord + Eq {}
impl<T: Sized + PartialEq + core::ops::Mul + core::ops::Div + core::ops::Add + core::ops::Sub + PartialOrd + Ord + Eq> Integer for T {}

pub trait Interpolatable<B: Integer, A: Interpolatable<B, A> = Self> {
    fn interpolate(a: &A, b: &A, t: B) -> A;
}

pub trait Clearable {
  fn clear(&mut self, color: &VGAColor);
}

pub trait PlainDrawable {
  fn draw_point(&mut self, x: usize, y: usize, color: &VGAColor);
  fn draw_point_p(&mut self, p: &VGAPoint, color: &VGAColor);
  fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: &VGAColor);
  fn draw_line_p(&mut self, a: &VGAPoint, b: &VGAPoint, color: &VGAColor);
  fn draw_bezier(&mut self, p1: &VGAPoint, p2: &VGAPoint, p3: &VGAPoint, p4: &VGAPoint, color: &VGAColor);
}

pub trait ShapeDrawable {
  fn draw_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize, color: &VGAColor);
  fn draw_rectangle_p(&mut self, a: &VGAPoint, b: &VGAPoint, color: &VGAColor);
  fn fill_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize, color: &VGAColor);
  fn fill_rectangle_p(&mut self, a: &VGAPoint, b: &VGAPoint, color: &VGAColor);
}

pub trait TextDrawable {
  fn draw_string(&mut self, x: usize, y: usize, color: &VGAColor, text: &str, reset_x: usize) -> (usize, usize);
  fn measure_string(&self, x: usize, y: usize, text: &str, reset_x: usize) -> (usize, usize);
}