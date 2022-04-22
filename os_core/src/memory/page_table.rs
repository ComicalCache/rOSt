use bootloader::boot_info::{MemoryRegionKind, MemoryRegions};
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::structures::paging::{Mapper, Page, PageTableFlags};
use x86_64::{
    structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

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
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid. The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
    pub unsafe fn init(memory_map: &'static MemoryRegions) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        self.memory_map
            .iter()
            .filter(|r| r.kind == MemoryRegionKind::Usable)
            // map each region to its address range
            .map(|r| r.start..r.end)
            // transform to an iterator of frame start addresses
            .flat_map(|r| r.step_by(4096))
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    /// Returns the next usable frame
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        // we need to call usable_frames every time because rust does not yet support impl Iterators
        // as struct fields and we dont have Vec<> yet to cache it either
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

pub fn create_mapping(
    page: Page,
    address: u64,
    flags: PageTableFlags,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let frame = PhysFrame::containing_address(PhysAddr::new(address));

    let map_to_result = unsafe {
        // FIXME: this is not safe, we do it only for testing
        // TODO: add checking for frame not in use

        // hangs in an endless loop :(
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}
