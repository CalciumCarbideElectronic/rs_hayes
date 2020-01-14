use super::{MQTT,MQTTFlags};
use crate::cffi::import;
use crate::bc26::cmd::{CommandParamater,Command};
use core::borrow::Borrow;
use alloc::string::String;
use alloc::boxed::Box;
use alloc::fmt::format;

static BaseCommand:&str = "AT+QMTCFG";

impl MQTT {
    fn getCfgBaseCommand<'a>(&'a self,tag:&'a str)->Command<'a>{
        Command{
            base:BaseCommand,
            parameters: vec![
                CommandParamater::literal(tag),
                CommandParamater::numerical(self.session as u32)
            ]
        }
    }

    pub fn set_will(&self)->Box<String>{
        let mut command = self.getCfgBaseCommand("WILL");
        if (self.flag & MQTTFlags::will).bits()!=0{
            command.parameters.extend(vec![
                CommandParamater::numerical(1),//will_fg
                CommandParamater::numerical(self.will_qos as u32)//qos
            ].into_iter());
            if (self.flag & MQTTFlags::will_retain) == MQTTFlags::will_retain{
                command.parameters.push(
                    CommandParamater::numerical(1) //reatin
                );
            } else{
                command.parameters.push(
                    CommandParamater::numerical(0) //retain
                );
            }
            command.parameters.extend(vec![
                CommandParamater::literal(self.will_topic),
                CommandParamater::literal(self.will_msg)
            ].into_iter());
        }
        command.as_write()
    }
    pub fn set_timeout(&self)->Box<String>{
        let mut command = self.getCfgBaseCommand("TIMEOUT");
        command.parameters.extend(vec![
            CommandParamater::numerical(self.pkg_timeout as u32),
            CommandParamater::numerical(self.retry_times as u32)
        ].into_iter());
        if (self.flag&MQTTFlags::timeout_notice).bits()!=0{
            command.parameters.push(
                CommandParamater::numerical(1));
        } else{
            command.parameters.push(
                CommandParamater::numerical(0));
        }
        command.as_write()
    }

    pub fn set_session(&self)->Box<String>{
        let mut command = self.getCfgBaseCommand("SESSION");
        if (self.flag & MQTTFlags::clean_session).bits()!=0{
            command.parameters.push(
                CommandParamater::numerical(1)
            )
        }
        command.as_write()
    }

    pub fn set_keepalive(&self)->Box<String>{
        let mut command = self.getCfgBaseCommand("KEEPALIVE");
        command.parameters.push(
            CommandParamater::numerical(self.keep_alive as u32)
        );
        command.as_write()
    }

    pub fn set_version(&self)->Box<String>{
        let mut command = self.getCfgBaseCommand("VERSION");
        command.parameters.push(
            CommandParamater::numerical(self.version as u32)
        );
        command.as_write()
    }

    pub fn set_dataformat(&self)->Box<String>{
        let mut command = self.getCfgBaseCommand("dataformat");

        if (self.flag&MQTTFlags::send_format).bits()!=0{
            command.parameters.push( CommandParamater::numerical(1))
        } else{
            command.parameters.push( CommandParamater::numerical(0))
        }

        if (self.flag&MQTTFlags::recv_format).bits()!=0{
            command.parameters.push( CommandParamater::numerical(1))
        } else{
            command.parameters.push( CommandParamater::numerical(0))
        }
        command.as_write()
    }

    pub fn set_echomode(&self)->Box<String>{
        let mut command = self.getCfgBaseCommand("echomode");
        if (self.flag&MQTTFlags::echo_mode).bits()!=0{
            command.parameters.push( CommandParamater::numerical(1))
        } else{
            command.parameters.push( CommandParamater::numerical(0))
        }
        command.as_write()
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
    pub fn test_set_will(){
        let mut a = getMqttObj();
        assert_eq!(a.set_will().as_str(),r#"AT+QMTCFG="WILL",3,1,2,1,"foo","msg""#);
        a.flag=a.flag-MQTTFlags::will;
        assert_eq!(a.set_will().as_str(),r#"AT+QMTCFG="WILL",3"#);
    }
    #[test]
    pub fn test_set_timeout(){
        let mut a = getMqttObj();
        assert_eq!(a.set_timeout().as_str(),r#"AT+QMTCFG="TIMEOUT",3,7,5,0"#);
    }

    #[test]
    pub fn test_set_session(){
        let mut a = getMqttObj();
        assert_eq!(a.set_session().as_str(),r#"AT+QMTCFG="SESSION",3"#);
        a.flag=a.flag | MQTTFlags::clean_session;
        assert_eq!(a.set_session().as_str(),r#"AT+QMTCFG="SESSION",3,1"#);
    }

    #[test]
    pub fn test_set_keepalive(){
        let mut a = getMqttObj();
        a.keep_alive=12;
        assert_eq!(a.set_keepalive().as_str(),r#"AT+QMTCFG="KEEPALIVE",3,12"#);
    }
    #[test]
    pub fn test_set_version(){
        let mut a = getMqttObj();
        a.version=3;
        assert_eq!(a.set_version().as_str(),r#"AT+QMTCFG="VERSION",3,3"#);
        a.version=4;
        assert_eq!(a.set_version().as_str(),r#"AT+QMTCFG="VERSION",3,4"#);
    }

    #[test]
    pub fn test_setdataformat(){
        let mut a = getMqttObj();
        assert_eq!(a.set_dataformat().as_str(),r#"AT+QMTCFG="dataformat",3,0,0"#);
        a.flag= a.flag|MQTTFlags::send_format;
        assert_eq!(a.set_dataformat().as_str(),r#"AT+QMTCFG="dataformat",3,1,0"#);
        a.flag= a.flag|MQTTFlags::send_format|MQTTFlags::recv_format;
        assert_eq!(a.set_dataformat().as_str(),r#"AT+QMTCFG="dataformat",3,1,1"#);
    }

    #[test]
    pub fn test_echomode(){
        let mut a = getMqttObj();
        assert_eq!(a.set_echomode().as_str(),r#"AT+QMTCFG="echomode",3,0"#);
        a.flag= a.flag|MQTTFlags::echo_mode;
        assert_eq!(a.set_echomode().as_str(),r#"AT+QMTCFG="echomode",3,1"#);
    }


}

