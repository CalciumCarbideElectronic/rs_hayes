use super::{MQTTFlags, MQTT};
use crate::bc26::cmd::{
    de::{from_resp_vec, Deserializer},
    process::LiveCommand,
    Command, CommandForm, CommandParamater,
};
use crate::cffi::import::DebugS;
use crate::constant::{restype::QMTPUBResponse, BC26Status};
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use core::cell::RefCell;

impl MQTT {
    pub fn qmtpub_write(
        &self,
        conn_id: u8,
        msg_id: u16,
        qos: u8,
        retain: bool,
        topic: &str,
        msg: &str,
    ) -> Result<QMTPUBResponse, BC26Status> {
        let mut publish = LiveCommand::new(Command {
            key: "QMTPUB",
            asyncResp: true,
            form: CommandForm::ExtWrite,
            parameters: vec![
                CommandParamater::Numerical(conn_id as u32),
                CommandParamater::Numerical(msg_id as u32),
                CommandParamater::Numerical(qos as u32),
                CommandParamater::Numerical(retain as u32),
                CommandParamater::Literal(String::from(topic)),
                CommandParamater::Literal(String::from(msg)),
            ],
        });
        match &mut self.bc26.lock() {
            Ok(e) => {
                e.poll_cmd(publish.clone(), 5000)?;
                Ok(from_resp_vec::<QMTPUBResponse>(&publish.borrow().response)?)
            }
            _ => Err(BC26Status::ErrMutexError),
        }
    }
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
    pub fn test_cmd_publish() {
        let mut a = getMqttObj();
        assert_eq!(
            a.cmd_publish(65533, 2, false, "foo", "hello,world")
                .as_str(),
            r#"AT+QMTPUB=3,65533,2,0,"foo","hello,world""#
        );
        // assert_eq!(a.cmd_publish(65533,2,false,"foo","hello,world").as_str(),
        // r#"AT+QMTPUB=3,65533,2,0,"foo","68656c6c6f2c776f726c64""#);
    }
}
