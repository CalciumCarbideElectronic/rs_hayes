mod opt;
use bitflags::bitflags;

pub mod cfg;
pub mod connect;

bitflags!{
    pub struct MQTTFlags: u8{
        const will = (1<<0);
        const will_retain =  (1<<1);
        const timeout_notice = (1<<2);
        const clean_session= (1<<3);
        const keep_alive = (1<<4);
        const send_format = (1<<5);
        const recv_format = (1<<6);
        const echo_mode= (1<<7);
    }
}

#[derive(Debug)]
pub struct MQTT{
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
            flag: MQTTFlags::will | MQTTFlags::keep_alive,
            ..Default::default()}
    }

    #[test]
    pub fn test_foo(){
        let a = getMqttObj();
        println!("{:?}",a);

    }

}

