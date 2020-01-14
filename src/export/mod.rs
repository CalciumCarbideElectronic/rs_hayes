use crate::bc26::BC26;
use crate::cffi::cstr::{CStr};
use crate::cffi::import::{uart_send};

use core::str::from_utf8;
use alloc::string::String;
use alloc::boxed::Box;

#[cfg(not(test))]
use super::allocator::ALLOCATOR;



#[no_mangle]
pub extern fn construct(begin:*mut u8, size: usize) -> *mut BC26 {
    unsafe {
        let start = begin as usize;
        #[cfg(not(test))]
        ALLOCATOR.init(start,size) ;
        let mut obj: BC26 = BC26::new();
        &mut obj as *mut BC26
    }
}

#[no_mangle] 
pub extern fn heap_free(ptr: * mut u8) {
    unsafe{

    let _boxed = Box::from_raw(ptr as * mut String);
    };
 }

#[no_mangle] 
pub extern fn heap_test() -> * const u8{
    // let boxed: Box<String> = Box::new(String::from( format!("hello worxd {:x} 123456789", 123)));

    // let hello1 = "Hello, World!";
    // let hello2 =  String::from("again, hello");
    let hello3 =  Box::new(String::from("again,hello"));

    // let dummy  =  String::from("dummy,dummy,dummy");
    // unsafe {hello1.as_ptr()}
    // unsafe {hello2.as_ptr()}
    // unsafe {hello2.as_ptr()}
    // unsafe {hello3.as_ptr()}

    Box::into_raw(hello3) as *const u8
 }

#[no_mangle]
pub extern fn print_pointer(tag: *const u8, p: * const u8) {
    unsafe{
        let tag = CStr::from_ptr(tag);
        let s =  String::from(format!("{:}:{:p}\n",tag.to_str_unsafe(),p));
        uart_send(s.as_bytes().as_ptr(),s.len());
    }
}


#[no_mangle]
pub extern fn send_cmd(p: *mut BC26, cmd: *const u8, len: u16) {
    let obj = unsafe { &mut *p };
    let a = from_utf8(unsafe { core::slice::from_raw_parts(cmd, len as usize) }).unwrap_or("");
    obj.send_cmd(a);
}

#[no_mangle]
pub extern fn nothing() {}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
}
