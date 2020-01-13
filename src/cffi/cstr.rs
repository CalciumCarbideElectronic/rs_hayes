use alloc::slice;
use core::str;
use crate::cffi::import::{strlen};


#[derive(Hash)]
pub struct CStr{ 
    inner: [ u8]
}


impl CStr{
    pub unsafe fn len(&self)->usize{
        let bytes = self.to_bytes_with_nul();
        bytes.len()

    }

    pub unsafe fn from_ptr<'a>(ptr: *const u8) -> &'a CStr {
        let len = strlen(ptr);
        let ptr = ptr as *const u8;
        CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(ptr, len as usize + 1))
    }

    #[inline]
    const unsafe fn from_bytes_with_nul_unchecked(bytes: &[u8]) -> &CStr {
        &*(bytes as *const [u8] as *const CStr)
    }


    #[inline]
    pub fn to_bytes(&self) -> &[u8] {
        let bytes = self.to_bytes_with_nul();
        &bytes[..bytes.len() - 1]
    }

    #[inline]
    pub fn to_bytes_with_nul(&self) -> &[u8] {
        unsafe { &*(&self.inner as *const [ u8] as *const [u8]) }
    }

    pub unsafe fn to_str_unsafe(&self) -> &str{
        str::from_utf8_unchecked(self.to_bytes())
    }
    
}
