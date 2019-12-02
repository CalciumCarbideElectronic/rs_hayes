use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    //just do nothing
    loop {}
}