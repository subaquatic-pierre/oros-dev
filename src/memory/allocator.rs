use alloc::alloc::{GlobalAlloc, Layout};
use core::{ops::DerefMut, ptr::null_mut};
use lazy_static::__Deref;
use linked_list_allocator::LockedHeap;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

use super::{bump::BumpAllocator, fixed::FixedSizeAllocator, linked_list::LinkedListAllocator};

#[global_allocator]
// pub static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());
// pub static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());
pub static ALLOCATOR: Locked<FixedSizeAllocator> = Locked::new(FixedSizeAllocator::new());

// define heap memeory location
pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100KiB

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should never be called")
    }
}

/// Create spin::Mutex wrapper for allocator
///
/// Used for interior mutability of the GlobalAllocator
/// trait
pub struct Locked<T> {
    inner: spin::Mutex<T>,
}

impl<T> Locked<T> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<T> {
        self.inner.lock()
    }
}

/// Align `addr` upwards to alignment `align`
///
/// Requires that `align` is power of two
pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
