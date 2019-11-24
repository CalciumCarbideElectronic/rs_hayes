use heapless::Vec;
use heapless::consts::*;

#[no_mangle]
pub extern fn holder(feed: *mut u8, len: usize) {
    unsafe {
//        let mut vec = from_raw_parts_mut(feed, len);
//        let str = from_utf8_mut(vec).unwrap();
    }
}

#[no_mangle]
pub extern fn nothing() {}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
}

