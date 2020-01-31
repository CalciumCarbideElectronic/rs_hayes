use crate::bc26::BC26;
use crate::cffi::import::DebugS;
use crate::constant::{restype, BC26Status};
use alloc::boxed::Box;
use alloc::string::String;
use core::mem::transmute;
use core::slice;


#[no_mangle]
pub extern "C" fn construct(_begin: *mut u8, _size: usize) -> *mut BC26 {
    return unsafe { transmute(Box::new(BC26::new())) };
}

#[no_mangle]
pub extern "C" fn feed(ptr: *mut BC26, begin: *mut u8, size: usize) -> BC26Status {
    let bc26 = unsafe { &mut *ptr };

    let line = unsafe {
        slice::from_raw_parts(begin, size)
            .iter()
            .map(|e| *e as char)
            .collect::<String>()
    };

    // unsafe { DebugS(format!("Feed Line: {:?}\n", line.chars())) };

    match bc26.feed(line) {
        Err(e) => e,
        Ok(o) => o,
    }
}

#[no_mangle]
pub extern "C" fn Init<'a>(ptr: *mut BC26) -> BC26Status {
    let bc26 = unsafe { transmute::<*mut BC26, &mut BC26>(ptr) };
    match bc26.ATE() {
        Ok(o) => o,
        Err(e) => e,
    }
}

#[no_mangle]
pub extern "C" fn checkConnect(ptr: *mut BC26) -> BC26Status {
    let bc26 = unsafe { transmute::<*mut BC26, &mut BC26>(ptr) };
    match bc26.CGATT_read() {
        Ok(e) => {
            if e == restype::CGATT_STATE::Attached {
                unsafe { DebugS(format!("Network attached\n")) };
                return BC26Status::Ok;
            } else {
                unsafe { DebugS(format!("Network detached\n")) };
                return BC26Status::Ok;
            }
        }
        Err(e) => {
            unsafe { DebugS(format!("CGATT Error:{:?}\n", e)) };
            return e;
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
}
