use crate::bc26::cmd::{process::LiveCommand, Command, CommandForm, Response, Standard};
use crate::bc26::BC26;
use crate::cffi::import::DebugS;
use crate::constant::{restype, BC26Status};
use alloc::{rc::Rc, string::String, vec::Vec};
use core::cell::RefCell;

macro_rules! assert_resp_len {
    ($res:ident,$len:expr) => {
        if !(($res).parameter.len() == $len) {
            return Err(BC26Status::ErrResponseParsedLengthMismatch);
        }
    };
}

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
    pub fn CGATT_read(&mut self) -> Result<restype::CGATT_STATE, BC26Status> {
        let mut CGATT = Rc::new(RefCell::new(LiveCommand::init(Command {
            key: "CGATT",
            asyncResp: false,
            form: CommandForm::ExtRead,
            parameters: vec![],
        })));
        match self.poll_cmd(CGATT.clone(), 200) {
            Err(e) => {
                return Err(e);
            }
            Ok(_) => {
                if let Some(res) = self.get_standard_response(&CGATT.borrow().response) {
                    assert_resp_len!(res, 1);
                    let tcode = res.parameter[0].as_str();
                    if tcode == "1" {
                        return Ok(restype::CGATT_STATE::Attached);
                    } else if tcode == "0" {
                        return Ok(restype::CGATT_STATE::Detached);
                    } else {
                        return Err(BC26Status::ErrResponseTypeMismatch);
                    }
                } else {
                    return Err(BC26Status::ErrResponseTypeMismatch);
                }
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
