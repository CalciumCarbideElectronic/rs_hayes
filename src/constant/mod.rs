use crate::bc26::cmd::de::Error as DeError;
use crate::sysutil::import::osStatus_t;

pub mod errtype;
pub mod restype;
#[repr(C)]
#[derive(Debug)]
pub enum BC26Status {
    Ok,
    ErrStateMismatch,
    ErrLocked,
    ErrResponseTypeMismatch,
    ErrResponseParsedLengthMismatch,
    ErrUnexpectedError,
    ErrMutexError,
    ErrOSError,
    ErrDeserializer,
    Timeout,
}
impl From<osStatus_t> for BC26Status {
    fn from(t: osStatus_t) -> BC26Status {
        match t {
            osStatus_t::osOK => BC26Status::Ok,
            _ => BC26Status::ErrOSError,
        }
    }
}
impl From<DeError> for BC26Status {
    fn from(_t: DeError) -> BC26Status {
        BC26Status::ErrDeserializer
    }
}
