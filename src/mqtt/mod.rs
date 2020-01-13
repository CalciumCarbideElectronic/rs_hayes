mod opt;
use bitflags::bitflags;

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
impl MQTT{

    pub fn set_will(){ }
    pub fn set_timeout(){}
    pub fn set_keepalive(){}
    pub fn set_version(){}
    pub fn set_format(){}
    pub fn set_echomode(){}

}

#[cfg(test)]
mod tests{
    use super::{MQTT};
}

