use bootloader::boot_info::{MemoryRegionKind, MemoryRegions};
use spin::Mutex;
use x86_64::{
    structures::paging::{FrameAllocator, PageSize, PhysFrame},
    PhysAddr,
};

/// A FrameAllocator that returns usable frames from the bootloader's memory map.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FullFrameAllocator {
    memory_map: &'static MemoryRegions,
}

static FRAME_NEXT: Mutex<usize> = Mutex::new(0);
static FRAME_REGION: Mutex<usize> = Mutex::new(usize::MAX);

impl FullFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// ## Safety
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid. The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
    pub unsafe fn init(memory_map: &'static MemoryRegions) -> Self {
        let mut region_index = FRAME_REGION.lock();

        // initialise FRAME_REGION with the first usable region
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
