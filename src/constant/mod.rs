pub mod errtype;
pub mod restype;
#[repr(C)]
#[derive(Debug)]
pub enum BC26Status {
    Ok,
    ErrStateMismatch,
    ErrLocked,
    ErrResponseTypeMismatch,
    Timeout,
}
