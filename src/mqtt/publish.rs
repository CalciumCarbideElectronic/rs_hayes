use super::{MQTT,MQTTFlags};
use alloc::string::String;
use alloc::boxed::Box;
// use hex::encode;

impl MQTT{
    pub fn cmd_publish(&self,msg_id:u16,qos:u8,retain:bool,topic:&str,msg:&str)->Box<String>{
        let mut base = Box::new(
            format!(r#"AT+QMTPUB={:},{:},{:},{:},"{:}""#,
            self.session,msg_id,qos,retain as u8, topic
        ));
        if( self.flag & MQTTFlags::SEND_FORMAT )==MQTTFlags::SEND_FORMAT{
            // base.push_str( format!(r#","{:}""#,encode(msg)).as_str())
        }else{
            base.push_str( format!(r#","{:}""#,msg).as_str());
        }
        base
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
    pub fn test_cmd_publish(){
        let mut a = getMqttObj();
        assert_eq!(a.cmd_publish(65533,2,false,"foo","hello,world").as_str(),
        r#"AT+QMTPUB=3,65533,2,0,"foo","hello,world""#);
        a.flag=a.flag| MQTTFlags::send_format;

        // assert_eq!(a.cmd_publish(65533,2,false,"foo","hello,world").as_str(),
        // r#"AT+QMTPUB=3,65533,2,0,"foo","68656c6c6f2c776f726c64""#);
    }
}


