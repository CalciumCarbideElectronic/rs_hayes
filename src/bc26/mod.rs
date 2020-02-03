pub mod cmd;
pub mod ctl;
use crate::cffi::import::uart_send;
use crate::constant::BC26Status;
use crate::sysutil::poll_for_result;
use alloc::{rc::Rc, string::String, vec::Vec};
use cmd::{
    process::{CommandState, LiveCommand},
    Command, Response,
};
use core::cell::RefCell;
use core::clone::Clone;
use core::result::Result;

#[derive(Eq, PartialEq, Debug)]
#[repr(C)]
pub enum BC26State {
    IDLE,
    WaitForResponse,
    WaitForProcess,
}

#[derive(Debug)]
pub struct BC26 {
    async_in_flight: Option<Rc<RefCell<LiveCommand>>>,
    in_flight: Option<Rc<RefCell<LiveCommand>>>,
    urc_stack: Vec<Response>,
}

impl BC26 {
    pub fn new() -> BC26 {
        BC26 {
            async_in_flight: None,
            in_flight: None,
            urc_stack: vec![],
        }
    }
    #[inline]
    fn lock_sync(&mut self, live_cmd: Rc<RefCell<LiveCommand>>) {
        if self.in_flight.is_none() {
            self.in_flight = Some(live_cmd)
        }
    }

    #[inline]
    fn lock_async(&mut self, live_cmd: Rc<RefCell<LiveCommand>>) {
        if self.async_in_flight.is_none() {
            self.async_in_flight = Some(live_cmd)
        }
    }

    #[inline]
    fn unlock_sync(&mut self) {
        self.in_flight = None;
    }
    #[inline]
    fn unlocak_async(&mut self) {
        self.async_in_flight = None
    }
    #[inline]
    fn can_send_sync_cmd(&self) -> bool {
        self.in_flight.is_none()
    }
    #[inline]
    fn can_send_async_cmd(&self) -> bool {
        self.async_in_flight.is_none() && self.in_flight.is_none()
    }
    #[inline]
    fn send_sync_cmd(
        &mut self,
        live_cmd: Rc<RefCell<LiveCommand>>,
    ) -> Result<BC26Status, BC26Status> {
        let e = live_cmd.clone();
        let cmd = &e.borrow().cmd;
        match self.send_cmd(cmd) {
            Ok(o) => {
                self.lock_sync(live_cmd);
                Ok(o)
            }
            Err(e) => {
                self.unlock_sync();
                Err(e)
            }
        }
    }
    #[inline]
    fn send_async_cmd(
        &mut self,
        live_cmd: Rc<RefCell<LiveCommand>>,
    ) -> Result<BC26Status, BC26Status> {
        let e = live_cmd.clone();
        let cmd = &e.borrow().cmd;
        match self.send_cmd(cmd) {
            Ok(o) => {
                self.lock_async(live_cmd);
                Ok(o)
            }
            Err(e) => {
                self.unlocak_async();
                Err(e)
            }
        }
    }

    fn send_cmd(&mut self, cmd: &Command) -> Result<BC26Status, BC26Status> {
        unsafe {
            let (p, len, cap) = cmd.construct().into_raw_parts();
            uart_send(p, len);
            // this line for correct memory deallocation
            let _rebuilt = String::from_raw_parts(p, len, cap);
        }
        Ok(BC26Status::Ok)
    }

    pub fn feed(&mut self, line: String) -> Result<BC26Status, BC26Status> {
        let parsed_resp = Command::parse_line(line.as_str());
        //Handle URC Here
        match &mut self.in_flight {
            Some(cmd) => {
                cmd.borrow_mut().feed(parsed_resp);
                return Ok(BC26Status::Ok);
            }
            None => match &mut self.async_in_flight {
                Some(async_cmd) => {
                    async_cmd.borrow_mut().feed(parsed_resp);
                    if async_cmd.borrow().state == CommandState::Terminated {
                        return Ok(BC26Status::Ok);
                    }
                    return Ok(BC26Status::Ok);
                }
                None => Err(BC26Status::ErrStateMismatch),
            },
        }
    }

    pub fn process_sync(&mut self) -> bool {
        let terminated: bool = match &self.in_flight {
            Some(cmd) => {
                if cmd.borrow().state == CommandState::Terminated {
                    return true;
                } else {
                    return false;
                }
            }
            None => false,
        };

        if terminated {
            self.unlock_sync();
        }
        return terminated;
    }
    pub fn process_async(&mut self) -> bool {
        let terminated: bool = match &self.async_in_flight {
            Some(cmd) => {
                if cmd.borrow().state == CommandState::Terminated {
                    return true;
                } else {
                    return false;
                }
            }
            None => false,
        };

        if terminated {
            self.unlocak_async();
        }
        return terminated;
    }

    pub fn poll_cmd(
        &mut self,
        live_cmd: Rc<RefCell<LiveCommand>>,
        timeout: usize,
    ) -> Result<BC26Status, BC26Status> {
        let is_async = live_cmd.borrow().cmd.asyncResp;
        if !is_async {
            if !self.can_send_sync_cmd() {
                return Err(BC26Status::ErrLocked);
            }
            if let Err(e) = self.send_sync_cmd(live_cmd) {
                return Err(e);
            }
            let res = match poll_for_result(2, timeout, || self.process_sync()) {
                true => Ok(BC26Status::Ok),
                false => Err(BC26Status::Timeout),
            };
            self.unlock_sync();
            return res;
        } else {
            if !self.can_send_async_cmd() {
                return Err(BC26Status::ErrLocked);
            }
            if let Err(e) = self.send_async_cmd(live_cmd) {
                return Err(e);
            }
            let res = match poll_for_result(2, timeout, || self.process_async()) {
                true => Ok(BC26Status::Ok),
                false => Err(BC26Status::Timeout),
            };
            self.unlocak_async();
            return res;
        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        cmd::{process::LiveCommand, Command, CommandForm, CommandParamater, Standard},
        BC26State, Response, BC26,
    };
    #[test]
    fn test_normal_process() {
        // let mut a = BC26::new();
        // let b = Command {
        //     key: "CESQ",
        //     asyncResp: false,
        //     form: CommandForm::ExtWrite,
        //     parameters: vec![CommandParamater::Numerical(1)],
        // };
        // let live_cmd = LiveCommand::init(b);

        // a.send_cmd(live_cmd);
        // a.feed("+CESQ: 36,99,255,255,12,53".to_string());
        // a.feed("OK".to_string());
        // let resp = a.process();
        // assert_eq!(resp.is_some(), true);
        // assert_eq!(
        //     resp.unwrap().response[0],
        //     Response::Standard(Standard {
        //         key: "CESQ".to_string(),
        //         parameter: vec!["36", "99", "255", "255", "12", "53"]
        //             .iter()
        //             .map(|e| e.to_string())
        //             .collect::<Vec<String>>()
        //     })
        // );
    }
}
