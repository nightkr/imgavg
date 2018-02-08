mod imgavg;

use std::{mem, slice};
use std::ffi::CString;
use std::os::raw::{c_char, c_void};

#[no_mangle]
pub extern "C" fn calculate_background(data: *mut u8, size: usize) -> *mut c_char {
    unsafe {
        let buf = slice::from_raw_parts(data, size);
        let colour = imgavg::calculate_background(&buf);
        CString::new(colour).unwrap().into_raw()
    }
}

// copied from https://www.hellorust.com/demos/sha1/index.html
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}
