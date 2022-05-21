use core::arch::asm;

use utils::constants::MIB;
use x86_64::{
    structures::paging::{
        FrameAllocator, PageTable, PageTableFlags, PhysFrame, Size2MiB, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

use crate::{
    debug, interrupts::GDT, memory::FullFrameAllocator,
    structures::kernel_information::KernelInformation,
};

/// Initializes and returns the level-4 page table that maps memory for a user-mode process.
unsafe fn get_user_mode_mapping(
    pmo: u64,
    allocator: &mut FullFrameAllocator,
) -> Option<(PhysFrame, PhysAddr)> {
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

type UserModeFunction = extern "C" fn();

/// Jumps to the passed function, entering the user mode (Ring 3)
pub unsafe fn run_in_user_mode(
    function: UserModeFunction,
    kernel_info: &mut KernelInformation,
) -> ! {
    let function_pointer = function as *const () as *const u8;
    debug::log("Creating user mode mapping");
    let (user_page_map, user_physical_address) = get_user_mode_mapping(
        kernel_info.physical_memory_offset,
        &mut kernel_info.allocator,
    )
    .expect("Error while creating user mode mapping");

    let user_mode_code_address = 0x1000u64;

    let virtual_address = VirtAddr::new(
        user_physical_address.as_u64()
            + user_mode_code_address
            + kernel_info.physical_memory_offset,
    )
    .as_mut_ptr::<u8>();
    debug::log("Loading program");

    // TODO: loading the user mode function from e.g. an ELF file
    virtual_address.copy_from_nonoverlapping(function_pointer, 1024);

    x86_64::instructions::interrupts::disable();

    let virtual_address = user_mode_code_address as *mut u8;
    let code_selector = ((GDT.1.user_code_selector.index() * 8) | 3) as u64;
    let data_selector = ((GDT.1.user_data_selector.index() * 8) | 3) as u64;

    debug::log("Moving to user mode");

    asm!(
        "mov cr3, r10",
        "mov rax, 0",
        "push rax", // aligning the stack
        "push r14", // data selector
        "push r12", // user mode stack pointer
        "pushfq",
        "pop rax",
        "or eax, 0x200",
        "and eax, 0xffffbfff",
        "push rax", // eflags
        "push r13", // code selector (ring 3 code with bottom 2 bits set for ring 3)
        "push r15", // instruction address to return to
        // TODO: Should probably zero all the registers before running user program to avoid data leaks
        "iretq",
        in("r10") (user_page_map.start_address().as_u64()),
        // TODO: better user mode stack pointer
        in("r12") (virtual_address.add(2 * MIB as usize - 4096)), // For now we only use the first 2MiB page
        in("r13") (code_selector),
        in("r14") (data_selector),
        in("r15") (virtual_address),
        options(noreturn)
    );
}
