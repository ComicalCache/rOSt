use crate::debug;
use bootloader::BootInfo;
use x86_64::VirtAddr;

use super::{
    frame_allocator::FullFrameAllocator,
    heap::init_heap,
    page_table::{self, MEMORY_MAPPER},
};

/// Initializes the page tables and kernel heap memory
pub fn init(boot_info: &'static BootInfo) {
    let pmo = VirtAddr::new(
        boot_info
            .physical_memory_offset
            .into_option()
            .expect("physical memory mapping not set"),
    );
    unsafe { page_table::init(pmo) };
    let mut frame_allocator = unsafe { FullFrameAllocator::init(&boot_info.memory_regions) };
    let mut mapper = MEMORY_MAPPER.lock();
    init_heap(mapper.as_mut().unwrap(), &mut frame_allocator).expect("heap initialization failed");
    debug::log("Heap initialized");
}
