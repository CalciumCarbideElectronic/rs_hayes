use super::mutex::Mutex;
use crate::cffi::import::DebugS;
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::mem::transmute;

#[repr(C)]
pub struct Foo {
    counter: i32,
}

#[repr(C)]
pub struct MutexedFoo(*const Arc<Mutex<Foo>>);

#[no_mangle]
pub extern "C" fn construct_mutex_obj() -> *const MutexedFoo {
    unsafe {
        let obj = Box::new(Arc::new(Mutex::new(Foo { counter: 0 })));
        return transmute::<Box<Arc<Mutex<Foo>>>, *const MutexedFoo>(obj);
    }
}

#[no_mangle]
pub extern "C" fn counter_add(ptr: *const MutexedFoo) {
    let bj = unsafe { transmute::<*const MutexedFoo, &Arc<Mutex<Foo>>>(ptr) };
    match &mut (*bj).lock() {
        Ok(e) => {
            e.counter += 1;
            unsafe { DebugS(format!("counter:{}\n", e.counter)) };
        }
        _ => {}
    }
}
