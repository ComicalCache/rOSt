use alloc::vec::Vec;
use bootloader::BootInfo;
use lazy_static::lazy_static;
use spin::Mutex;
use test_framework::serial_println;

use crate::{
    interrupts::{self, syscalls::register_syscall},
    memory,
    processes::get_scheduler,
    structures::{
        driver::{Driver, Registrator},
        kernel_information::KernelInformation,
    },
};

use crate::debug;

lazy_static! {
    static ref REGISTERED_DRIVERS: Mutex<Vec<Registrator>> = Mutex::new(Vec::new());
    static ref INITIALIZED_DRIVERS: Mutex<Vec<Driver>> = Mutex::new(Vec::new());
}

extern "C" fn test_syscall(a: u64, b: u64) -> u64 {
    let thr = get_scheduler().running_thread.clone().unwrap();
    let thread = thr.as_ref().borrow();
    serial_println!(
        "Syscall 0 from process {} and thread {}",
        thread.process.as_ref().borrow().id,
        thread.id
    );
    0
}

extern "C" fn test_syscall2(a: u64, b: u64) -> u64 {
    let thr = get_scheduler().running_thread.clone().unwrap();
    let thread = thr.as_ref().borrow();
    serial_println!(
        "Syscall 1 from process {} and thread {}",
        thread.process.as_ref().borrow().id,
        thread.id
    );
    1
}

/// Initialises the components of the OS, **must** be called before any other functions.
pub fn init(boot_info: &'static BootInfo) -> KernelInformation {
    debug::print_memory_map(&boot_info.memory_regions);
    memory::save_kernel_memory();
    memory::init(boot_info);
    let kernel_info = KernelInformation::new(boot_info);
    interrupts::reload_gdt();
    interrupts::init_idt();
    interrupts::syscalls::setup_syscalls();
    interrupts::enable();

    register_syscall(0, test_syscall);
    register_syscall(1, test_syscall2);

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

/// Endless loop calling halt continuously.
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
