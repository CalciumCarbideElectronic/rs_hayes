use super::Command;
use super::Response;
use alloc::vec::Vec;

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
    pub fn init(cmd: Command) -> LiveCommand {
        LiveCommand {
            cmd: cmd,
            state: CommandState::Issued,
            response: vec![],
        }
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
            Response::Standard(st) => {
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
}
