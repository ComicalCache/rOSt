#![no_std] // no standard library
#![no_main]
#![allow(incomplete_features)]
#![feature(
    custom_test_frameworks,
    abi_x86_interrupt,
    generic_const_exprs,
    core_intrinsics,
    alloc_error_handler
)]
#![test_runner(test_framework::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::{arch::asm, panic::PanicInfo};
use kernel::structures::kernel_information::KernelInformation;
use test_framework::serial_println;
use tinytga::RawTga;
use utils::constants::MIB;
use vga::vga_core::{Clearable, ImageDrawable};

use core::alloc::Layout;

entry_point!(kernel);
pub fn kernel(boot_info: &'static mut BootInfo) -> ! {
    let mut kernel_info = kernel::init(boot_info);
    bootup_sequence(kernel_info);

    #[cfg(test)]
    kernel_test(kernel_info);
    #[cfg(not(test))]
    kernel_main(&mut kernel_info);

    kernel::hlt_loop();
}

fn bootup_sequence(kernel_info: KernelInformation) {
    kernel::register_driver(vga::driver_init);
    kernel::register_driver(ata::driver_init);
    kernel::reload_drivers(kernel_info);
    let data = include_bytes!("./assets/rost-logo.tga");
    let logo = RawTga::from_slice(data).unwrap();
    let logo_header = logo.header();
    let mut vga_device = vga::vga_device::VGADeviceFactory::from_kernel_info(kernel_info);
    vga_device.clear(vga::vga_color::BLACK);
    vga_device.draw_image(
        (vga_device.width as u16 - logo_header.width) / 2,
        (vga_device.height as u16 - logo_header.height) / 2,
        &logo,
    );
}

#[no_mangle]
extern "C" fn user_mode_check_1() {
    let mut i = 1;
    loop {
        i += 1;
        if i > 10_000_000 {
            syscall(1, 0, 0);
            i = 1;
        }
    }
}

#[no_mangle]
extern "C" fn user_mode_check_2() {
    let mut i = 1;
    loop {
        i += 1;
        if i > 10_000_000 {
            syscall(2, 0, 0);
            i = 1;
        }
    }
}

#[inline(always)]
fn syscall(rdi: u64, rsi: u64, rdx: u64) {
    unsafe {
        asm!(
            "push r10; push rcx",
            "push rdi; push rsi; push rdx",
            "syscall", 
            "pop rdx; pop rsi; pop rdi",
            "pop rcx; pop r10",
            in("rdi")(rdi), 
            in("rsi")(rsi), 
            in("rdx")(rdx));
    }
}

pub fn kernel_main(kernel_info: &mut KernelInformation) {
    use kernel::processes::{add_process, run_processes, Process, Thread};

    let process1 = add_process(Process::new(user_mode_check_1, *kernel_info, 1));
    let thread1 = Thread::new(0x1000, 2 * MIB, process1);

    let process2 = add_process(Process::new(user_mode_check_2, *kernel_info, 2));
    let thread2 = Thread::new(0x1000, 2 * MIB, process2);

    run_processes();
    serial_println!("Something went wrong");
    /*
        let test = Box::new(4);
        log_println!("New boxed value: {:#?}", test);
        log_println!("im not dying :)");
    */
    /*
        log_println!("Getting all disks...");
        let disks = ata::get_all_disks();
        log_println!("Got {} disks, taking the non-bootable one...", disks.len());
        let mut disk = disks
            .into_iter()
            .map(|mut disk| (disk.has_bootloader(), disk))
            .find(|(boot, _)| !boot.unwrap_or(true))
            .expect("No non-bootable disk found")
            .1;
        log_println!("Got a disk, looking for partitions...");
        let mut partitions = disk.get_partitions().expect("Error getting partitions");
        if partitions.len() == 0 {
            log_println!("No partitions found, creating a new one...");
            let partition_size = disk.descriptor.lba_48_addressable_sectors as u32 / 2;
            disk.create_partition(partition_size, 0xED)
                .expect("Error creating partition");
            log_println!("Partition created, double-checking...");
            partitions = disk.get_partitions().expect("Error getting partitions");
            if partitions.len() == 0 {
                log_println!("No partitions found, giving up.");
                return;
            }
        }
        log_println!("Found {} partitions:", partitions.len());
        for partition in partitions {
            log_println!(
                "{:8} - starting at {:8X}",
                format_size(partition.descriptor.sectors * 512),
                partition.descriptor.start_lba
            )
        }
    */
}

/// Panic handler for the OS.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::panic_handler(info);
}

/// This is the main function for tests.
#[cfg(test)]
pub fn kernel_test(_kernel_info: KernelInformation) {
    test_main();
}

/// Panic handler for the OS in test mode.
#[cfg(test)]
#[panic_handler]
// this function is called if a panic occurs and it is a test, all output is redirected to the serial port
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info);
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
