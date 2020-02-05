use super::MQTT;
use crate::bc26::cmd::{process::LiveCommand, Command, CommandForm, CommandParamater};
use crate::constant::BC26Status;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use core::cell::RefCell;

impl MQTT {
    pub fn open_network(
        &mut self,
        conn_id: u16,
        host_name: &str,
        port: u16,
    ) -> Result<BC26Status, BC26Status> {
        let opencmd = LiveCommand::new(Command {
            key: "QMTOPEN",
            asyncResp: true,
            form: CommandForm::ExtWrite,
            parameters: vec![
                CommandParamater::Numerical(conn_id as u32),
                CommandParamater::Literal(String::from(host_name)),
                CommandParamater::Numerical(port as u32),
            ],
        });
        match &mut self.bc26.lock() {
            Ok(e) => e.poll_cmd(opencmd, 5000),
            _ => Err(BC26Status::ErrMutexError),
        }
    }

    pub fn open_connection(
        &mut self,
        conn_id: u16,
        client_id: &str,
    ) -> Result<BC26Status, BC26Status> {
        let conncmd = LiveCommand::new(Command {
            key: "QMTCONN",
            asyncResp: true,
            form: CommandForm::ExtWrite,
            parameters: vec![
                CommandParamater::Numerical(conn_id as u32),
                CommandParamater::Literal(String::from(client_id)),
            ],
        });
        match &mut self.bc26.lock() {
            Ok(e) => e.poll_cmd(conncmd, 5000),
            _ => Err(BC26Status::ErrMutexError),
        }
    }

    // pub fn update_connection_info(&mut self,connid)->Result<,BC26Status>{

    // }
}


#[cfg(test)]
mod tests {
    use super::super::{MQTTFlags, MQTT};
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
}
