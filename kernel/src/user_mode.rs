use core::arch::asm;

use test_framework::serial_println;
use utils::{
    constants::MIB,
    phys_addr_conversion::{KernelConverter, ToPhysAddr},
};
use x86_64::{
    structures::paging::{FrameAllocator, PageTable, PageTableFlags, PhysFrame, Size4KiB},
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
) -> Option<(u64, PhysAddr)> {
    let frame1: PhysFrame<Size4KiB> = allocator.allocate_frame()?;
    let frame2: PhysFrame<Size4KiB> = allocator.allocate_frame()?;
    let frame3: PhysFrame<Size4KiB> = allocator.allocate_frame()?;
    let frame4: PhysFrame<Size4KiB> = allocator.allocate_frame()?;

    let page_table_flags =
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;

    let level_4_table_address = frame1.start_address().as_u64();
    let level_3_table_address = frame2.start_address().as_u64();
    let level_2_table_address = frame3.start_address().as_u64();
    let level_2_kernel_table_address = frame4.start_address().as_u64();

    let level_4_table = (level_4_table_address + pmo) as *mut PageTable;
    // Mapping 0x0000_0000_0000 to level 3 table
    level_4_table.as_mut().unwrap()[0]
        .set_addr(PhysAddr::new(level_3_table_address), page_table_flags);

    let level_3_table = (level_3_table_address + pmo) as *mut PageTable;
    // Mapping 0x0000_0000_0000 to level 2 table
    level_3_table.as_mut().unwrap()[0]
        .set_addr(PhysAddr::new(level_2_table_address), page_table_flags);
    // Mapping 0x007F_C000_0000 to kernel
    level_3_table.as_mut().unwrap()[511].set_addr(
        PhysAddr::new(level_2_kernel_table_address),
        page_table_flags,
    );

    let level_2_table = (level_2_table_address + pmo) as *mut PageTable;
    // Mapping level 2 entries to 2mb frames
    level_2_table
        .as_mut()
        .unwrap()
        .iter_mut()
        .enumerate()
        .take(8) // We're mapping 16mb for now, e.g. 0x0100_0000
        .for_each(|(i, entry)| {
            entry.set_addr(
                // Offsetting by 64mb of physical memory to go over the kernel data
                // TODO: Should be changed so it actually maps some free area
                PhysAddr::new((i as u64 * MIB * 2) + (64 * MIB)),
                PageTableFlags::HUGE_PAGE | page_table_flags,
            );
        });

    let level_2_kernel_table = (level_2_kernel_table_address + pmo) as *mut PageTable;
    // Mapping the kernel
    level_2_kernel_table
        .as_mut()
        .unwrap()
        .iter_mut()
        .enumerate()
        .take(32) // Taking first 64MB for the kernel
        .for_each(|(i, entry)| {
            entry.set_addr(
                PhysAddr::new(i as u64 * MIB * 2),
                PageTableFlags::HUGE_PAGE | PageTableFlags::PRESENT | PageTableFlags::GLOBAL,
            );
        });

    Some((level_4_table_address, PhysAddr::new(64 * MIB)))
}

type UserModeFunction = extern "C" fn();

/// Jumps to the passed function, entering the user mode (Ring 3)
pub unsafe fn run_in_user_mode(
    function: UserModeFunction,
    kernel_info: &mut KernelInformation,
) -> ! {
    let function_pointer = function as *const () as *const u8;
    debug::log("Creating user mode mapping");
    let (_user_page_map, user_physical_address) = get_user_mode_mapping(
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
    debug::log("Moving to user mode");

    let inner_function_mapped_addr = VirtAddr::from_ptr(
        run_in_user_mode_inner
            .to_kernel_address(kernel_info.physical_memory_offset, kernel_info.kernel_start),
    )
    .as_ptr::<u8>();
    serial_println!(
        "address was: {:X?}, now is: {:X?}",
        run_in_user_mode_inner as u64,
        inner_function_mapped_addr as u64
    );
    {
        let a = run_in_user_mode_inner as *const u8;
        let b = inner_function_mapped_addr;
        let c = (VirtAddr::from_ptr(run_in_user_mode as *const u8)
            .to_phys_address(kernel_info.physical_memory_offset)
            + kernel_info.physical_memory_offset)
            .as_u64() as *const u8;
        serial_println!("First bytes of the function:");
        for i in 0..8 {
            serial_println!(
                "{:X?} vs {:X?} vs {:X?}",
                a.add(i).read_volatile(),
                b.add(i).read_volatile(),
                c.add(i).read_volatile()
            );
        }
    }
    let virtual_address = user_mode_code_address as *mut u8;
    let code_selector = ((GDT.2.code_selector.index() * 8) | 3) as u64;
    let data_selector = ((GDT.2.data_selector.index() * 8) | 3) as u64;
    asm!(
        "jmp r11",
        in("r10") (_user_page_map),
        in("r11") (inner_function_mapped_addr),
        // TODO: better user mode stack pointer
        in("r12") (virtual_address.add(2 * MIB as usize - 4097)), // For now we only use the first 2MiB page
        in("r13") (code_selector),
        in("r14") (data_selector),
        in("r15") (virtual_address),
        options(noreturn)
    );
}

/// Runs the assembler script which switches the paging register, prepares the return stack, and jumps to user mode
#[naked]
unsafe extern "C" fn run_in_user_mode_inner() {
    asm!(
        //"mov cr3, r10",
        "mov rax, 0",
        "push rax", // aligning the stack
        "push r14", // data selector
        "push r12", // user mode stack pointer
        "pushf",
        "pop rax",
        "or eax, 0x200",
        "and eax, 0xffffbfff",
        "push rax", // eflags
        "push r13", // code selector (ring 3 code with bottom 2 bits set for ring 3)
        "push r15", // instruction address to return to
        "iretq",
        options(noreturn)
    );
}
