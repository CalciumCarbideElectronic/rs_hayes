pub mod parse;
use alloc::{
    vec::Vec,
    boxed::Box,
    string::String,
    collections::BTreeMap
};
use core::iter::Iterator;


#[derive(Clone,Debug)]
pub enum CommandParamater{ 
    Literal(String),
    Numerical( u32),
}

#[derive(Debug,Eq,PartialEq,Clone)]
pub struct Standard{
    pub key: String,
    pub parameter: Vec<String>
}
#[derive(Debug,Eq,PartialEq,Clone)]
pub enum Response{
    OK,
    Error,
    Empty,
    Standard(Standard),
    KVs(BTreeMap<String,String>),
    Genric (String),
}

#[derive(Clone,Debug)]
pub struct Command {
    pub base: &'static str,
    pub key: &'static str,
    pub parameters:  Vec<CommandParamater>
}

impl Command{
    pub fn as_test(&self)->Box<String>{
        Box::new(String::from(format!("{:}=?",self.base)))
    }
    pub fn as_read(&self)->Box<String>{
        Box::new(String::from(format!("{:}?",self.base)))
    }
    pub fn as_write(&self)->Box<String>{
        let param_str = self.parameters.iter().map(|e|match e{
            CommandParamater::Literal(l)=>format!(r#""{:}""#,l),
            CommandParamater::Numerical(d)=>format!(r#"{:}"#,d),
        }).collect::<Vec<String>>().join(",");
        Box::new(String::from(format!("{:}={:}",self.base,param_str)))
    }
    pub fn as_exec(&self)->Box<String>{
        Box::new(String::from(format!("{:}",self.base)))
    }
}

#[cfg(test)]
mod test{
    use super::{Command,CommandParamater};

    fn  getCommand()->Command{
        Command{
            key:"QATWAKEUP",
            base:"AT+QATWAKEUP",
            parameters: vec![
                CommandParamater::Numerical(1)
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
            CommandParamater::Literal("foo".to_string())
        );
        assert_eq!(c.as_write().as_str(),r#"AT+QATWAKEUP=1,"foo""#);
    }
}

