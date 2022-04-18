#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_core::test_framework::test_runner)]
#![feature(abi_x86_interrupt)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use lazy_static::lazy_static;

use bootloader::{entry_point, BootInfo};
use os_core::{
    ansi_colors::{Green, Red, Yellow},
    serial_print, serial_println,
    test_framework::qemu_exit::{exit_qemu, QemuExitCode},
};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_core::test_panic_handler(info)
}

entry_point!(kernel_start);
#[no_mangle]
pub fn kernel_start(_boot_info: &'static mut BootInfo) -> ! {
    serial_println!("{} 1 {}", Yellow("Running"), Yellow("test(s):"));

    serial_print!("page_fault::page_fault_test...\t");
    os_core::gdt::init_gdt();
    init_test_idt();

    // should throw a page fault with protection violation
    unsafe {
        *(0x205017 as *mut u64) = 42;
    }

    loop {}
}

// create custom IDT for this test
lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.page_fault.set_handler_fn(test_page_fault_handler);
        idt
    };
}

// sends the successful exit code on trigger
extern "x86-interrupt" fn test_page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    // ? idk if this is reliable to test like this, maybe can be ignored if this test fails
    if stack_frame.instruction_pointer.as_u64() != 0x204F17
        || error_code
            != PageFaultErrorCode::PROTECTION_VIOLATION | PageFaultErrorCode::CAUSED_BY_WRITE
    {
        serial_print!("{}\n", Red("[failed]"));
        exit_qemu(QemuExitCode::Failed);
    }

    serial_print!("{}\n", Green("[ok]"));
    exit_qemu(QemuExitCode::Success);
}

fn init_test_idt() {
    TEST_IDT.load();
}
