use super::allocator::Locked;
use alloc::alloc::{GlobalAlloc, Layout};

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // get lock of inner allocator
        let mut allocator = self.lock();

        // get start of allocation, pointing at head
        let alloc_start = allocator.next;
        allocator.next = alloc_start + layout.size();
        allocator.allocations += 1;
        alloc_start as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}
pub struct BumpAllocator {
    heap_start: usize,
    heap_size: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_size: 0,
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
        self.heap_start = heap_start;
        self.heap_size = heap_size;
        self.next = heap_start;
    }
}
