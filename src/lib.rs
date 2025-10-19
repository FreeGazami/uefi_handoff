#![no_main]
#![no_std]

use core::ffi::c_void;

#[repr(C)]
pub struct BootInfo {
    pub image_handle: *mut c_void,
    pub runtime_services: *mut c_void,
    pub mm_ptr: *mut c_void,
}