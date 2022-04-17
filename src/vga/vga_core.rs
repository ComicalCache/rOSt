
use super::{vga_color::VGAColor, point_2d::Point2D};


pub trait Interpolatable<T, A: Interpolatable<T, A> = Self> {
    fn interpolate(a: A, b: A, t: T) -> A;
}

pub trait Clearable {
  fn clear(&mut self, color: VGAColor<u8>);
}

pub trait PlainDrawable {
  fn draw_point(&mut self, x: u16, y: u16, color: VGAColor<u8>);
  fn draw_point_p(&mut self, p: Point2D<u16>, color: VGAColor<u8>);
  fn draw_line(&mut self, x1: u16, y1: u16, x2: u16, y2: u16, color: VGAColor<u8>);
  fn draw_line_p(&mut self, a: Point2D<u16>, b: Point2D<u16>, color: VGAColor<u8>);
  fn draw_bezier(&mut self, p1: Point2D<u16>, p2: Point2D<u16>, p3: Point2D<u16>, p4: Point2D<u16>, color: VGAColor<u8>);
}

pub trait ShapeDrawable {
  fn draw_rectangle(&mut self, x: u16, y: u16, width: u16, height: u16, color: VGAColor<u8>);
  fn draw_rectangle_p(&mut self, min: Point2D<u16>, max: Point2D<u16>, color: VGAColor<u8>);
  fn fill_rectangle(&mut self, x: u16, y: u16, width: u16, height: u16, color: VGAColor<u8>);
  fn fill_rectangle_p(&mut self, min: Point2D<u16>, max: Point2D<u16>, color: VGAColor<u8>);
}

pub trait TextDrawable {
  fn draw_string(&mut self, x: u16, y: u16, color: VGAColor<u8>, text: &str, reset_x: u16) -> (u16, u16);
  fn measure_string(&self, x: u16, y: u16, text: &str, reset_x: u16) -> (u16, u16);
}