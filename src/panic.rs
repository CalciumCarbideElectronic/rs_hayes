use core::panic::PanicInfo;
use super::cffi::import;

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    //just do nothing
    let s =  alloc::string::String::from( format!("panic:{:?}", info));
    unsafe{
        import::uart_send(s.as_bytes().as_ptr(),s.len());
    }
    loop {}
}