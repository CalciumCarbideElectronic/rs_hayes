use super::Command;
use super::Response;
use alloc::vec::Vec;


pub enum CommandState{
    Issued,
    Terminated
}

pub struct LiveCommand{
    pub cmd : Command,
    pub state: CommandState,
    pub response: Vec<Response>
}

impl LiveCommand{
    fn feed(&mut self,line_resp: &Response){
        match line_resp.clone(){
            Response::Error=>{
                self.response.push(Response::Error);
                self.state=CommandState::Terminated;
            },
            Response::OK=>{
                if !self.cmd.asyncResp{
                    self.state= CommandState::Terminated;
                }
            },
            Response::Standard(st) =>{
                self.response.push(line_resp.clone());
                if self.cmd.asyncResp{
                    self.state= CommandState::Terminated;
                } },
            _=>{
                self.response.push(line_resp.clone());
            }
        }
    }
}