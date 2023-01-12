use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;

use super::allocator::{align_up, Locked};
use crate::serial_println;

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // get lock of inner allocator
        let mut allocator = self.lock();

        // get start of allocation, pointing at head
        let alloc_start = align_up(allocator.next, layout.align());

        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };

        if alloc_end > allocator.heap_end {
            ptr::null_mut()
        } else {
            allocator.next = alloc_end;
            allocator.allocations += 1;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut allocator = self.lock();

        allocator.allocations -= 1;
        if allocator.allocations == 0 {
            allocator.next = allocator.heap_start;
        }
    }
}
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    /// Initialize allocator with given heap bounds
    ///
    /// # Safety
    ///
    /// Marked as unsafe because the caller must ensure the given
    /// memory range is unused. Method should only be called once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        serial_println!("Initializing BumpAllocator heap allocator");

        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}
