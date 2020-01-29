pub mod cmd;
use crate::cffi::import::uart_send;
use crate::constant::BC26Status;
use alloc::{boxed::Box, collections::vec_deque::VecDeque, string::String, vec::Vec};
use cmd::{
    process::{CommandState, LiveCommand},
    Command, Response,
};
use core::clone::Clone;
use core::marker::Copy;
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
    async_in_flight: Option<LiveCommand>,
    in_flight: Option<LiveCommand>,
    urc_stack: Vec<Response>,
    _lock: bool,
}

impl BC26 {
    pub fn new() -> BC26 {
        BC26 {
            async_in_flight: None,
            in_flight: None,
            urc_stack: vec![],
            _lock: false,
        }
    }
    #[inline]
    fn lock(&mut self) {
        self._lock = true;
    }
    #[inline]
    fn unlock(&mut self) {
        self._lock = false;
    }
    #[inline]
    fn isLocked(&self) -> bool {
        return self._lock;
    }

    pub fn send_cmd(&mut self, live_cmd: LiveCommand) -> Result<BC26Status, BC26Status> {
        let raw = live_cmd.cmd.construct();

        if !live_cmd.cmd.asyncResp {
            self.lock();
        }
        unsafe {
            let (p, len, _cap) = raw.into_raw_parts();
            uart_send(p, len);
        }
        self.in_flight = Some(live_cmd);
        Ok(BC26Status::Ok)
    }
    pub fn feed(&mut self, line: String) -> Result<BC26Status, BC26Status> {
        let mut parsed_resp = Command::parse_line(line.as_str());
        //Handle URC Here
        match &mut self.in_flight {
            Some(cmd) => {
                cmd.feed(parsed_resp);
                return Ok(BC26Status::Ok);
            }
            None => match &mut self.async_in_flight {
                Some(async_cmd) => {
                    async_cmd.feed(parsed_resp);
                    if async_cmd.state == CommandState::Terminated {
                        return Ok(BC26Status::Ok);
                    }
                    return Ok(BC26Status::Ok);
                }
                None => Err(BC26Status::ErrStateMismatch),
            },
        }
    }

    pub fn process(&mut self) -> Option<LiveCommand> {
        let m = match &self.in_flight {
            Some(cmd) => {
                if cmd.state == CommandState::Terminated {
                    return Some(cmd.clone());
                } else {
                    return None;
                }
            }
            None => None,
        };
        if m.is_some() {
            self.unlock();
        }
        return m;
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
        let mut a = BC26::new();
        let b = Command {
            key: "CESQ",
            asyncResp: false,
            form: CommandForm::ExtWrite,
            parameters: vec![CommandParamater::Numerical(1)],
        };
        let live_cmd = LiveCommand::init(b);

        a.send_cmd(live_cmd);
        a.feed("+CESQ: 36,99,255,255,12,53".to_string());
        a.feed("OK".to_string());
        let resp = a.process();
        assert_eq!(resp.is_some(), true);
        assert_eq!(
            resp.unwrap().response[0],
            Response::Standard(Standard {
                key: "CESQ".to_string(),
                parameter: vec!["36", "99", "255", "255", "12", "53"]
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
            })
        );
    }
}
