pub mod cmd;
use core::result::Result;
use crate::constant::{ OK,ErrStateMismatch};
use crate::cffi::import::uart_send;
use alloc::{
    vec::Vec,
};
use cmd::{
    Command,
    Response,
};

#[derive(Eq, PartialEq)]
enum BC26State {
    IDLE,
    WaitForResponse,
    WaitForProcess,
}

pub struct BC26{
    state: BC26State,
    in_flight: Option< Command >,
    response_stack: Vec<Response>,
    urc_stack:Vec<Response>
}

impl BC26 {
    pub fn new() -> BC26 {
        BC26 {
            state: BC26State::IDLE,
            response_stack:vec![],
            urc_stack:vec![],
            in_flight: None
        }
    }
    pub fn send_cmd(&mut self, cmd: &str) -> Result<OK,ErrStateMismatch> {
        if self.state == BC26State::IDLE {
            unsafe {
                uart_send("Hello2".as_ptr(), 6);
            }
            self.state = BC26State::WaitForResponse;
            return Ok(OK);
        } else {
            return Err(ErrStateMismatch);
        }
    }
}
