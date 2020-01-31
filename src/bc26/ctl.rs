use crate::bc26::cmd::{process::LiveCommand, Command, CommandForm, Response};
use crate::bc26::BC26;
use crate::cffi::import::DebugS;
use crate::constant::{restype, BC26Status};
use alloc::{boxed::Box, rc::Rc, string::String};
use core::cell::RefCell;

impl BC26 {
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
                for r in &CGATT.borrow().response {
                    if let Response::Standard(res) = r {
                        if let Some(p) = res.parameter.iter().filter(|e| (**e).len() > 0).nth(0) {
                            if p == &String::from("1") {
                                return Ok(restype::CGATT_STATE::Attached);
                            } else if p == &String::from("0") {
                                return Ok(restype::CGATT_STATE::Detached);
                            } else {
                                return Err(BC26Status::ErrResponseTypeMismatch);
                            }
                        }
                    }
                }
                return Err(BC26Status::ErrResponseTypeMismatch);
            }
        }
    }
}
