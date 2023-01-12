use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

/// A frame allocator that returns usable frames from bootloaders memory map
pub struct BootInfoFrameAllocator {
    mem_map: &'static MemoryMap,
    next: usize,
}

/// Main impl for Memory allocator
impl BootInfoFrameAllocator {
    /// Create FrameAllocator from the passed memory map
    /// # Safety
    ///
    /// Has to be unsafe
    /// The caller must ensure that passed memory map is valid
    pub unsafe fn init(mem_map: &'static MemoryMap) -> Self {
        Self { mem_map, next: 0 }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        //get usable regions from the memory map
        let regions = self.mem_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);

        // map each region to its address range
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());

        // transform to an iter of frame start addrs
        let frame_addrs = addr_ranges.flat_map(|r| r.step_by(4096));

        // create 'PhysFrame' from start addrs
        frame_addrs.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

/// Implement Frame allocator from bootloader crate
/// Allows creation of page memory mappings
/// Used for main memory allocation
unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
