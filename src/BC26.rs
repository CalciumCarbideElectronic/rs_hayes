use core::result::Result;
use heapless::Vec;
use heapless::mpmc::Q32;
use heapless::consts::U100;
use crate::constant::{ERR_STATE_MISMATCH, OK};
use crate::import::uart_send;
use crate::BC26::BC26State::WaitForProcess;
use heapless::i::String;

#[derive(Eq, PartialEq)]
enum BC26State {
    IDLE,
    WaitForResponse,
    WaitForProcess,
}

enum ProcessState {}

struct BC26Result<'a> {
    result: Vec<&'a str, U100>,
    len: usize,
}

pub struct BC26<'a> {
    pub probe: &'a str,
    state: BC26State,
    response_stack: Q32<&'a str>,
    result: Option<BC26State>,
}


impl BC26<'_> {
    pub const fn new<'a>() -> BC26<'static> {
        BC26 {
            probe: "hnjkj",
            state: BC26State::IDLE,
            response_stack: Q32::new(),
            result: None,
        }
    }
    pub fn send_cmd(&mut self, cmd: &str) -> Result<OK, ERR_STATE_MISMATCH> {
        if self.state == BC26State::IDLE {
            unsafe {
                uart_send("Hello2".as_ptr(), 6);
            }
            self.state = BC26State::WaitForResponse;
            return Ok(OK);
        } else {
            return Err(ERR_STATE_MISMATCH);
        }
    }
    pub fn recv_process(&mut self, response: &'static str) {
        match &self.state {
            WaitForResponse => {
                self.response_stack.enqueue(response);
                if response.contains("OK") {
                    self.state = WaitForProcess;
                }
            }
            _ => {
                //TODO: Do something for URC Caching
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_bc_26() {}
}

