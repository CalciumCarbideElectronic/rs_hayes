use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::String;
use core::iter::Iterator;

pub enum CommandParamater<'a>{ 
    literal(&'a str),
    numerical( u32),
}

pub struct Command<'a> {
    pub base: &'a str,
    pub parameters:  Vec< CommandParamater<'a>>
}

impl<'a> Command<'a>{
    pub fn as_test(&self)->Box<String>{
        Box::new(String::from(format!("{:}=?",self.base)))
    }
    pub fn as_read(&self)->Box<String>{
        Box::new(String::from(format!("{:}?",self.base)))
    }
    pub fn as_write(&self)->Box<String>{
        let paramStr = self.parameters.iter().map(|e|match e{
            CommandParamater::literal(l)=>format!(r#""{:}""#,l),
            CommandParamater::numerical(d)=>format!(r#"{:}"#,d),
        }).collect::<Vec<String>>().join(",");
        Box::new(String::from(format!("{:}={:}",self.base,paramStr)))
    }
    pub fn as_exec(&self)->Box<String>{
        Box::new(String::from(format!("{:}",self.base)))
    }
}

#[cfg(test)]
mod test{
    use super::{Command,CommandParamater};

    fn  getCommand<'a>()->Command<'a>{
        Command{
            base:"AT+QATWAKEUP",
            parameters: vec![
                CommandParamater::numerical(1)
            ]
        }
    }

    #[test]
    fn test_as_test(){
        let c = getCommand();
        assert_eq!(c.as_test().as_str(),"AT+QATWAKEUP=?")
    }

    #[test]
    fn test_as_read(){
        let c = getCommand();
        assert_eq!(c.as_read().as_str(),"AT+QATWAKEUP?")
    }

    #[test]
    fn test_as_exec(){
        let c = getCommand();
        assert_eq!(c.as_exec().as_str(),"AT+QATWAKEUP")
    }

    #[test]
    fn test_as_write(){
        let mut c = getCommand();
        assert_eq!(c.as_write().as_str(),"AT+QATWAKEUP=1");
        c.parameters.push(
            CommandParamater::literal("foo")
        );
        assert_eq!(c.as_write().as_str(),r#"AT+QATWAKEUP=1,"foo""#);
    }
}

