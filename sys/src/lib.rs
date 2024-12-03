#![allow(dead_code)]
#![allow(missing_docs)]

extern crate libc;

unsafe extern "C" {
    pub fn yesmalloc(size: libc::size_t, align: libc::size_t) -> *mut libc::c_void;
    pub fn yescalloc(size: libc::size_t, align: libc::size_t) -> *mut libc::c_void;
    pub fn yesremalloc(ptr: *mut libc::c_void, size: libc::size_t, align: libc::size_t, new_size: libc::size_t) -> *mut libc::c_void;
    pub fn yesrecalloc(ptr: *mut libc::c_void, size: libc::size_t, align: libc::size_t, new_size: libc::size_t) -> *mut libc::c_void;
    pub fn yesfree(ptr: *mut libc::c_void, size: libc::size_t, align: libc::size_t);
}