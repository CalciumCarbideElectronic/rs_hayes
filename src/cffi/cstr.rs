use crate::cffi::import::strlen;
use alloc::slice;
use core::str;

#[derive(Hash)]
pub struct CStr {
    inner: [u8],
}

impl CStr {
    pub unsafe fn from_raw_ptr<'a>(ptr: *const u8) -> &'a str {
        let len = strlen(ptr) as usize;
        str::from_utf8_unchecked(slice::from_raw_parts(ptr, len))
    }
}
