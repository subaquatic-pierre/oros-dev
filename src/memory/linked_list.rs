use core::{
    alloc::{GlobalAlloc, Layout},
    mem, ptr,
};

use crate::{println, serial_println};

use super::allocator::{align_up, Locked};

unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let (size, align) = LinkedListAllocator::size_align(layout);
        let mut allocator = self.lock();

        if let Some((region, alloc_start)) = allocator.find_region(size, align) {
            let alloc_end = alloc_start.checked_add(size).expect("overflow");
            let excess_size = region.end_addr() - alloc_end;

            if excess_size > 0 {
                allocator.add_free_region(alloc_end, excess_size);
            }

            alloc_start as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // perform layout adjustments
        let (size, _) = LinkedListAllocator::size_align(layout);

        self.lock().add_free_region(ptr as usize, size)
    }
}

pub struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    const fn new(size: usize) -> Self {
        Self { size, next: None }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

pub struct LinkedListAllocator {
    head: ListNode,
}

impl LinkedListAllocator {
    /// Create an new empty LinkedListAllocator
    pub const fn new() -> Self {
        Self {
            head: ListNode::new(0),
        }
    }

    /// Initialize allocator with given heap bounds
    ///
    /// # Safety
    ///
    /// It is required that the called ensure that the bounds are valid
    /// Can only be called once
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        serial_println!("Initializing LinkedList heap allocator");
        self.add_free_region(heap_start, heap_size);
    }

    /// Create new region of available ListNodes
    /// Adds givin region to front of list
    ///
    /// # Safety
    ///
    /// Called from init method
    pub unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        // ensure freed retion is capable of holding ListNode
        assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);

        // create new list node, add to front of allocator
        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_ptr = addr as *mut ListNode;
        node_ptr.write(node);
        self.head.next = Some(&mut *node_ptr);
    }

    /// Looks for free region with given size
    /// Removes it from list if available to be used
    ///
    /// returns tuple of list node and start address of the allocation
    fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
        // get mut ref to current head
        let mut current = &mut self.head;

        // walk down list to find large enough space
        while let Some(ref mut region) = current.next {
            // this is the check if region is good enough
            // if not Ok, ie. required size not found, move onto next ListNode
            if let Ok(alloc_start) = Self::alloc_from_region(region, size, align) {
                // region suitable for allocation -> remove node from list
                let next = region.next.take();
                let ret = Some((current.next.take().unwrap(), alloc_start));

                current.next = next;
                return ret;
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        // no region found
        None
    }

    /// Try to use given region for an allocation with given size
    ///
    /// Returns allocation start address on success
    /// ie. region is an unused ListNode
    fn alloc_from_region(region: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
        let alloc_start = align_up(region.start_addr(), align);
        let alloc_end = alloc_start.checked_add(size).ok_or(())?;

        // required allocation size too large for region
        if alloc_end > region.end_addr() {
            return Err(());
        }

        // check if we can add list node to excess space
        // if not the error
        let excess_size = region.end_addr() - alloc_end;
        if excess_size > 0 && excess_size < mem::size_of::<ListNode>() {
            // rest of the region is too small to hold ListNode (required because the
            // allocation splits region in a used and a free part)
            return Err(());
        }

        Ok(alloc_start)
    }

    /// Adjust the given layout so the resulting memory region is also capable of
    /// storing a `ListNode`
    ///
    /// Returns adjusted size and alignment as (size,align) tuple
    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(mem::align_of::<ListNode>())
            .expect("adjusting alignment failed")
            .pad_to_align();
        let size = layout.size().max(mem::size_of::<ListNode>());

        (size, layout.align())
    }
}
