use bootloader::boot_info::{FrameBuffer, FrameBufferInfo, PixelFormat};

use crate::{structures::static_stack::StaticStack, vga::vga_core::CHAR_HEIGHT};

use super::{
    point_2d::Point2D,
    vga_color::VGAColor,
    vga_core::{
        Clearable, PlainDrawable, ShapeDrawable, TextDrawable, CHAR_WEIGHT, CHAR_WIDTH,
        INVALID_CHAR,
    },
};
use noto_sans_mono_bitmap::{get_bitmap, BitmapChar};

pub struct VGADevice<'a> {
    frame_pointer: &'a mut [u8],
    frame_buffer_info: FrameBufferInfo,
    bytes_per_row: usize,
}

impl Clearable for VGADevice<'_> {
    fn clear(&mut self, color: VGAColor<u8>) {
        for x in 0..self.frame_buffer_info.horizontal_resolution {
            for y in 0..self.frame_buffer_info.vertical_resolution {
                self.draw_point(x as u16, y as u16, color);
            }
        }
    }
}

impl PlainDrawable for VGADevice<'_> {
    fn draw_point(&mut self, x: u16, y: u16, color: VGAColor<u8>) {
        let _x = x as usize;
        let _y = y as usize;
        let index = _y * self.bytes_per_row + _x * self.frame_buffer_info.bytes_per_pixel;
        match self.frame_buffer_info.pixel_format {
            PixelFormat::RGB => {
                let frame_color = VGAColor {
                    red: self.frame_pointer[index + 0],
                    green: self.frame_pointer[index + 1],
                    blue: self.frame_pointer[index + 2],
                    alpha: 255,
                };
                let result_color = VGAColor::interpolate(frame_color, color, color.alpha);
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
                let result_color = VGAColor::interpolate(frame_color, color, color.alpha);
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
    fn draw_point_p(&mut self, p: Point2D<u16>, color: VGAColor<u8>) {
        self.draw_point(p.x, p.y, color);
    }

    fn draw_line(&mut self, x1: u16, y1: u16, x2: u16, y2: u16, color: VGAColor<u8>) {
        let ix2: isize = x2 as isize;
        let iy2: isize = y2 as isize;
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
        self.draw_point(x as u16, y as u16, color);

        let ai;
        let bi;
        let mut d: isize;
        if dx > dy {
            ai = (dy - dx) * 2;
            bi = dy * 2;
            d = bi - dx;
            // pętla po kolejnych x
            while x != ix2 {
                // test współczynnika
                if d >= 0 {
                    x += xi;
                    y += yi;
                    d += ai;
                } else {
                    d += bi;
                    x += xi;
                }
                self.draw_point(x as u16, y as u16, color);
            }
        }
        // oś wiodąca OY
        else {
            ai = (dx - dy) * 2;
            bi = dx * 2;
            d = bi - dy;
            // pętla po kolejnych y
            while y != iy2 {
                // test współczynnika
                if d >= 0 {
                    x += xi;
                    y += yi;
                    d += ai;
                } else {
                    d += bi;
                    y += yi;
                }
                self.draw_point(x as u16, y as u16, color);
            }
        }
    }
    fn draw_line_p(&mut self, a: Point2D<u16>, b: Point2D<u16>, color: VGAColor<u8>) {
        self.draw_line(a.x, a.y, b.x, b.y, color);
    }

    fn draw_bezier(
        &mut self,
        p1: Point2D<u16>,
        p2: Point2D<u16>,
        p3: Point2D<u16>,
        p4: Point2D<u16>,
        color: VGAColor<u8>,
    ) {
        let mut t_stack: StaticStack<(f32, f32), 32> = StaticStack::new();
        t_stack.push(&(0f32, 1f32));
        while t_stack.length() > 0 {
            let frame = t_stack.pop().unwrap();
            let a = bezier_point(p1, p2, p3, p4, frame.0);
            let b = bezier_point(p1, p2, p3, p4, frame.1);
            if a.sqr_distance::<i32>(b) > 16 {
                let mid = (frame.1 + frame.0) * 0.5;
                t_stack.push(&(frame.0, mid));
                t_stack.push(&(mid, frame.1));
            } else {
                self.draw_line_p(a, b, color);
            }
        }
    }
}

fn bezier_point(
    p1: Point2D<u16>,
    p2: Point2D<u16>,
    p3: Point2D<u16>,
    p4: Point2D<u16>,
    t: f32,
) -> Point2D<u16> {
    let t_1 = 1f32 - t;
    let t2 = t * t;
    let t3 = t2 * t;
    let _p1: Point2D<f32> = p1.into();
    let _p2: Point2D<f32> = (p2 * 3).into();
    let _p3: Point2D<f32> = (p3 * 3).into();
    let _p4: Point2D<f32> = p4.into();
    (((_p1 * t_1 + _p2 * t) * t_1 + _p3 * t2) * t_1 + _p4 * t3).into()
}

impl ShapeDrawable for VGADevice<'_> {
    fn draw_rectangle(&mut self, x: u16, y: u16, width: u16, height: u16, color: VGAColor<u8>) {
        self.draw_line(x, y, x + width, y, color);
        self.draw_line(x, y + height, x + width, y + height, color);
        self.draw_line(x, y, x, y + height, color);
        self.draw_line(x + width, y, x + width, y + height, color);
    }
    fn draw_rectangle_p(&mut self, min: Point2D<u16>, max: Point2D<u16>, color: VGAColor<u8>) {
        self.draw_rectangle(min.x, min.y, max.x - min.x, max.y - min.y, color);
    }

    fn fill_rectangle(&mut self, x: u16, y: u16, width: u16, height: u16, color: VGAColor<u8>) {
        for i in x..x + width {
            for j in y..y + height {
                self.draw_point(i, j, color);
            }
        }
    }
    fn fill_rectangle_p(&mut self, min: Point2D<u16>, max: Point2D<u16>, color: VGAColor<u8>) {
        self.fill_rectangle(min.x, min.y, max.x - min.x, max.y - min.y, color);
    }
}

impl TextDrawable for VGADevice<'_> {
    fn draw_string(
        &mut self,
        x: u16,
        y: u16,
        color: VGAColor<u8>,
        text: &str,
        reset_x: u16,
    ) -> (u16, u16) {
        let mut pos_x = x;
        let mut pos_y = y;
        for (_i, c) in text.chars().enumerate() {
            match c {
                '\n' => {
                    pos_x = reset_x;
                    pos_y += CHAR_HEIGHT as u16;
                }
                _ => {
                    if pos_x + CHAR_WIDTH as u16
                        > self.frame_buffer_info.horizontal_resolution as u16
                    {
                        pos_x = reset_x;
                        pos_y += CHAR_HEIGHT as u16;
                    }
                    let invalid_char = &*INVALID_CHAR;
                    let bitmap_char = get_bitmap(c, CHAR_WEIGHT, CHAR_HEIGHT);
                    self.draw_char(
                        pos_x,
                        pos_y,
                        &bitmap_char.as_ref().unwrap_or(invalid_char),
                        color,
                    );
                    pos_x += CHAR_WIDTH as u16;
                }
            }
        }
        (pos_x, pos_y)
    }

    fn measure_string(&self, x: u16, y: u16, text: &str, reset_x: u16) -> (u16, u16) {
        let mut pos_x = x;
        let mut pos_y = y;
        for (_i, c) in text.chars().enumerate() {
            match c {
                '\n' => {
                    pos_x = reset_x;
                    pos_y += CHAR_HEIGHT as u16;
                }
                _ => {
                    pos_x += CHAR_WIDTH as u16;
                    if pos_x > self.frame_buffer_info.horizontal_resolution as u16 {
                        pos_x = reset_x;
                        pos_y += CHAR_HEIGHT as u16;
                    }
                }
            }
        }
        (pos_x, pos_y)
    }
}

impl VGADevice<'_> {
    fn draw_char(&mut self, x: u16, y: u16, char: &BitmapChar, color: VGAColor<u8>) {
        for (iy, row) in char.bitmap().iter().enumerate() {
            for (ix, byte) in row.iter().enumerate() {
                self.draw_point(ix as u16 + x, iy as u16 + y, color.mul_alpha(*byte));
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
