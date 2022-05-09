use core::mem;

use alloc::vec::Vec;
use bootloader::BootInfo;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::{
    interrupts, memory,
    structures::{
        driver::{Driver, Registrator},
        kernel_information::KernelInformation,
    },
};

#[cfg(debug_assertions)]
use crate::debug;

pub type SysCallHandlerFunc = extern "C" fn(*const u8);
lazy_static! {
    static ref REGISTERED_DRIVERS: Mutex<Vec<Registrator>> = Mutex::new(Vec::new());
    static ref INITIALIZED_DRIVERS: Mutex<Vec<Driver>> = Mutex::new(Vec::new());
    static ref SYSCALLS: Mutex<[SysCallHandlerFunc; 256]> =
        Mutex::new([unsafe { mem::transmute(0u8 as *const ()) }; 256]);
}

/// Initialises the components of the OS, **must** be called before any other functions.
pub fn init(boot_info: &'static BootInfo) -> KernelInformation {
    debug::print_memory_map(&boot_info.memory_regions);

    let kernel_info = KernelInformation::new(boot_info);

    memory::init(boot_info);
    interrupts::reload_gdt();
    interrupts::init_idt();
    interrupts::setup_syscalls();
    interrupts::enable();

    kernel_info
}

/// Reinitializes all the registered drivers
pub fn reload_drivers(kernel_info: KernelInformation) {
    let mut initialized_drivers = INITIALIZED_DRIVERS.lock();
    initialized_drivers.clear();
    initialized_drivers.extend(
        REGISTERED_DRIVERS
            .lock()
            .iter()
            .map(|registrator| registrator(kernel_info)),
    );
}

/// Registers a driver. After registering drivers call reload_drivers to initialize them.
pub fn register_driver(registrator: Registrator) {
    REGISTERED_DRIVERS.lock().push(registrator);
}

pub fn register_syscall(syscall_number: u8, handler: SysCallHandlerFunc) {
    SYSCALLS.lock()[syscall_number as usize] = handler;
}

/// Endless loop calling halt continuously.
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
