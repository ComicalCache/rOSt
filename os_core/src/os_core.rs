#![no_std]
#![allow(incomplete_features)]
#![feature(abi_x86_interrupt, generic_const_exprs, core_intrinsics)]

use crate::structures::driver::Driver;
use bootloader::BootInfo;
use lazy_static::lazy_static;
use spin::Mutex;
use structures::driver::Registrator;
use structures::kernel_information::KernelInformation;

pub use crate::interrupts::gdt;
pub use crate::test_framework::ansi_colors;
use crate::test_framework::qemu_exit::exit_qemu;
use crate::test_framework::qemu_exit::QemuExitCode;
pub use crate::test_framework::serial;

use core::panic::PanicInfo;

pub mod interrupts;
pub mod logger;
pub mod structures;
pub mod test_framework;

lazy_static! {
    static ref REGISTERED_DRIVERS: Mutex<[Option<Registrator>; 256]> = Mutex::new([None; 256]);
    static ref INITIALIZED_DRIVERS: Mutex<[Option<Driver>; 256]> = Mutex::new([None; 256]);
}

pub fn init(boot_info: BootInfo) -> KernelInformation {
    let kernel_info = KernelInformation::new(boot_info);
    //logger::init(framebuffer);
    interrupts::init_gdt();
    interrupts::init_idt();
    unsafe {
        // can cause undefined behaviour if the offsets were not set correctly
        interrupts::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
    let mut initialized_drivers = INITIALIZED_DRIVERS.lock();
    for (index, driver_generator) in REGISTERED_DRIVERS.lock().iter().enumerate() {
        if let Some(driver) = driver_generator {
            initialized_drivers[index] = Some(driver(kernel_info));
        } else {
            break;
        }
    }
    kernel_info
}

pub fn register_driver(registrator: Registrator) {
    for driver in REGISTERED_DRIVERS.lock().iter_mut() {
        if driver.is_none() {
            driver.replace(registrator);
            return;
        }
    }
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("{}", ansi_colors::Red("[failed]\n"));
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    hlt_loop();
}
