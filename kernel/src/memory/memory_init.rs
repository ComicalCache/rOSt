use crate::debug;
use bootloader::BootInfo;
use x86_64::VirtAddr;

use super::{
    frame_allocator::BitmapFrameAllocator,
    heap::init_heap,
    page_table::{self, MEMORY_MAPPER},
};

/// Initializes the page tables and kernel heap memory
pub fn init(boot_info: &'static BootInfo, allocator: &mut BitmapFrameAllocator) {
    let pmo = VirtAddr::new(
        boot_info
            .physical_memory_offset
            .into_option()
            .expect("physical memory mapping not set"),
    );
    unsafe { page_table::init(pmo) };
    let mut mapper = MEMORY_MAPPER.lock();
    init_heap(mapper.as_mut().unwrap(), allocator).expect("heap initialization failed");
    debug::log("Heap initialized");
}
