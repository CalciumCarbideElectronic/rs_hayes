#![cfg_attr(not(test), no_std)]
#![feature(vec_into_raw_parts)]
#![feature(const_raw_ptr_deref)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate alloc;

pub mod export;

mod allocator;
mod panic;
mod mqtt;
mod cffi;
mod bc26;
mod constant;
mod sysutil;

