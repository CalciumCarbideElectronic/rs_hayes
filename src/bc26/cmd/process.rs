use super::Command;
use super::Response;
use alloc;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

#[derive(Debug, PartialEq, Clone)]
pub enum CommandState {
    Issued,
    Terminated,
}

#[derive(Debug, Clone)]
pub struct LiveCommand {
    pub cmd: Command,
    pub state: CommandState,
    pub response: Vec<Response>,
}

impl LiveCommand {
    //deprecated
    pub fn init(cmd: Command) -> LiveCommand {
        LiveCommand {
            cmd: cmd,
            state: CommandState::Issued,
            response: vec![],
        }
    }

    pub fn new(cmd: Command) -> Rc<RefCell<LiveCommand>> {
        Rc::new(RefCell::new(LiveCommand::init(cmd)))
    }
    pub fn feed(&mut self, line_resp: Response) {
        match line_resp.clone() {
            Response::Error(e) => {
                self.response.push(Response::Error(e));
                self.state = CommandState::Terminated;
            }
            Response::OK => {
                if !self.cmd.asyncResp {
                    self.state = CommandState::Terminated;
                }
            }
            Response::Standard(_) => {
                self.response.push(line_resp.clone());
                if self.cmd.asyncResp {
                    self.state = CommandState::Terminated;
                }
            }
            _ => {
                self.response.push(line_resp.clone());
            }
        }
    }
    pub fn is_ok(&self) -> bool {
        if self.state != CommandState::Terminated {
            return false;
        }
        for i in &self.response {
            if let Response::Error(_) = i {
                return false;
            }
        }
        return true;
    }
}

impl IntoIterator for LiveCommand {
    type Item = Response;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.response.into_iter()
    }
}
