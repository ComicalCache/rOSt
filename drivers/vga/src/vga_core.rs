use lazy_static::lazy_static;
use noto_sans_mono_bitmap::{get_bitmap, get_bitmap_width, BitmapChar, BitmapHeight, FontWeight};

use super::{point_2d::Point2D, vga_color::VGAColor};

pub const CHAR_HEIGHT: BitmapHeight = BitmapHeight::Size14;
pub const CHAR_WEIGHT: FontWeight = FontWeight::Regular;
pub const CHAR_WIDTH: u16 = get_bitmap_width(CHAR_WEIGHT, CHAR_HEIGHT) as u16;
lazy_static! {
    pub static ref INVALID_CHAR: BitmapChar = get_bitmap(' ', CHAR_WEIGHT, CHAR_HEIGHT).unwrap();
}

pub trait Clearable {
    fn clear(&mut self, color: VGAColor<u8>);
}

pub trait PlainDrawable {
    fn draw_point(&mut self, x: u16, y: u16, color: VGAColor<u8>);
    fn draw_point_p(&mut self, p: Point2D<u16>, color: VGAColor<u8>);
    fn draw_line(&mut self, x1: u16, y1: u16, x2: u16, y2: u16, color: VGAColor<u8>);
    fn draw_line_p(&mut self, a: Point2D<u16>, b: Point2D<u16>, color: VGAColor<u8>);
    fn draw_bezier(
        &mut self,
        p1: Point2D<u16>,
        p2: Point2D<u16>,
        p3: Point2D<u16>,
        p4: Point2D<u16>,
        color: VGAColor<u8>,
    );
}

pub trait ShapeDrawable {
    fn draw_rectangle(&mut self, x: u16, y: u16, width: u16, height: u16, color: VGAColor<u8>);
    fn draw_rectangle_p(&mut self, min: Point2D<u16>, max: Point2D<u16>, color: VGAColor<u8>);
    fn fill_rectangle(&mut self, x: u16, y: u16, width: u16, height: u16, color: VGAColor<u8>);
    fn fill_rectangle_p(&mut self, min: Point2D<u16>, max: Point2D<u16>, color: VGAColor<u8>);
}

pub trait TextDrawable {
    fn draw_string(
        &mut self,
        x: u16,
        y: u16,
        color: VGAColor<u8>,
        text: &str,
        reset_x: u16,
    ) -> (u16, u16);
    fn measure_string(&self, x: u16, y: u16, text: &str, reset_x: u16) -> (u16, u16);
}
