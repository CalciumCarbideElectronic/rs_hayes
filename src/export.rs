use heapless::Vec;
use heapless::consts::*;
use crate::BC26::BC26;
use core::str::from_utf8;

#[no_mangle]
pub extern fn holder(feed: *mut u8, len: usize) {
    unsafe {
//        let mut vec = from_raw_parts_mut(feed, len);
//        let str = from_utf8_mut(vec).unwrap();
    }
}

#[no_mangle]
pub extern fn construct() -> *mut BC26<'static> {
    unsafe {
        static mut obj: BC26<'static> = BC26::new();
        &mut obj as *mut BC26
    }
}

#[no_mangle]
pub extern fn get_probe(p: *mut BC26) -> *const u8 {
//    static a: &str = "probe";
    let obj = unsafe { &mut *p };
    obj.probe.as_ptr()
}

#[no_mangle]
pub extern fn send_cmd(p: *mut BC26, cmd: *const u8, len: u16) {
    let obj = unsafe { &mut *p };
    let a = from_utf8(unsafe { core::slice::from_raw_parts(cmd, len as usize) }).unwrap_or("");
    obj.send_cmd(a);
}

#[no_mangle]
pub extern fn recv(p: *mut BC26, cmd: *const u8, len: u16) {
    let obj = unsafe { &mut *p };
    let a = from_utf8(unsafe { core::slice::from_raw_parts(cmd, len as usize) }).unwrap_or("");
    obj.recv_process(a);
}

#[no_mangle]
pub extern fn nothing() {}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
}


