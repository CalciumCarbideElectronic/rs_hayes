use alloc_cortex_m::CortexMHeap;
use crate::cffi::import;

#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[cfg(not(test))]
#[alloc_error_handler]
fn my_example_handler(layout: core::alloc::Layout) -> ! {
    let s =  alloc::string::String::from(
        format!("memory allocation of {} bytes failed", layout.size())
    );
    unsafe{
        import::uart_send(s.as_bytes().as_ptr(),s.len());
    }
    panic!("memory allocation of {} bytes failed", layout.size())
}