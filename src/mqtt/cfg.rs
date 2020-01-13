use super::{MQTT,MQTTFlags};
use crate::cffi::import;
use alloc::string::String;
use alloc::boxed::Box;
use alloc::fmt::format;

static BaseCommand:&str = "AT+QMTCFG=";

impl MQTT {

    pub fn set_will(&self)->Box<String>{
        let mut a:Box<String> = Box::new(
            String::from(format!("{:}\"WILL\",{:}",BaseCommand,self.session))
        );
        if (self.flag & MQTTFlags::will)== MQTTFlags::will{
            //will_fg
            a.push_str(",1");
            a.push_str(format!(",{:}",self.will_qos).as_str() );
            if (self.flag & MQTTFlags::will_retain) == MQTTFlags::will_retain{
                a.push_str(",1");
            } else{
                a.push_str(",0");
            }
            a.push_str(format!(",{:}",self.will_topic).as_str() );
            a.push_str(format!(",{:}",self.will_msg).as_str() );
        }
        a
    }
    pub fn set_timeout(&self)->Box<String>{
        let mut base = Box::new(
            String::from(format!("{:}\"TIMEOUT\",{:}",BaseCommand,self.session))
        );
        base.push_str(format!(",{:}",self.pkg_timeout).as_str() );
        base.push_str(format!(",{:}",self.retry_times).as_str() );
        if (self.flag&MQTTFlags::timeout_notice)==MQTTFlags::timeout_notice{
            base.push_str(",1");
        } else{
            base.push_str(",0");
        }
        base
    }

    pub fn set_session(&self)->Box<String>{
        let mut base = Box::new(
            String::from(format!("{:}\"SESSION\",{:}",BaseCommand,self.session))
        );
        if (self.flag & MQTTFlags::clean_session) == MQTTFlags::clean_session{
            base.push_str(",1");
        }
        base
    }

    pub fn set_keepalive(&self)->Box<String>{
        let mut base = Box::new(
            String::from(format!("{:}\"KEEPALIVE\",{:}",BaseCommand,self.session))
        );
        base.push_str(format!(",{:}",self.keep_alive).as_str() );
        base
    }

    pub fn set_version(&self)->Box<String>{
        let mut base = Box::new(
            String::from(format!("{:}\"VERSION\",{:}",BaseCommand,self.session))
        );
        base.push_str(format!(",{:}",self.version).as_str() );
        base
    }

    pub fn set_dataformat(&self)->Box<String>{
        let mut base = Box::new(
            String::from(format!("{:}\"dataformat\",{:}",BaseCommand,self.session))
        );

        if (self.flag&MQTTFlags::send_format)==MQTTFlags::send_format{
            base.push_str(",1");
        } else{
            base.push_str(",0");
        }

        if (self.flag&MQTTFlags::recv_format)==MQTTFlags::recv_format{
            base.push_str(",1");
        } else{
            base.push_str(",0");
        }

        base
    }

    pub fn set_echomode(&self)->Box<String>{
        let mut base = Box::new(
            String::from(format!("{:}\"echomode\",{:}",BaseCommand,self.session))
        );
        if (self.flag&MQTTFlags::echo_mode)==MQTTFlags::echo_mode{
            base.push_str(",1");
        } else{
            base.push_str(",0");
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
    pub fn test_set_will(){
        let mut a = getMqttObj();
        assert_eq!(a.set_will().as_str(),r#"AT+QMTCFG="WILL",3,1,2,1,foo,msg"#);
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

