pub mod cmd;
use core::result::Result;
use crate::constant::{ BC26Status};
use crate::cffi::import::uart_send;
use core::marker::Copy;
use core::clone::Clone;
use alloc::{
    vec::Vec,
    boxed::Box,
    collections::vec_deque::VecDeque
};
use cmd::{
    process::{
        LiveCommand,
        CommandState
    },
    Command,
    Response,
};

#[derive(Eq, PartialEq,Debug)]
enum BC26State {
    IDLE,
    WaitForResponse,
    WaitForProcess,
}

#[derive(Debug)]
pub struct BC26{
    in_flight: VecDeque<LiveCommand>,
    response_stack: Vec<LiveCommand>,
    urc_stack:Vec<Response>,
}


impl BC26 {
    pub fn new() -> BC26 {
        BC26 {
            in_flight:VecDeque::new(),
            response_stack:vec![],
            urc_stack:vec![],
        }
    }
    pub fn send_cmd(&mut self,live_cmd:LiveCommand ) -> Result<BC26Status,BC26Status> {

        let raw = live_cmd.cmd.construct();
        unsafe{
            let (p,len,_cap) = raw.into_raw_parts();
            uart_send(p, len);
        }
        self.in_flight.push_back(live_cmd);
        Ok(BC26Status::Ok)
    }
    pub fn feed(&mut self,line: &str)->Result<BC26Status,BC26Status>{
        match self.in_flight.back_mut(){
            Some(cmd)=>{
                cmd.feed( Command::parse_line(line) );
                if cmd.state == CommandState::Terminated{
                    let cmd = self.in_flight.pop_back().unwrap();
                    self.response_stack.push(cmd)
                }
                return Ok(BC26Status::Ok)
            }
            None=>Err(BC26Status::ErrStateMismatch)
        }
    }

    pub fn process(&mut self)->Vec<LiveCommand>{
        let e = self.response_stack.clone();
        self.response_stack.clear();
        e
    }
}

#[cfg(test)]
mod test{
    use super::{
        BC26State,
        BC26,
        Response,cmd::
            { 
                Standard,CommandParamater,Command,CommandForm,
                process::LiveCommand
            }
        };
    #[test]
    fn test_normal_process(){
        let mut a = BC26::new();
        let b = Command{
            key:"CESQ",
            asyncResp:false,
            form:CommandForm::ExtWrite,
            parameters: vec![
                CommandParamater::Numerical(1)
            ]
        };
        let live_cmd = LiveCommand::init(b);


        a.send_cmd(live_cmd);
        a.feed("+CESQ: 36,99,255,255,12,53");
        a.feed("OK");
        let resp = a.process();
        assert_eq!(resp.len(),1);
        assert_eq!(resp[0].response.len(),1);
        assert_eq!(resp[0].response[0],
            Response::Standard(Standard{
                key:"CESQ".to_string(),
                parameter:vec!["36","99","255","255","12","53"].iter()
                .map(|e|e.to_string()).collect::<Vec<String>>()
            })
        );
    }

}

