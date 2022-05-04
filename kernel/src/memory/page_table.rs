use bootloader::boot_info::{MemoryRegionKind, MemoryRegions};
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::structures::paging::{Mapper, Page, PageSize, PageTableFlags, Size2MiB};
use x86_64::{
    structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame},
    PhysAddr, VirtAddr,
};

use crate::structures::kernel_information::KernelInformation;

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
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

/// A FrameAllocator that returns usable frames from the bootloader's memory map.
pub struct FullFrameAllocator {
    memory_map: &'static MemoryRegions,
}

static FRAME_NEXT: Mutex<usize> = Mutex::new(0);
static FRAME_REGION: Mutex<usize> = Mutex::new(usize::MAX);

impl FullFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid. The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
    pub unsafe fn init(memory_map: &'static MemoryRegions) -> Self {
        let mut region_index = FRAME_REGION.lock();
        if *region_index == usize::MAX {
            *region_index = memory_map
                .iter()
                .enumerate()
                .find(|(_, region)| region.kind == MemoryRegionKind::Usable)
                .expect("No usable memory found")
                .0;
        }
        FullFrameAllocator { memory_map }
    }
}

unsafe impl<S> FrameAllocator<S> for FullFrameAllocator
where
    S: PageSize,
{
    /// Returns the next usable frame
    fn allocate_frame(&mut self) -> Option<PhysFrame<S>> {
        let mut next = FRAME_NEXT.lock();
        let mut region_index = FRAME_REGION.lock();
        // Get the current region
        let mut region = self.memory_map.get(*region_index)?;
        // If we don't have any more pages left in the region...
        if region.end - region.start < ((*next as u64) + 1) * S::SIZE {
            // Find the index of a next Usable region
            *region_index = self
                .memory_map
                .iter()
                .enumerate()
                .filter(|(i, _)| i > &region_index)
                .find(|(_, region)| region.kind == MemoryRegionKind::Usable)
                .map(|(i, _)| i)?;
            // Use the new region
            *next = 0;
            region = self.memory_map.get(*region_index)?;
        }
        // According to the Rust community, step_by+nth should be O(1)
        let phys_frame = (region.start..region.end)
            .step_by(S::SIZE as usize)
            .nth(*next)
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))?;
        *next += 1;
        Some(phys_frame)
    }
}

/// Maps a given virtual page to a given physical address. If a physical address is not given, a frame will be allocated from the FrameAllocator.
pub fn create_mapping(
    page: Page<Size2MiB>,
    address: Option<u64>,
    flags: PageTableFlags,
    kernel_info: KernelInformation,
) {
    let mut frame_allocator = unsafe { FullFrameAllocator::init(kernel_info.memory_regions) };
    let mut mapper = MEMORY_MAPPER.lock();
    let frame = if let Some(address) = address {
        PhysFrame::<Size2MiB>::containing_address(PhysAddr::new(address))
    } else {
        frame_allocator
            .allocate_frame()
            .expect("No more frames available")
    };

    let map_to_result = unsafe {
        // TODO: add checking for frame not in use
        mapper
            .as_mut()
            .unwrap()
            .map_to(page, frame, flags, &mut frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}
