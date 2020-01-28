use crate::bc26::cmd::{process::LiveCommand, Command, CommandForm, CommandParamater};
use crate::bc26::BC26;
use alloc::boxed::Box;
use alloc::string::String;
use core::mem::transmute;

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
pub extern "C" fn feed(ptr: *mut BC26, begin: *mut u8, size: usize) {
    unsafe {
        let mut bc26 = &mut *ptr;
        let line = String::from_raw_parts(begin, size, size);
        bc26.feed(line);
    };
}

#[no_mangle]
pub extern "C" fn AT(ptr: *mut BC26) {
    let mut bc26 = unsafe { &mut *ptr };
    let b = Command {
        key: "CESQ",
        asyncResp: false,
        form: CommandForm::ExtWrite,
        parameters: vec![CommandParamater::Numerical(1)],
    };
    let live_cmd = LiveCommand::init(b);
    bc26.send_cmd(live_cmd);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
}
