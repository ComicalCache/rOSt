use noto_sans_mono_bitmap::{get_bitmap_width, BitmapHeight, FontWeight};

use super::vga_color::VGAColor;

pub const CHAR_HEIGHT: BitmapHeight = BitmapHeight::Size14;
pub const CHAR_WEIGHT: FontWeight = FontWeight::Regular;
pub const CHAR_WIDTH: usize = get_bitmap_width(CHAR_WEIGHT, CHAR_HEIGHT);

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
    fn draw_string(
        &mut self,
        x: usize,
        y: usize,
        color: &VGAColor,
        text: &str,
        reset_x: usize,
    ) -> (usize, usize);
    fn measure_string(&self, x: usize, y: usize, text: &str, reset_x: usize) -> (usize, usize);
}
