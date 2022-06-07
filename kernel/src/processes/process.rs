use core::cell::RefCell;

use crate::debug;
use crate::processes::memory_mapper::get_user_mode_mapping;
use crate::processes::ProcessFunction;
use alloc::rc::Rc;
use internal_utils::get_current_tick;
use internal_utils::structures::kernel_information::KernelInformation;
use x86_64::{PhysAddr, VirtAddr};

use alloc::vec::Vec;

use super::Thread;

#[derive(Debug)]
pub struct Process {
    /// The process's ID.
    pub id: u64,
    /// The page table the process is using.
    pub cr3: PhysAddr,
    /// Total ticks the process has been running for.
    pub total_ticks: u64,
    /// The tick the process has been created on.
    pub start_tick: u64,
    /// The tick the process has been last ran on.
    pub last_tick: u64,
    /// Is the process a kernel process (should it run in ring 0 or 3?).
    pub kernel_process: bool,
    /// The threads of the process.
    pub threads: Vec<Rc<RefCell<Thread>>>,
}

impl Process {
    /// Returns the percentage of ticks the process spent running, calculated from the creation time of the process
    pub fn tick_density(&self, current_tick: u64) -> u64 {
        let ticks_maximum = current_tick - self.start_tick;
        self.total_ticks * 100 / ticks_maximum
    }

    /// Creates a new process from a function pointer.
    ///
    // TODO: loading the process from e.g. an ELF file
    // We have to look up the structure of an ELF file and prepare the user memory mapping according to it.
    // Then we can load the program and it's data to proper places and create a process out of it.
    pub fn new(function: ProcessFunction, kernel_info: KernelInformation, id: u64) -> Self {
        let function_pointer = function as *const () as *const u8;
        unsafe {
            let (user_page_map, user_physical_address) = get_user_mode_mapping(
                kernel_info.physical_memory_offset,
                kernel_info.allocator.clone(),
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

            virtual_address.copy_from_nonoverlapping(function_pointer, 1024);

            Process {
                id,
                cr3: user_page_map.start_address(),
                total_ticks: 0,
                start_tick: get_current_tick(),
                last_tick: 0,
                kernel_process: false,
                threads: Vec::new(),
            }
        }
    }
}
