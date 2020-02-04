pub mod cmd;
pub mod ctl;
use crate::cffi::import::uart_send;
use crate::constant::BC26Status;
use crate::sysutil::import::osMessageQueueId_t;
use crate::sysutil::mutex::Mutex;
use crate::sysutil::poll_for_result;
use crate::sysutil::queue::Queue;
use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::{rc::Rc, string::String, vec::Vec};
use cmd::{
    process::{CommandState, LiveCommand},
    Command, Response,
};
use core::cell::RefCell;
use core::clone::Clone;
use core::result::Result;

pub type MutexedBC26 = Arc<Mutex<BC26>>;

#[derive(Debug)]
pub struct BC26 {
    in_flight: Option<Rc<RefCell<LiveCommand>>>,
    urc_stack: Vec<Response>,
    queue: Queue<Box<str>>,
}

impl BC26 {
    pub fn new(q_id: osMessageQueueId_t) -> Arc<Mutex<BC26>> {
        Arc::new(Mutex::new(BC26 {
            in_flight: None,
            urc_stack: vec![],
            queue: Queue::from(q_id),
        }))
    }
    pub fn get_qid(&self) -> osMessageQueueId_t {
        self.queue.get_qid()
    }
    #[inline]
    fn send_cmd(&mut self, live_cmd: Rc<RefCell<LiveCommand>>) -> Result<BC26Status, BC26Status> {
        self.in_flight = Some(live_cmd.clone());
        let e = live_cmd.clone();
        let cmd = &e.borrow().cmd;
        unsafe {
            let (p, len, cap) = cmd.construct().into_raw_parts();
            uart_send(p, len);
            // this line for correct memory deallocation
            let _rebuilt = String::from_raw_parts(p, len, cap);
        }
        Ok(BC26Status::Ok)
    }

    fn poll_input(&mut self, tick: u32) -> Result<BC26Status, BC26Status> {
        let line = self.queue.get(tick)?;
        let parsed_resp = Command::parse_line(&line);
        match &mut self.in_flight {
            Some(cmd) => {
                cmd.borrow_mut().feed(parsed_resp);
                return Ok(BC26Status::Ok);
            }
            None => Err(BC26Status::ErrUnexpectedError),
        }
    }

    fn process(&mut self) -> bool {
        match &self.in_flight {
            Some(cmd) => {
                if cmd.borrow().state == CommandState::Terminated {
                    return true;
                } else {
                    return false;
                }
            }
            None => false,
        }
    }

    pub fn poll_cmd(
        &mut self,
        live_cmd: Rc<RefCell<LiveCommand>>,
        timeout: usize,
    ) -> Result<BC26Status, BC26Status> {
        self.send_cmd(live_cmd)?;
        match poll_for_result(1, timeout, || {
            let res = self.poll_input(5);
            if res.is_err() {
                return false;
            }
            self.process()
        }) {
            true => Ok(BC26Status::Ok),
            false => Err(BC26Status::Timeout),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        cmd::{process::LiveCommand, Command, CommandForm, CommandParamater, Standard},
        Response, BC26,
    };
    #[test]
    fn test_normal_process() {}
}
