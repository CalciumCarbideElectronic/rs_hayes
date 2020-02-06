use crate::bc26::cmd::{
    from_resp_vec, process::LiveCommand, Command, CommandForm, Response, Standard,
};
use crate::bc26::BC26;

use crate::constant::restype::CGATTResponse;
use crate::constant::{restype, BC26Status};
use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;


impl BC26 {
    fn get_standard_response<'a>(&self, resps: &'a Vec<Response>) -> Option<&'a Standard> {
        for i in resps {
            if let Response::Standard(s) = i {
                return Some(&s);
            }
        }
        return None;
    }

    pub fn ATE(&mut self) -> Result<BC26Status, BC26Status> {
        let mut cmd_ate = Rc::new(RefCell::new(LiveCommand::init(Command {
            key: "E0",
            asyncResp: false,
            form: CommandForm::AT,
            parameters: vec![],
        })));
        match self.poll_cmd(cmd_ate, 200) {
            Ok(_) => Ok(BC26Status::Ok),
            Err(e) => Err(e),
        }
    }
    pub fn CGATT_read(&mut self) -> Result<restype::CGATTResponse, BC26Status> {
        let mut CGATT = Rc::new(RefCell::new(LiveCommand::init(Command {
            key: "CGATT",
            asyncResp: false,
            form: CommandForm::ExtRead,
            parameters: vec![],
        })));
        match self.poll_cmd(CGATT.clone(), 500) {
            Err(e) => {
                return Err(e);
            }
            Ok(_) => {
                let res = from_resp_vec::<CGATTResponse>(&CGATT.borrow().response);
                let r = match res {
                    Ok(s) => Ok(s),
                    Err(e) => Err(BC26Status::ErrResponseTypeMismatch),
                };
                return r
            }
        }
    }
    pub fn CSCON_read(&mut self) -> Result<restype::CSCON_STATE, BC26Status> {
        let mut CGATT = Rc::new(RefCell::new(LiveCommand::init(Command {
            key: "CSCON",
            asyncResp: false,
            form: CommandForm::ExtRead,
            parameters: vec![],
        })));
        // match self.poll_cmd(CSCON.clone(), 300){
        //     Err(e) =>{
        //         return Err(e)
        //     },
        //     Ok(_)=>{
        //         for r in &CSCON.borrow().response{

        //         }
        //     }
        // }
        return Err(BC26Status::Timeout);
    }
}
