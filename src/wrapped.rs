use core::alloc::Layout;
use std::ptr;
use crate::raw;

#[inline]
pub unsafe fn allocate(layout: Layout, zeroed: bool) -> *mut [u8] {
    unsafe {
        // SAFETY: alle garantien liegen bei dem user.
        let ptr = if !zeroed {
            raw::alloc(layout)
        } else {
            raw::alloc_zeroed(layout)
        };
        // SAFETY: alle garantien liegen beim user.
        ptr::slice_from_raw_parts_mut(ptr, layout.size())
    }
}

#[inline]
pub unsafe fn reallocate(ptr: *mut u8, layout: Layout, new_size: usize, zeroed: bool) -> *mut [u8] {
    unsafe {
        // SAFETY: alle garantien liegen beim user.
        let nptr = if !zeroed {
            raw::realloc(ptr, layout, new_size)
        } else {
            raw::realloc_zeroed(ptr, layout, new_size)
        };
        // SAFETY: alle garantien liegen beim user.
        ptr::slice_from_raw_parts_mut(nptr, layout.size())
    }
}

#[inline]
pub unsafe fn deallocate(ptr: *mut [u8]) {
    unsafe {
        // SAFETY: alle garantien kommen vom user.
        raw::dealloc(ptr as *mut u8, Layout::from_size_align_unchecked(0, 0));
    }
}