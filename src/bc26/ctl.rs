use crate::bc26::cmd::{from_resp_vec, process::LiveCommand, Command, CommandForm};
use crate::bc26::BC26;

use crate::constant::restype::CGATTResponse;
use crate::constant::{restype, BC26Status};
use alloc::rc::Rc;
use core::cell::RefCell;

impl BC26 {
    pub fn ATE(&mut self) -> Result<BC26Status, BC26Status> {
        let cmd_ate = Rc::new(RefCell::new(LiveCommand::init(Command {
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
        let CGATT = Rc::new(RefCell::new(LiveCommand::init(Command {
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
                    Err(_) => Err(BC26Status::ErrResponseTypeMismatch),
                };
                return r;
            }
        }
    }
}
