use alloc::string::String;
use stdi
#[cfg(not(test))]
extern {
    pub fn uart_send(data: *const u8, size: usize) -> i32;
    pub fn Debug(data: *const u8, len: u16);
    pub fn strlen(p: *const u8) -> size_t;
}

#[warn(non_snake_case)]
pub unsafe fn DebugS(s: String) {
    Debug(s.as_str().as_ptr(), s.len() as u16);
}

#[cfg(test)]
use core::slice;

#[cfg(test)]
pub unsafe fn Debug(data: *const u8, len: u16) {
    let s = slice::from_raw_parts(data, len)
        .iter()
        .map(|e| *e as char)
        .collect::<String>();
    println!("{}", s);
}

#[cfg(test)]
pub fn uart_send(data: *const u8, size: usize) -> i32 {
    0
}
#[cfg(test)]
use std::{thread, time};

#[cfg(test)]
pub fn osDelay(tick: usize) {
    thread::sleep(time::Duration::from_millis(2));
}
#[cfg(test)]
extern "C" {
    pub fn strlen(p: *const u8) -> u32;
}
