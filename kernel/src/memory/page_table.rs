use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{Mapper, Page, PageTableFlags, Size2MiB};
use x86_64::{
    structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame},
    PhysAddr, VirtAddr,
};

use crate::structures::kernel_information::KernelInformation;

use super::frame_allocator::FullFrameAllocator;

lazy_static! {
    pub static ref MEMORY_MAPPER: Mutex<Option<OffsetPageTable<'static>>> = Mutex::new(None);
}

/// Initialize a new OffsetPageTable.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn init(physical_memory_offset: VirtAddr) {
    let level_4_table = active_level_4_table(physical_memory_offset);
    let _ = MEMORY_MAPPER
        .lock()
        .insert(OffsetPageTable::new(level_4_table, physical_memory_offset));
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

/// Maps a given virtual page to a given physical address. If a physical address is not given, a frame will be allocated from the FrameAllocator.
#[allow(dead_code)]
pub fn create_mapping(
    page: Page<Size2MiB>,
    address: Option<PhysAddr>,
    flags: PageTableFlags,
    kernel_info: KernelInformation,
) {
    let mut frame_allocator = unsafe { FullFrameAllocator::init(kernel_info.memory_regions) };
    let mut mapper = MEMORY_MAPPER.lock();
    let frame = if let Some(address) = address {
        PhysFrame::<Size2MiB>::containing_address(address)
    } else {
        frame_allocator
            .allocate_frame()
            .expect("No more frames available")
    };

    let map_to_result = unsafe {
        mapper
            .as_mut()
            .unwrap()
            .map_to(page, frame, flags, &mut frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}
