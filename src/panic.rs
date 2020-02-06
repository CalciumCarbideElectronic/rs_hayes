use crate::sysutil::import::osDelay;
use crate::cffi::import::DebugS;
use alloc::string::String;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    //just do nothing
    unsafe {
        DebugS(format!("{:?}\n\n", info));
        loop {
            DebugS(String::from("Panicing!\n"));
            osDelay(10000);
        }
    }
}
