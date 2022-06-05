mod allocator;
pub mod frame_allocator;
mod heap;
mod memory_init;
mod page_table;
pub use memory_init::init;
pub use page_table::MEMORY_MAPPER;

use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::{
    registers::control::{Cr3, Cr3Flags},
    structures::paging::PhysFrame,
    PhysAddr,
};

lazy_static! {
    static ref KERNEL_CR3: Mutex<PhysAddr> = Mutex::new(PhysAddr::new(0));
}

/// Saves the current paging table used as the kernel's paging table.
pub(crate) fn save_kernel_memory() {
    *KERNEL_CR3.lock() = x86_64::registers::control::Cr3::read().0.start_address();
}

/// Switches the paging table used to the kernel's paging table.
pub(crate) fn switch_to_kernel_memory() {
    let kernel_cr3 = *KERNEL_CR3.lock();
    if !kernel_cr3.is_null() {
        unsafe {
            Cr3::write(
                PhysFrame::from_start_address_unchecked(kernel_cr3),
                Cr3Flags::empty(),
            );
        }
    }
}

/// Performs an action while having kernel paging table. Then switches back.
pub(crate) fn with_kernel_memory<T, V>(action: T) -> V
where
    T: Fn() -> V,
{
    // save the current paging table
    let cr3 = Cr3::read().0.start_address();
    switch_to_kernel_memory();
    let result = action();

    // restore the paging table
    unsafe {
        Cr3::write(
            PhysFrame::from_start_address_unchecked(cr3),
            Cr3Flags::empty(),
        )
    };
    result
}
