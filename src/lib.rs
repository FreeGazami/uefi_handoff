#![no_main]
#![no_std]

use core::ffi::c_void;

#[repr(C)]
pub struct BootInfo {
    pub image_handle: *mut c_void,
    pub runtime_services: *mut c_void,
    pub mm: *mut c_void,
    pub mm_len: usize,
    pub acpi_table: *mut c_void,
    pub frame_buffer_base: u64,
    pub frame_buffer_size: usize,
    pub info: GraphicsOutputModeInformation,
}