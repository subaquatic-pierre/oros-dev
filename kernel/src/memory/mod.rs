use alloc::alloc::{GlobalAlloc, Layout};
use bootloader_api::info::{MemoryRegionKind, MemoryRegions};

use core::ptr::null_mut;
use linked_list_allocator::LockedHeap;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};
use x86_64::{
    structures::paging::{OffsetPageTable, PageTable, PhysFrame},
    PhysAddr,
};

pub mod allocator;
pub mod bump;
pub mod fixed;
pub mod frame;
pub mod linked_list;

use allocator::{ALLOCATOR, HEAP_SIZE, HEAP_START};

/// Initialize new OffsetPageTable
/// # Safety
///
/// Need to be unsafe
pub unsafe fn init(phys_mem_offset: VirtAddr) -> OffsetPageTable<'static> {
    let lvl_4_table = active_lvl_4_table(phys_mem_offset);
    OffsetPageTable::new(lvl_4_table, phys_mem_offset)
}

/// Returns mutable address to active level 4 table
/// # Safety
///
/// raw pointers need usafe actions
unsafe fn active_lvl_4_table(physical_mem_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (lvl_4_table_frame, _) = Cr3::read();

    let phys = lvl_4_table_frame.start_address();
    let virt_addr = physical_mem_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt_addr.as_mut_ptr();

    &mut *page_table_ptr //unsafe
}

/// Create allocation frames for heap memory
pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    // get range of pages to be used for heap memory
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    // allocate frames to each page in page range, size of page is 4KiB
    for page in page_range {
        // create frame for addresses available in frame allocator
        // BootInfoFrameAllocator is initial entry point for Frame allocations

        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        // map physical frame to heap pages
        // update TLB (translation look aside buffer) after successful map
        // of each frame, with call to flush
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}
