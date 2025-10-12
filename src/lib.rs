#![no_main]
#![no_std]

use uefi_raw::table::system::{ SystemTable };
use uefi_raw::Handle;

#[repr(C)]
pub struct BootInfo {
    pub image_handle: *mut c_void,
    pub system_table:  *mut u8,
    pub mm_ptr: *mut [u8],
}