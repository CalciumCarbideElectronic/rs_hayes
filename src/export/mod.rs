use crate::bc26::cmd::{process::LiveCommand, Command, CommandForm, CommandParamater};
use crate::bc26::BC26;
use crate::cffi::import::osDelay;
use crate::cffi::import::Debug;
use crate::cffi::import::DebugS;
use crate::constant::BC26Status;
use crate::sysutil::poll_for_result;
use alloc::boxed::Box;
use alloc::string::String;
use core::mem::transmute;
use core::slice;

#[cfg(not(test))]
use super::allocator::ALLOCATOR;

#[no_mangle]
pub extern "C" fn construct(begin: *mut u8, size: usize) -> *mut BC26 {
    unsafe {
        let start = begin as usize;
        #[cfg(not(test))]
        ALLOCATOR.init(start, size);
    }
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
    unsafe { DebugS(format!("Feed Line: {:?}\n", line.chars())) };
    match bc26.feed(line) {
        Err(e) => e,
        Ok(o) => o,
    }
}

#[no_mangle]
pub extern "C" fn Init(ptr: *mut BC26) -> BC26Status {
    let bc26 = unsafe { &mut *ptr };
    let res = bc26.send_cmd(LiveCommand::init(Command {
        key: "E0",
        asyncResp: false,
        form: CommandForm::AT,
        parameters: vec![],
    }));
    if res.is_err() {
        return res.err().unwrap();
    }
    if poll_for_result(2, 100, || match bc26.process() {
        Some(_) => true,
        None => false,
    }) {
        return BC26Status::Ok;
    } else {
        return BC26Status::Timeout;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
}
