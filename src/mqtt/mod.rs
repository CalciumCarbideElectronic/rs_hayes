mod opt;
use bitflags::bitflags;
use crate::bc26::BC26;

pub mod cfg;
pub mod connect;
pub mod publish;

bitflags!{
    pub struct MQTTFlags: u8{
        const WILL = (1<<0);
        const WILL_RETAIN =  (1<<1);
        const TIMEOUT_NOTICE = (1<<2);
        const CLEAN_SESSION= (1<<3);
        const KEEP_ALIVE = (1<<4);
        const SEND_FORMAT = (1<<5);
        const RECV_FORMAT = (1<<6);
        const ECHO_MODE= (1<<7);
    }
}

#[derive(Debug)]
pub struct MQTT{
    BC26:Option<&'static BC26>,
    session: u8,
    host: &'static str,
    port: u16,
    pkg_timeout: u8,
    retry_times: u8,
    will_qos :u8,
    version :u8,
    keep_alive: u16,
    will_topic: &'static str,
    will_msg: &'static str,
    flag: MQTTFlags
}

impl Default for MQTT{
    fn default()->MQTT{
        MQTT{
            BC26:None ,
            session: 0,
            host: "",
            port: 0,
            pkg_timeout: 0,
            retry_times:0,
            will_qos :0,
            version :0,
            keep_alive:0,
            will_topic: "",
            will_msg: "",
            flag: MQTTFlags::empty()
        }
    }
}

impl MQTT{
}

#[cfg(test)]
mod tests{
    use super::{MQTT,MQTTFlags};
    use std::println;
    fn getMqttObj()->MQTT{
        MQTT{
            session:0,
            host:"foo.bar.com",
            port:12345,
            will_qos:2,
            will_topic:"foo",
            will_msg:"msg",
            flag: MQTTFlags::WILL | MQTTFlags::KEEP_ALIVE,
            ..Default::default()}
    }

    #[test]
    pub fn test_foo(){
        let a = getMqttObj();
        println!("{:?}",a);

    }

}

