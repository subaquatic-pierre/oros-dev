use core::alloc::{GlobalAlloc, Layout};
use core::mem;
use core::ptr::{self, NonNull};

use crate::serial_println;

use super::allocator::Locked;

unsafe impl GlobalAlloc for Locked<FixedSizeAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();

        match FixedSizeAllocator::list_index(&layout) {
            Some(index) => {
                match allocator.list_heads[index].take() {
                    Some(node) => {
                        allocator.list_heads[index] = node.next.take();
                        node as *mut ListNode as *mut u8
                    }
                    None => {
                        // no list exists for that size => allocate new ListNode to the list
                        let block_size = BLOCK_SIZES[index];
                        // only works if size is power of two
                        let block_align = block_size;

                        let layout = Layout::from_size_align(block_size, block_align).unwrap();

                        allocator.fallback_alloc(layout)
                    }
                }
            }
            None => allocator.fallback_alloc(layout),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut allocator = self.lock();

        match FixedSizeAllocator::list_index(&layout) {
            Some(index) => {
                let new_node = ListNode {
                    next: allocator.list_heads[index].take(),
                };

                // verify that block has size and alignment
                assert!(mem::size_of::<ListNode>() <= BLOCK_SIZES[index]);
                assert!(mem::align_of::<ListNode>() <= BLOCK_SIZES[index]);

                let new_node_ptr = ptr as *mut ListNode;

                new_node_ptr.write(new_node);
                allocator.list_heads[index] = Some(&mut *new_node_ptr)
            }
            None => {
                let ptr = NonNull::new(ptr).unwrap();
                allocator.fallback_allocator.deallocate(ptr, layout)
            }
        }
    }
}

pub struct ListNode {
    next: Option<&'static mut ListNode>,
}

/// Block sizes to use in bytes
///
/// We don’t define any block sizes smaller than 8 because
/// each block must be capable of storing 64-bit pointer
/// The sizes much be power of two because they are also used
/// as block alignment
pub const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

pub struct FixedSizeAllocator {
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
    fallback_allocator: linked_list_allocator::Heap,
}

impl FixedSizeAllocator {
    pub const fn new() -> Self {
        const EMPTY: Option<&'static mut ListNode> = None;
        Self {
            list_heads: [EMPTY; BLOCK_SIZES.len()],
            fallback_allocator: linked_list_allocator::Heap::empty(),
        }
    }

    /// Initialize allocator with given heap bounds
    ///
    /// # Safety
    ///
    /// Marked as unsafe because the caller must ensure the given
    /// memory range is unused. Method should only be called once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        serial_println!("Initializing FixedSize heap allocator");
        self.fallback_allocator.init(heap_start, heap_size)
    }

    /// Allocates using the fallback allocator
    fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
        match self.fallback_allocator.allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => ptr::null_mut(),
        }
    }

    /// Chose an appropriate block size given a layout
    ///
    /// Returns the index of block size in `BLOCK_SIZES`
    fn list_index(layout: &Layout) -> Option<usize> {
        let required_block_size = layout.size().max(layout.align());
        BLOCK_SIZES.iter().position(|&s| s >= required_block_size)
    }
}
