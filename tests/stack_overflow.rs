#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_framework::test_runner)]
#![feature(abi_x86_interrupt)]
#![reexport_test_harness_main = "test_main"]

use core::ops::{Deref, DerefMut};
use core::panic::PanicInfo;
use lazy_static::lazy_static;

use bootloader::{entry_point, BootInfo};
use kernel::{
    ansi_colors::{Green, Yellow},
    serial_print, serial_println,
    test_framework::qemu_exit::{exit_qemu, QemuExitCode},
};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

entry_point!(kernel_start);
#[no_mangle]
pub fn kernel_start(_boot_info: &'static mut BootInfo) -> ! {
    serial_println!("{} 1 {}", Yellow("Running"), Yellow("test(s):"));

    serial_print!("stack_overflow::stack_overflow_test...\t");
    kernel::gdt::init_gdt();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

// temporary struct to use as volatile dummy
#[derive(Clone, Copy)]
struct Dummy {}
impl Deref for Dummy {
    type Target = Dummy;

    fn deref(&self) -> &Self::Target {
        self
    }
}
impl DerefMut for Dummy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(Dummy {}).read(); // prevent tail recursion optimizations
}

// create custom IDT for this test to send the successful exit code on trigger
lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(kernel::interrupts::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

// sends the successful exit code on trigger
extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("{}", Green("[ok]"));
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn init_test_idt() {
    TEST_IDT.load();
}
