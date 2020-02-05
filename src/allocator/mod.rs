use crate::cffi::import;
use crate::cffi::import::DebugS;
mod freertos;

// #[cfg(not(test))]
// #[global_allocator]
// pub static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: freertos::FreeRTOSAllocator = freertos::FreeRTOSAllocator {};

#[cfg(not(test))]
#[alloc_error_handler]
fn my_example_handler(layout: core::alloc::Layout) -> ! {
    let s = format!("memory allocation of {} bytes failed", layout.size());
    unsafe {
        DebugS(s);
    }
    panic!("memory allocation of {} bytes failed", layout.size())
}
