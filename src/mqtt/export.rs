use crate::bc26::MutexedBC26;
use crate::cffi::cstr::CStr;
use crate::cffi::import::DebugS;
use crate::constant::BC26Status;
use crate::mqtt::MQTT;
use alloc::{boxed::Box};
use core::intrinsics::transmute;

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
        let mqtt = &mut *ptr;
        let hostname = CStr::from_raw_ptr(host_name);
        match mqtt.open_network(conn_id, hostname, port) {
            Ok(o) => o,
            Err(e) => e,
        }
    }
}

#[no_mangle]
pub extern "C" fn mqtt_conn(ptr: *mut MQTT, conn_id: u16, client_id: *const u8) -> BC26Status {
    unsafe {
        let mqtt = &mut *ptr;
        match mqtt.open_connection(conn_id, CStr::from_raw_ptr(client_id)) {
            Ok(o) => o,
            Err(e) => e,
        }
    }
}

#[no_mangle]
pub extern "C" fn mqtt_publish(
    ptr: *mut MQTT,
    conn_id: u8,
    msg_id: u16,
    qos: u8,
    retain: bool,
    topic: *const u8,
    msg: *const u8,
) -> BC26Status {
    unsafe {
        let mqtt = &mut *ptr;
        match mqtt.qmtpub_write(
            conn_id,
            msg_id,
            qos,
            retain,
            CStr::from_raw_ptr(topic),
            CStr::from_raw_ptr(msg),
        ) {
            Ok(o) => {
                DebugS(format!("mqtt publish  ok, resp: {:?}\n", o));
                BC26Status::Ok
            }
            Err(e) => {
                DebugS(format!("mqtt  publish error: {:?} \n", e));
                e
            }
        }
    }
}
