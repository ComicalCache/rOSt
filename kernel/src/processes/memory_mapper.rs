use alloc::sync::Arc;
use internal_utils::FullFrameAllocator;
use spin::Mutex;
use x86_64::{
    structures::paging::{PageTable, PageTableFlags, PhysFrame, Size2MiB, Size4KiB},
    PhysAddr,
};

use crate::debug;

/// Initializes and returns the level-4 page table that maps memory for a user-mode process.
pub unsafe fn get_user_mode_mapping(
    pmo: u64,
    allocator: Arc<Mutex<dyn FullFrameAllocator>>,
) -> Option<(PhysFrame, PhysAddr)> {
    debug::log("Creating user mode mapping");
    let mut allocator = allocator.lock();
    let level_4_frame: PhysFrame<Size4KiB> = allocator.allocate_frame()?;
    let level_3_frame: PhysFrame<Size4KiB> = allocator.allocate_frame()?;
    let level_2_frame: PhysFrame<Size4KiB> = allocator.allocate_frame()?;

    let page_table_flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    let user_page_table_flags = page_table_flags | PageTableFlags::USER_ACCESSIBLE;

    let level_4_table_address = level_4_frame.start_address();
    let level_3_table_address = level_3_frame.start_address();
    let level_2_table_address = level_2_frame.start_address();
    // Just take the mapping from the bootloader's page tables
    let (level_2_kernel_data_table_address, level_2_kernel_stack_table_address) =
        get_kernel_data_and_stack_level_2_table_addresses(pmo);

    let level_4_table = (level_4_table_address.as_u64() + pmo) as *mut PageTable;
    let level_4_table = level_4_table.as_mut().unwrap();
    // Mapping 0x0000_0000_0000 to level 3 table
    level_4_table[0].set_addr(level_3_table_address, user_page_table_flags);

    let level_3_table = (level_3_table_address.as_u64() + pmo) as *mut PageTable;
    let level_3_table = level_3_table.as_mut().unwrap();
    // Mapping 0x0000_0000_0000 to level 2 table
    level_3_table[0].set_addr(level_2_table_address, user_page_table_flags);
    // Mapping 0x007F_8000_0000 to kernel stack
    level_3_table[510].set_addr(level_2_kernel_stack_table_address, page_table_flags);
    // Mapping 0x007F_C000_0000 to kernel data
    level_3_table[511].set_addr(level_2_kernel_data_table_address, page_table_flags);

    let level_2_table = (level_2_table_address.as_u64() + pmo) as *mut PageTable;
    // Mapping level 2 entries to 2mb frames
    let level_2_table = level_2_table.as_mut().unwrap();
    level_2_table
        .iter_mut()
        .take(8) // We're mapping 16mb for now, e.g. 0x0100_0000
        .for_each(|entry| {
            let frame: PhysFrame<Size2MiB> = allocator
                .allocate_frame()
                .expect("Failed to allocate user process frame");
            entry.set_addr(
                frame.start_address(),
                PageTableFlags::HUGE_PAGE | user_page_table_flags,
            );
        });

    Some((level_4_frame, level_2_table[0].addr()))
}

unsafe fn get_kernel_data_and_stack_level_2_table_addresses(pmo: u64) -> (PhysAddr, PhysAddr) {
    use x86_64::registers::control::Cr3;
    let level4 = (Cr3::read().0.start_address().as_u64() + pmo) as *const PageTable;
    let level4 = level4.as_ref().unwrap();

    let level3 = ((level4[0].addr().as_u64() + pmo) as *const PageTable)
        .as_ref()
        .unwrap();
    (level3[511].addr(), level3[510].addr())
}
