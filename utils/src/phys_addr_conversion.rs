use x86_64::{
    structures::paging::{PageTable, PageTableFlags, PhysFrame, Size2MiB},
    PhysAddr, VirtAddr,
};

pub trait ToPhysAddr {
    fn to_phys_address(&self, physical_memory_offset: u64) -> PhysAddr;
}

impl ToPhysAddr for VirtAddr {
    fn to_phys_address(&self, physical_memory_offset: u64) -> PhysAddr {
        let p4i = self.p4_index();
        let p3i = self.p3_index();
        let p2i = self.p2_index();
        let p1i = self.p1_index();
        let offset = self.page_offset();

        let (p4_frame, _) = x86_64::registers::control::Cr3::read();
        unsafe {
            let p4 = ((p4_frame.start_address().as_u64() + physical_memory_offset)
                as *const PageTable)
                .as_ref()
                .unwrap();
            let p3 = ((p4[p4i].addr().as_u64() + physical_memory_offset) as *const PageTable)
                .as_ref()
                .unwrap();
            let p2 = ((p3[p3i].addr().as_u64() + physical_memory_offset) as *const PageTable)
                .as_ref()
                .unwrap();
            let p2_entry = &p2[p2i];
            // If the P2 is a huge page then it maps 2MiB of memory and we don't have a P1
            if p2_entry.flags().contains(PageTableFlags::HUGE_PAGE) {
                let frame = (p2_entry.addr().as_u64() as *const PhysFrame<Size2MiB>)
                    .as_ref()
                    .unwrap();
                PhysAddr::new(
                    frame.start_address().as_u64() + ((u64::from(p1i) << 9) | u64::from(offset)),
                )
            } else {
                let p1 = ((p2_entry.addr().as_u64() + physical_memory_offset) as *const PageTable)
                    .as_ref()
                    .unwrap();
                let maybe_frame = p1[p1i].frame();
                let frame = maybe_frame.as_ref().unwrap();
                PhysAddr::new(frame.start_address().as_u64() + u64::from(offset))
            }
        }
    }
}

pub trait KernelConverter {
    fn to_kernel_address(&self, physical_memory_offset: u64, kernel_virtual_address: u64) -> &Self;
}

impl<T> KernelConverter for T {
    fn to_kernel_address(&self, physical_memory_offset: u64, kernel_virtual_address: u64) -> &Self {
        let phys_addr =
            VirtAddr::from_ptr(self as *const Self).to_phys_address(physical_memory_offset);
        unsafe {
            ((phys_addr.as_u64() + kernel_virtual_address) as *const Self)
                .as_ref()
                .unwrap()
        }
    }
}
