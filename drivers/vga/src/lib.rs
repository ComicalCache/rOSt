#![no_std] // no standard library
#![no_main]
#![allow(incomplete_features)]
#![feature(ptr_const_cast, generic_const_exprs, adt_const_params)]
use alloc::boxed::Box;
use core::fmt;
use internal_utils::structures::{driver::Driver, kernel_information::KernelInformation};
use kernel::logger::Logger;
use vga_core::{Clearable, TextDrawable, CHAR_HEIGHT};
use vga_device::{VGADevice, VGADeviceFactory};
extern crate alloc;

mod pixel_buffer;
pub mod point_2d;
mod static_stack;
pub mod vga_color;
pub mod vga_core;
pub mod vga_device;

struct VGALogger {
    x: u16,
    y: u16,
    start_x: u16,
    device: VGADevice,
    took_over: bool,
}

impl VGALogger {
    fn __log(&mut self, text: &str) {
        let (x, y) =
            self.device
                .draw_string(self.x, self.y, vga_color::CHARLOTTE, text, self.start_x);
        self.x = x;
        self.y = y;
    }
}

impl Logger for VGALogger {
    fn log(&mut self, text: &str) {
        if !self.took_over {
            self.device.clear(vga_color::BSOD_BLUE);
            self.took_over = true;
        }
        self.__log(text);
    }

    fn logln(&mut self, text: &str) {
        self.log(text);
        if self.x > 0 {
            self.x = 0;
            self.y += CHAR_HEIGHT as u16;
        }
    }
}

impl fmt::Write for VGALogger {
    /// This will never fail and can always be unwrapped.
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.log(s);
        Ok(())
    }
}

pub extern "C" fn driver_init(kernel_info: KernelInformation) -> Driver {
    kernel::LOGGER.lock().replace(Box::new(VGALogger {
        x: 0,
        start_x: 0,
        y: 0,
        device: VGADeviceFactory::from_kernel_info(kernel_info),
        took_over: false,
    }));
    Driver {
        signature: [
            0xf2, 0xf3, 0xf4, 0xf5, 0xf2, 0xf3, 0xf4, 0xf5, 0xf2, 0xf3, 0xf4, 0xf5, 0xf2, 0xf3,
            0xf4, 0xf5,
        ],
    }
}
