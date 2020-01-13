use super::{MQTT,MQTTFlags};
use crate::cffi::import;
use alloc::string::String;
use alloc::boxed::Box;
use alloc::fmt::format;

impl MQTT {
    pub fn set_open(&self)->Box<String>{
        Box::new(format!(r#"AT+QMTOPEN={:},"{:}",{:}"#,self.session,self.host,self.port))
    }
    pub fn set_close(&self)->Box<String>{
        Box::new(format!(r#"AT+QMTCLOSE={:}"#,self.session))
    }
    pub fn set_conn(&self,cID:&str,username:&str,password:&str)->Box<String>{
        if username.len()>0{
            Box::new(format!(r#"AT+QMTCONN={:},"{:}","{:}","{:}""#
            ,self.session,cID,username,password))
        } else{
            Box::new(format!(r#"AT+QMTCONN={:},"{:}""#
            ,self.session,cID))
        }
    }
    pub fn set_disconn(&self)->Box<String>{
        Box::new(format!(r#"AT+QMTDISC={:}"#,self.session))
    }
}


#[cfg(test)]
mod tests{
    use super::{MQTT,MQTTFlags};
    use std::println;
    fn getMqttObj()->MQTT{
        MQTT{
            session:3,
            host:"foo.bar.com",
            port:12345,
            will_qos:2,
            will_topic:"foo",
            will_msg:"msg",
            retry_times:5,
            pkg_timeout:7,
            version:3,
            flag: MQTTFlags::will |
                  MQTTFlags::keep_alive |
                  MQTTFlags::will_retain,
            ..Default::default()}
    }

    #[test]
    pub fn test_set_open(){
        let mut a = getMqttObj();
        assert_eq!(a.set_open().as_str(),r#"AT+QMTOPEN=3,"foo.bar.com",12345"#);
    }
    #[test]
    pub fn test_set_close(){
        let mut a = getMqttObj();
        assert_eq!(a.set_close().as_str(),r#"AT+QMTCLOSE=3"#);
    }
    #[test]
    pub fn test_set_conn(){
        let mut a = getMqttObj();
        assert_eq!(a.set_conn("fooo","1234","4567").as_str(),r#"AT+QMTCONN=3,"fooo","1234","4567""#);
        assert_eq!(a.set_conn("fooo","","").as_str(),r#"AT+QMTCONN=3,"fooo""#);
    }
    #[test]
    pub fn test_set_disconn(){
        let mut a = getMqttObj();
        assert_eq!(a.set_disconn().as_str(),r#"AT+QMTDISC=3"#);
    }
}
