use crate::bc26::{MutexedBC26, BC26};
use crate::cffi::import::DebugS;
use crate::constant::{restype, BC26Status};
use crate::sysutil::import::osMessageQueueId_t;
use crate::sysutil::queue::Queue;
use alloc::boxed::Box;
use alloc::string::String;
use core::mem::transmute;
use core::ptr::null;
use core::slice;
use core::str;

#[no_mangle]
pub extern "C" fn construct(_begin: *mut u8, _size: usize) -> *mut MutexedBC26 {
    let queue: Queue<Box<str>> = Queue::new(40);
    return unsafe { transmute(Box::new(BC26::new(queue.get_qid()))) };
}

macro_rules! get_bc26_obj {
    ($ptr:expr) => {{
        unsafe { &*$ptr }
    }};
}
macro_rules! expand_result {
    ($result:expr) => {
        match $result {
            Err(e) => e,
            Ok(o) => o,
        }
    };
}

#[no_mangle]
pub extern "C" fn feed(qid: osMessageQueueId_t, begin: *mut u8, size: usize) -> BC26Status {
    let queue: Queue<Box<str>> = Queue::from(qid);

    let line = unsafe {
        slice::from_raw_parts(begin, size)
            .iter()
            .map(|e| *e as char)
            .collect::<String>()
            .into_boxed_str()
    };
    match queue.put(line, 20) {
        Ok(o) => BC26Status::from(o),
        Err(r) => BC26Status::from(r),
    }
}
#[no_mangle]
pub extern "C" fn get_bc26_qid<'a>(ptr: *mut MutexedBC26) -> osMessageQueueId_t {
    let bc26 = get_bc26_obj!(ptr);
    match &mut bc26.clone().lock() {
        Ok(e) => e.get_qid(),
        _ => null() as osMessageQueueId_t,
    }
}

#[no_mangle]
pub extern "C" fn Init<'a>(ptr: *mut MutexedBC26) -> BC26Status {
    let bc26 = get_bc26_obj!(ptr);
    match &mut bc26.clone().lock() {
        Ok(e) => expand_result!(e.ATE()),
        _ => BC26Status::ErrMutexError,
    }
}

#[no_mangle]
pub extern "C" fn checkConnect(ptr: *mut MutexedBC26) -> BC26Status {
    let bc26 = get_bc26_obj!(ptr);
    match &mut bc26.clone().lock() {
        Ok(e) => match e.CGATT_read() {
            Ok(e) => {
                if e.state == String::from("1") {
                    unsafe { DebugS(format!("Network attached\n")) };
                    return BC26Status::Ok;
                } else {
                    unsafe { DebugS(format!("Network detached\n")) };
                    return BC26Status::Ok;
                }
            }
            Err(e) => {
                unsafe { DebugS(format!("\nCGATT Error:{:?}\n", e)) };
                return e;
            }
        },
        _ => BC26Status::ErrMutexError,
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
}
