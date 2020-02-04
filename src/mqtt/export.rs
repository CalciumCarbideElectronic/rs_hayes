use crate::bc26::MutexedBC26;
use crate::bc26::BC26;
use crate::cffi::cstr::CStr;
use crate::cffi::import::strlen;
use crate::cffi::import::DebugS;
use crate::constant::BC26Status;
use crate::mqtt::MQTT;
use alloc::{boxed::Box, string::String};
use core::intrinsics::transmute;
use core::{slice, str};


#[no_mangle]
pub extern "C" fn mqtt_construct(ptr: *mut MutexedBC26) -> *mut MQTT {
    return unsafe { transmute(Box::new(MQTT::new((&*ptr).clone()))) };
}

#[no_mangle]
pub extern "C" fn mqtt_open(
    ptr: *mut MQTT,
    conn_id: u16,
    host_name: *const u8,
    port: u16,
) -> BC26Status {
    unsafe {
        DebugS(String::from("Try to open: "));
        let mut mqtt = &mut *ptr;
        let hostname = CStr::from_raw_ptr(host_name);
        DebugS(String::from(hostname));
        DebugS(String::from("\n"));
        match mqtt.open_network(conn_id, hostname, port) {
            Ok(o) => {
                DebugS(String::from("Mqtt open ok\n"));
                o
            }
            Err(e) => {
                DebugS(format!("Mqtt open error {:?}", e));
                e
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn mqtt_conn(ptr: *mut MQTT, conn_id: u16, client_id: *const u8) -> BC26Status {
    unsafe {
        let mut mqtt = &mut *ptr;
        let cid = CStr::from_raw_ptr(client_id);
        match mqtt.open_connection(conn_id, cid) {
            Ok(o) => {
                DebugS(String::from("Mqtt connect ok\n"));
                o
            }
            Err(e) => {
                DebugS(format!("Mqtt connect error: {:?} \n", e));
                e
            }
        }
    }
}
