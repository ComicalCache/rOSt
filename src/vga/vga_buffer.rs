use bootloader::boot_info::{FrameBuffer, FrameBufferInfo, PixelFormat};

use super::{
    vga_color::VGAColor,
    vga_core::{Clearable, PlainDrawable, TextDrawable, CHAR_HEIGHT, CHAR_WEIGHT, CHAR_WIDTH},
};
use noto_sans_mono_bitmap::{get_bitmap, BitmapChar};

pub struct VGADevice<'a> {
    frame_pointer: &'a mut [u8],
    frame_buffer_info: FrameBufferInfo,
    bytes_per_row: usize,
}

impl Clearable for VGADevice<'_> {
    fn clear(&mut self, color: &VGAColor) {
        for x in 0..self.frame_buffer_info.horizontal_resolution {
            for y in 0..self.frame_buffer_info.vertical_resolution {
                self.draw_point(x, y, color);
            }
        }
    }
}

impl PlainDrawable for VGADevice<'_> {
    fn draw_point(&mut self, x: usize, y: usize, color: &VGAColor) {
        let index = y * self.bytes_per_row + x * self.frame_buffer_info.bytes_per_pixel;
        match self.frame_buffer_info.pixel_format {
            PixelFormat::RGB => {
                let frame_color = VGAColor {
                    red: self.frame_pointer[index + 0],
                    green: self.frame_pointer[index + 1],
                    blue: self.frame_pointer[index + 2],
                    alpha: 255,
                };
                let result_color = VGAColor::interpolate(&frame_color, color, color.alpha);
                self.frame_pointer[index + 0] = result_color.red;
                self.frame_pointer[index + 1] = result_color.green;
                self.frame_pointer[index + 2] = result_color.blue;
            }
            PixelFormat::BGR => {
                let frame_color = VGAColor {
                    red: self.frame_pointer[index + 2],
                    green: self.frame_pointer[index + 1],
                    blue: self.frame_pointer[index + 0],
                    alpha: 255,
                };
                let result_color = VGAColor::interpolate(&frame_color, color, color.alpha);
                self.frame_pointer[index + 2] = result_color.red;
                self.frame_pointer[index + 1] = result_color.green;
                self.frame_pointer[index + 0] = result_color.blue;
            }
            PixelFormat::U8 => {
                let gray = self.frame_pointer[index] as u16;
                let color_gray = color.to_grayscale() as u16;
                let alpha = color.alpha as u16;
                let alpha1 = 255 - alpha;
                self.frame_pointer[index] = ((gray * alpha1 + color_gray * alpha) / 255) as u8;
            }
            _ => todo!(
                "Unsupported pixel format: {:?}",
                self.frame_buffer_info.pixel_format
            ),
        }
    }

    fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: &VGAColor) {
        let ix2: isize = x2.try_into().unwrap();
        let iy2: isize = y2.try_into().unwrap();
        // Bresenham's algorithm

        let mut x = x1 as isize;
        let mut y = y1 as isize;

        let xi: isize;
        let dx: isize;
        if x1 < x2 {
            xi = 1;
            dx = (x2 - x1) as isize;
        } else {
            xi = -1;
            dx = (x1 - x2) as isize;
        }

        let yi: isize;
        let dy: isize;
        if y1 < y2 {
            yi = 1;
            dy = (y2 - y1) as isize;
        } else {
            yi = -1;
            dy = (y1 - y2) as isize;
        }
        self.draw_point(x as usize, y as usize, color);

        let ai;
        let bi;
        let mut d: isize;
        // OX axis
        if dx > dy {
            ai = (dy - dx) * 2;
            bi = dy * 2;
            d = bi - dx;
            // Loop over next Xs
            while x != ix2 {
                if d >= 0 {
                    x += xi;
                    y += yi;
                    d += ai;
                } else {
                    d += bi;
                    x += xi;
                }
                self.draw_point(x as usize, y as usize, color);
            }
        }
        // OY axis
        else {
            ai = (dx - dy) * 2;
            bi = dx * 2;
            d = bi - dy;
            // Loop over next Ys
            while y != iy2 {
                if d >= 0 {
                    x += xi;
                    y += yi;
                    d += ai;
                } else {
                    d += bi;
                    y += yi;
                }
                self.draw_point(x as usize, y as usize, color);
            }
        }
    }

    fn draw_rectangle(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: &VGAColor,
    ) {
        self.draw_line(x, y, x + width, y, color);
        self.draw_line(x, y + height, x + width, y + height, color);
        self.draw_line(x, y, x, y + height, color);
        self.draw_line(x + width, y, x + width, y + height, color);
    }

    fn fill_rectangle(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: &VGAColor,
    ) {
        for i in x..x + width {
            for j in y..y + height {
                self.draw_point(i, j, color);
            }
        }
    }
}

impl TextDrawable for VGADevice<'_> {
    fn draw_string(
        &mut self,
        x: usize,
        y: usize,
        color: &VGAColor,
        text: &str,
        reset_x: usize,
    ) -> (usize, usize) {
        let mut pos_x = x;
        let mut pos_y = y;
        for (_i, c) in text.chars().enumerate() {
            match c {
                '\n' => {
                    pos_x = reset_x;
                    pos_y += CHAR_HEIGHT as usize;
                }
                _ => {
                    if pos_x + CHAR_WIDTH > self.frame_buffer_info.horizontal_resolution as usize {
                        pos_x = reset_x;
                        pos_y += CHAR_HEIGHT as usize;
                    }
                    let bitmap_char = get_bitmap(c, CHAR_WEIGHT, CHAR_HEIGHT).unwrap();
                    self.draw_char(pos_x, pos_y, bitmap_char, color);
                    pos_x += CHAR_WIDTH;
                }
            }
        }
        (pos_x, pos_y)
    }

    fn measure_string(&self, x: usize, y: usize, text: &str, reset_x: usize) -> (usize, usize) {
        let mut pos_x = x;
        let mut pos_y = y;
        for (_i, c) in text.chars().enumerate() {
            match c {
                '\n' => {
                    pos_x = reset_x;
                    pos_y += CHAR_HEIGHT as usize;
                }
                _ => {
                    pos_x += CHAR_WIDTH;
                    if pos_x > self.frame_buffer_info.horizontal_resolution as usize {
                        pos_x = reset_x;
                        pos_y += CHAR_HEIGHT as usize;
                    }
                }
            }
        }
        (pos_x, pos_y)
    }
}

impl VGADevice<'_> {
    fn draw_char(&mut self, x: usize, y: usize, char: BitmapChar, color: &VGAColor) {
        for (iy, row) in char.bitmap().iter().enumerate() {
            for (ix, byte) in row.iter().enumerate() {
                self.draw_point(ix + x, iy + y, &color.multiply_alpha(*byte));
            }
        }
    }
}

pub struct VGADeviceFactory;
impl VGADeviceFactory {
    pub fn from_buffer(frame_buffer: &mut FrameBuffer) -> VGADevice {
        let info = frame_buffer.info();
        VGADevice {
            frame_buffer_info: info,
            bytes_per_row: info.bytes_per_pixel * info.stride,
            frame_pointer: frame_buffer.buffer_mut(),
        }
    }
}
