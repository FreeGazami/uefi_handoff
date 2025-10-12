use uefi_raw::table::system::{ SystemTable };
use uefi_raw::Handle;

#[repr(C)]
struct BootInfo {
    img_handle: *mut Handle,
    sys_table:  *mut SystemTable,
    mm_ptr: *mut [u8],
}