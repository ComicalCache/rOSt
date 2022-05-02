use core::arch::asm;

use lazy_static::lazy_static;
use x86_64::registers::segmentation::{SegmentSelector, DS, ES, SS};
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};
use x86_64::structures::paging::{Page, PageTableFlags};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

use crate::debug;
use crate::memory::create_mapping;
use crate::structures::kernel_information::KernelInformation;

/// the interrupt stack table index of the stack used for double faults
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    /// The TSS of the OS.
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        const STACK_SIZE: usize = 4096;
        #[repr(align(16))]
        struct Stack([u8; STACK_SIZE]);

        // Stack used when an exception happens in user mode
        tss.privilege_stack_table[0] = {

            static mut STACK: Stack = Stack([0; STACK_SIZE]);

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });

            // returns the highest address of the stack because the stack grows downwards
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };

        // set the interrupt stack table to the appropriate address
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {

            static mut STACK: Stack = Stack([0; STACK_SIZE]);

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });

            // returns the highest address of the stack because the stack grows downwards
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };

        tss
    };

    /// The GDT used by the OS.
    pub static ref GDT: (GlobalDescriptorTable, Selectors, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();

        let kernel_code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let kernel_data_selector = gdt.add_entry(Descriptor::kernel_data_segment());

        let user_data_selector = gdt.add_entry(Descriptor::user_data_segment());
        let user_code_selector = gdt.add_entry(Descriptor::user_code_segment());

        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));

        (
            gdt,
            Selectors {
                code_selector: kernel_code_selector,
                data_selector: kernel_data_selector,
                tss_selector: tss_selector
            },
            Selectors {
                code_selector: user_code_selector,
                data_selector: user_data_selector,
                tss_selector: tss_selector
            },
        )
    };
}

pub struct Selectors {
    pub code_selector: SegmentSelector,
    pub data_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

/// Initialises the GDT and TSS.
pub fn reload_gdt() {
    use x86_64::instructions::segmentation::{Segment, CS};
    use x86_64::instructions::tables::load_tss;
    debug::log("Loading GDT and segment registers");
    GDT.0.load();
    debug::log("GDT loaded");
    let selector = &GDT.1;
    unsafe {
        CS::set_reg(selector.code_selector);
        load_tss(selector.tss_selector);
        SS::set_reg(selector.data_selector);
        DS::set_reg(selector.data_selector);
        ES::set_reg(selector.data_selector);
    }
    debug::log("Segment registers loaded");
}

type UserModeFunction = extern "C" fn();

/// Jumps to the passed function, entering the user mode (Ring 3)
pub unsafe fn run_in_user_mode(function: UserModeFunction, kernel_info: KernelInformation) -> ! {
    let function_pointer = function as *const () as *const u8;
    let virtual_address_u64 = 0x_3333_AAAA_0000u64;
    let virtual_address = VirtAddr::new(virtual_address_u64); // User space code address
    create_mapping(
        Page::containing_address(virtual_address),
        None,
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE,
        kernel_info,
    );
    let virtual_address = virtual_address.as_mut_ptr::<u8>();
    virtual_address.copy_from_nonoverlapping(function_pointer, 1024);

    let code_selector = ((GDT.2.code_selector.index() * 8) | 3) as u64;
    let data_selector = ((GDT.2.data_selector.index() * 8) | 3) as u64;
    x86_64::instructions::interrupts::disable();
    debug::log("Moving to user mode");
    asm!(
        "mov rax, 0",
        "push rax",         // aligning the stack
        "push r14",         // data selector
        "push r12",         // user mode stack pointer
        "pushf",
        "pop rax",
        "or eax, 0x200",
        "and eax, 0xffffbfff",
        "push rax",            // eflags
        "push r13",         // code selector (ring 3 code with bottom 2 bits set for ring 3)
        "push r15",         // instruction address to return to
        "iretq",
        in("r12") (virtual_address.add(0x1000)),
        in("r13") (code_selector),
        in("r14") (data_selector),
        in("r15") (virtual_address),
    );
    unreachable!();
}
