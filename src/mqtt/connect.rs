use super::MQTT;
use alloc::boxed::Box;
use alloc::string::String;

impl MQTT {
    pub fn set_open(&self) -> Box<String> {
        Box::new(format!(
            r#"AT+QMTOPEN={:},"{:}",{:}"#,
            self.session, self.host, self.port
        ))
    }
    pub fn set_close(&self) -> Box<String> {
        Box::new(format!(r#"AT+QMTCLOSE={:}"#, self.session))
    }
    pub fn set_conn(&self, c_id: &str, username: &str, password: &str) -> Box<String> {
        if username.len() > 0 {
            Box::new(format!(
                r#"AT+QMTCONN={:},"{:}","{:}","{:}""#,
                self.session, c_id, username, password
            ))
        } else {
            Box::new(format!(r#"AT+QMTCONN={:},"{:}""#, self.session, c_id))
        }
    }
    pub fn set_disconn(&self) -> Box<String> {
        Box::new(format!(r#"AT+QMTDISC={:}"#, self.session))
    }
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

    #[test]
    pub fn test_set_open() {
        let a = getMqttObj();
        assert_eq!(a.set_open().as_str(), r#"AT+QMTOPEN=3,"foo.bar.com",12345"#);
    }
    #[test]
    pub fn test_set_close() {
        let a = getMqttObj();
        assert_eq!(a.set_close().as_str(), r#"AT+QMTCLOSE=3"#);
    }
    #[test]
    pub fn test_set_conn() {
        let a = getMqttObj();
        assert_eq!(
            a.set_conn("fooo", "1234", "4567").as_str(),
            r#"AT+QMTCONN=3,"fooo","1234","4567""#
        );
        assert_eq!(
            a.set_conn("fooo", "", "").as_str(),
            r#"AT+QMTCONN=3,"fooo""#
        );
    }
    #[test]
    pub fn test_set_disconn() {
        let a = getMqttObj();
        assert_eq!(a.set_disconn().as_str(), r#"AT+QMTDISC=3"#);
    }
}
