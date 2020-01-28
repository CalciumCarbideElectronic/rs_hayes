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
enum BC26State {
    IDLE,
    WaitForResponse,
    WaitForProcess,
}

#[derive(Debug)]
pub struct BC26 {
    in_flight: VecDeque<LiveCommand>,
    response_stack: Vec<LiveCommand>,
    urc_stack: Vec<Response>,
    _lock: bool,
}

impl BC26 {
    pub fn new() -> BC26 {
        BC26 {
            in_flight: VecDeque::new(),
            response_stack: vec![],
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
        unsafe {
            let (p, len, _cap) = raw.into_raw_parts();
            uart_send(p, len);
        }
        self.lock();
        self.in_flight.push_back(live_cmd);
        Ok(BC26Status::Ok)
    }
    pub fn feed(&mut self, line: String) -> Result<BC26Status, BC26Status> {
        match self.in_flight.back_mut() {
            Some(cmd) => {
                let mut parsed_resp = Command::parse_line(line.as_str());
                if cmd.state == CommandState::Terminated {
                    let cmd = self.in_flight.pop_back().unwrap();
                    self.response_stack.push(cmd)
                }
                return Ok(BC26Status::Ok);
            }
            None => Err(BC26Status::ErrStateMismatch),
        }
    }

    pub fn process(&mut self) -> Vec<LiveCommand> {
        let e = self.response_stack.clone();
        self.response_stack.clear();
        e
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
        assert_eq!(resp.len(), 1);
        assert_eq!(resp[0].response.len(), 1);
        assert_eq!(
            resp[0].response[0],
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
