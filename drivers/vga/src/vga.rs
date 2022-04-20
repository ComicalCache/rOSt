#![feature(ptr_const_cast)]
use core::fmt;
use lazy_static::lazy_static;
use os_core::{logger::Logger, structures::kernel_information::KernelInformation};
use spin::Mutex;
use vga_buffer::{VGADevice, VGADeviceFactory};
use vga_core::{Clearable, TextDrawable, CHAR_HEIGHT};

pub mod point_2d;
pub mod vga_buffer;
pub mod vga_color;
pub mod vga_core;

struct VGALogger {
    x: u16,
    y: u16,
    start_x: u16,
    device: VGADevice<'static>,
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

pub extern "C" fn driver_init(kernel_info: KernelInformation) {
    os_core::logger::LOGGER.lock().replace(&mut VGALogger {
        x: 0,
        start_x: 0,
        y: 0,
        device: VGADeviceFactory::from_buffer(kernel_info),
        took_over: false,
    });
}
