use alloc::alloc::{GlobalAlloc, Layout};

pub struct FreeRTOSAllocator {}

extern  {
    pub fn pvPortMalloc(size: usize) -> *mut u8;
    pub fn vPortFree(pv: *mut u8);
}

unsafe impl GlobalAlloc for FreeRTOSAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        pvPortMalloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        vPortFree(ptr)
    }
}
