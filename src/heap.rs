use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use crate::raw;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Heap;

unsafe impl GlobalAlloc for Heap {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            // SAFETY: alle garantien kommen vom user.
            raw::alloc(layout)
        }
    }
    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe {
            // SAFETY: alle garantien kommen vom user.
            raw::alloc_zeroed(layout)
        }
    }
    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe {
            // SAFETY: alle garantien kommen vom user.
            raw::realloc(ptr, layout, new_size)
        }
    }
    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            // SAFETY: alle garantieren kommen vom user.
            raw::dealloc(ptr, layout);
        }
    }
}