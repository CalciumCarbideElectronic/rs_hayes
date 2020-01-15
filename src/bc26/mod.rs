pub mod cmd;
use core::result::Result;
use crate::constant::{ BC26Status};
use crate::cffi::import::uart_send;
use core::marker::Copy;
use core::clone::Clone;
use alloc::{
    vec::Vec,
    boxed::Box
};
use cmd::{
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
    state: BC26State,
    in_flight: Option< Box<Command> >,
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
    pub fn send_cmd(&mut self,cmd:Command, raw:&str) -> Result<BC26Status,BC26Status> {
        match self.state{
            BC26State::IDLE=>{
                unsafe{
                    uart_send(raw.as_ptr(), raw.len());
                }
                self.state = BC26State::WaitForResponse;
                Ok(BC26Status::Ok)
            },
            BC26State::WaitForProcess=>Err(BC26Status::ErrStateMismatch),
            _=>Err(BC26Status::ErrStateMismatch)
        }
    }
    pub fn feed(&mut self,line: &str)->Result<BC26Status,BC26Status>{
        match self.state{
        BC26State::WaitForResponse =>{
            match Command::parse_line(line){
                    Response::OK=>{
                        self.state=BC26State::WaitForProcess 
                    },
                    Response::Error=>{ self.state=BC26State::IDLE },
                    e=>{ self.response_stack.push(e) }

                };
                Ok(BC26Status::Ok)

        } 
        _=> Err(BC26Status::ErrStateMismatch)

        }
    }
    pub fn process(&mut self)->Vec<Response>{
        self.state=BC26State::IDLE;
        let e = self.response_stack.clone();
        self.response_stack.clear();
        e
    }
}

#[cfg(test)]
mod test{
    use super::{BC26State,BC26,Response,cmd::{Standard,CommandParamater,Command}};
    #[test]
    fn test_normal_process(){
        let mut a = BC26::new();
        let b = Command{
            key:"CESQ",
            base:"AT+CESQ",
            parameters: vec![
                CommandParamater::Numerical(1)
            ]
        };
        let raw = b.as_write();

        a.send_cmd(b,&raw);
    
        a.feed("+CESQ: 36,99,255,255,12,53");
        a.feed("OK");
        assert_eq!(a.state,BC26State::WaitForProcess);
        let e = a.process();
        //should dump here
        assert_eq!(a.response_stack.len(),0);
        assert_eq!(a.state,BC26State::IDLE);

        assert_eq!(e,vec![
            Response::Standard(Standard{
                key:"CESQ".to_string(),
                parameter:vec!["36","99","255","255","12","53"].iter()
                .map(|e|e.to_string()).collect::<Vec<String>>()
            })
        ]);
        


    }

}

