
use super::{ MQTT};
// use super::{MQTTFlags, MQTT};
// use crate::bc26::cmd::{process::LiveCommand, Command, CommandForm, CommandParamater};
// use crate::constant::BC26Status;
// use alloc::rc::Rc;
// use alloc::str;
// use alloc::string::{ToString};
// use core::cell::RefCell;

impl MQTT {
    // fn get_cfg_base_command(&self, tag: &str) -> Command {
    //     Command {
    //         asyncResp: false,
    //         key: "QMTCFG",
    //         form: CommandForm::ExtWrite,
    //         parameters: vec![
    //             CommandParamater::Literal(tag.to_string()),
    //             CommandParamater::Numerical(self.session as u32),
    //         ],
    //     }
    // }

    // fn gencmd_set_will(&self) -> Command {
    //     let mut command = self.get_cfg_base_command("WILL");
    //     if (self.flag & MQTTFlags::WILL).bits() != 0 {
    //         command.parameters.extend(
    //             vec![
    //                 CommandParamater::Numerical(1),                    //will_fg
    //                 CommandParamater::Numerical(self.will_qos as u32), //qos
    //             ]
    //             .into_iter(),
    //         );
    //         if (self.flag & MQTTFlags::WILL_RETAIN) == MQTTFlags::WILL_RETAIN {
    //             command.parameters.push(
    //                 CommandParamater::Numerical(1), //reatin
    //             );
    //         } else {
    //             command.parameters.push(
    //                 CommandParamater::Numerical(0), //retain
    //             );
    //         }
    //         command.parameters.extend(
    //             vec![
    //                 CommandParamater::Literal(self.will_topic.to_string()),
    //                 CommandParamater::Literal(self.will_msg.to_string()),
    //             ]
    //             .into_iter(),
    //         );
    //     }
    //     command
    // }

    // fn gencmd_set_timeout(&self) -> Command {
    //     let mut command = self.get_cfg_base_command("TIMEOUT");
    //     command.parameters.extend(
    //         vec![
    //             CommandParamater::Numerical(self.pkg_timeout as u32),
    //             CommandParamater::Numerical(self.retry_times as u32),
    //         ]
    //         .into_iter(),
    //     );
    //     if (self.flag & MQTTFlags::TIMEOUT_NOTICE).bits() != 0 {
    //         command.parameters.push(CommandParamater::Numerical(1));
    //     } else {
    //         command.parameters.push(CommandParamater::Numerical(0));
    //     }
    //     command
    // }

    // fn gencmd_set_session(&self) -> Command {
    //     let mut command = self.get_cfg_base_command("SESSION");
    //     if (self.flag & MQTTFlags::CLEAN_SESSION).bits() != 0 {
    //         command.parameters.push(CommandParamater::Numerical(1))
    //     }
    //     command
    // }

    // fn gencmd_set_keepalive(&self) -> Command {
    //     let mut command = self.get_cfg_base_command("KEEPALIVE");
    //     command
    //         .parameters
    //         .push(CommandParamater::Numerical(self.keep_alive as u32));
    //     command
    // }

    // fn gencmd_set_version(&self) -> Command {
    //     let mut command = self.get_cfg_base_command("VERSION");
    //     command
    //         .parameters
    //         .push(CommandParamater::Numerical(self.version as u32));
    //     command
    // }

    // fn gencmd_set_dataformat(&self) -> Command {
    //     let mut command = self.get_cfg_base_command("dataformat");

    //     if (self.flag & MQTTFlags::SEND_FORMAT).bits() != 0 {
    //         command.parameters.push(CommandParamater::Numerical(1))
    //     } else {
    //         command.parameters.push(CommandParamater::Numerical(0))
    //     }

    //     if (self.flag & MQTTFlags::RECV_FORMAT).bits() != 0 {
    //         command.parameters.push(CommandParamater::Numerical(1))
    //     } else {
    //         command.parameters.push(CommandParamater::Numerical(0))
    //     }
    //     command
    // }

    // fn gencmd_set_echomode(&self) -> Command {
    //     let mut command = self.get_cfg_base_command("echomode");
    //     if (self.flag & MQTTFlags::ECHO_MODE).bits() != 0 {
    //         command.parameters.push(CommandParamater::Numerical(1))
    //     } else {
    //         command.parameters.push(CommandParamater::Numerical(0))
    //     }
    //     command
    // }

    // fn sync_with_check(&mut self, cmd: Command) -> Result<BC26Status, BC26Status> {
    //     let mut cmd = Rc::new(RefCell::new(LiveCommand::init(cmd)));
    //     let res = match &mut self.bc26.lock() {
    //         Ok(e) => e.poll_cmd(cmd.clone(), 300),
    //         Err(e) => Err(BC26Status::ErrMutexError),
    //     };
    //     if let Err(_) = res {
    //         return Err(BC26Status::Timeout);
    //     } else {
    //         if !cmd.borrow().is_ok() {
    //             return Err(BC26Status::ErrUnexpectedError);
    //         }
    //     }
    //     return Err(BC26Status::ErrUnexpectedError);
    // }

    // pub fn flush_cfg(&mut self) -> Result<BC26Status, BC26Status> {
    //     if let Err(e) = self.sync_with_check(self.gencmd_set_will()) {
    //         return Err(e);
    //     }
    //     if let Err(e) = self.sync_with_check(self.gencmd_set_dataformat()) {
    //         return Err(e);
    //     }
    //     if let Err(e) = self.sync_with_check(self.gencmd_set_echomode()) {
    //         return Err(e);
    //     }
    //     if let Err(e) = self.sync_with_check(self.gencmd_set_keepalive()) {
    //         return Err(e);
    //     }
    //     if let Err(e) = self.sync_with_check(self.gencmd_set_session()) {
    //         return Err(e);
    //     }
    //     if let Err(e) = self.sync_with_check(self.gencmd_set_timeout()) {
    //         return Err(e);
    //     }
    //     if let Err(e) = self.sync_with_check(self.gencmd_set_version()) {
    //         return Err(e);
    //     }
    //     return Ok(BC26Status::Ok);
    // }
}

#[cfg(test)]
mod tests {
    use super::{MQTTFlags, MQTT};
    use std::println;
    fn getMqttObj() -> MQTT {
        MQTT {
            session: 3,
            host: "foo.bar.com",
            port: 12345,
            will_qos: 2,
            will_topic: "foo",
            will_msg: "msg",
            retry_times: 5,
            pkg_timeout: 7,
            version: 3,
            flag: MQTTFlags::WILL | MQTTFlags::KEEP_ALIVE | MQTTFlags::WILL_RETAIN,
            ..Default::default()
        }
    }

    #[test]
    pub fn test_set_will() {
        let mut a = getMqttObj();
        assert_eq!(
            a.gencmd_set_will().construct().as_str(),
            r#"AT+QMTCFG="WILL",3,1,2,1,"foo","msg""#.to_owned() + "\r\n"
        );
        a.flag = a.flag - MQTTFlags::WILL;
        assert_eq!(
            a.gencmd_set_will().construct().as_str(),
            r#"AT+QMTCFG="WILL",3"#.to_owned() + "\r\n"
        );
    }
    #[test]
    pub fn test_set_timeout() {
        let a = getMqttObj();
        assert_eq!(
            a.gencmd_set_timeout().construct().as_str(),
            r#"AT+QMTCFG="TIMEOUT",3,7,5,0"#.to_owned() + "\r\n"
        );
    }

    #[test]
    pub fn test_set_session() {
        let mut a = getMqttObj();
        assert_eq!(
            a.gencmd_set_session().construct().as_str(),
            r#"AT+QMTCFG="SESSION",3"#.to_owned() + "\r\n"
        );
        a.flag = a.flag | MQTTFlags::CLEAN_SESSION;
        assert_eq!(
            a.gencmd_set_session().construct().as_str(),
            r#"AT+QMTCFG="SESSION",3,1"#.to_owned() + "\r\n"
        );
    }

    #[test]
    pub fn test_set_keepalive() {
        let mut a = getMqttObj();
        a.keep_alive = 12;
        assert_eq!(
            a.gencmd_set_keepalive().construct().as_str(),
            r#"AT+QMTCFG="KEEPALIVE",3,12"#.to_owned() + "\r\n"
        );
    }
    #[test]
    pub fn test_set_version() {
        let mut a = getMqttObj();
        a.version = 3;
        assert_eq!(
            a.gencmd_set_version().construct().as_str(),
            r#"AT+QMTCFG="VERSION",3,3"#.to_owned() + "\r\n"
        );
        a.version = 4;
        assert_eq!(
            a.gencmd_set_version().construct().as_str(),
            r#"AT+QMTCFG="VERSION",3,4"#.to_owned() + "\r\n"
        );
    }

    #[test]
    pub fn test_setdataformat() {
        let mut a = getMqttObj();
        assert_eq!(
            a.gencmd_set_dataformat().construct().as_str(),
            r#"AT+QMTCFG="dataformat",3,0,0"#.to_owned() + "\r\n"
        );
        a.flag = a.flag | MQTTFlags::SEND_FORMAT;
        assert_eq!(
            a.gencmd_set_dataformat().construct().as_str(),
            r#"AT+QMTCFG="dataformat",3,1,0"#.to_owned() + "\r\n"
        );
        a.flag = a.flag | MQTTFlags::SEND_FORMAT | MQTTFlags::RECV_FORMAT;
        assert_eq!(
            a.gencmd_set_dataformat().construct().as_str(),
            r#"AT+QMTCFG="dataformat",3,1,1"#.to_owned() + "\r\n"
        );
    }

    #[test]
    pub fn test_echomode() {
        let mut a = getMqttObj();
        assert_eq!(
            a.gencmd_set_echomode().construct().as_str(),
            r#"AT+QMTCFG="echomode",3,0"#.to_owned() + "\r\n"
        );
        a.flag = a.flag | MQTTFlags::ECHO_MODE;
        assert_eq!(
            a.gencmd_set_echomode().construct().as_str(),
            r#"AT+QMTCFG="echomode",3,1"#.to_owned() + "\r\n"
        );
    }
}
