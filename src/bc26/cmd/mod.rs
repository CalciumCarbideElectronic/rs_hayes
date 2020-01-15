pub mod parse;
pub mod process;
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
pub enum CommandForm{
    AT,
    ExtTest,
    ExtWrite,
    ExtRead,
    ExtExec
}

#[derive(Clone,Debug)]
pub struct Command {
    pub key: &'static str,
    pub parameters: Option<Vec<CommandParamater>>,
    pub asyncResp: bool,
    pub form : CommandForm
}


impl Command{
    pub fn construct(&self)->String{
        match self.form{
            CommandForm::AT=> format!("AT{:}",self.key),
            CommandForm::ExtExec=>format!("AT+{:}",self.key),
            CommandForm::ExtRead=>format!("AT+{:}?",self.key),
            CommandForm::ExtTest=>format!("AT+{:}=?",self.key),
            CommandForm::ExtWrite=>format!("AT+{:}={:}",self.key,
                match self.parameters{
                    Some(para)=> para.iter().map(|e|match e{
                    CommandParamater::Literal(l)=>format!(r#""{:}""#,l),
                    CommandParamater::Numerical(d)=>format!(r#"{:}"#,d),
                    }).collect::<Vec<String>>().join(","),
                    None=>String::new()
                })
        }
    }
}

#[cfg(test)]
mod test{
    use super::{Command,CommandParamater,CommandForm};

    fn  getSyncCommand()->Command{
        Command{
            asyncResp:false,
            key:"QATWAKEUP",
            form: CommandForm:ExtExec,
            parameters: vec![
                CommandParamater::Numerical(1)
            ]
        }
    }

    #[test]
    fn test_as_test(){
        let mut c = getSyncCommand();
        c.form=CommandForm::ExtTest;
        assert_eq!(c.construct().as_str(),"AT+QATWAKEUP=?")
    }

    #[test]
    fn test_as_read(){
        let mut c = getSyncCommand();
        c.form=CommandForm::ExtRead;
        assert_eq!(c.construct().as_str(),"AT+QATWAKEUP?")
    }

    #[test]
    fn test_as_exec(){
        let mut c = getSyncCommand();
        c.form=CommandForm::ExtExec;
        assert_eq!(c.construct().as_str(),"AT+QATWAKEUP")
    }

    #[test]
    fn test_as_write(){
        let mut c = getSyncCommand();
        c.form=CommandForm::ExtWrite;

        assert_eq!(c.construct().as_str(),"AT+QATWAKEUP=1");
        c.parameters.push(
            CommandParamater::Literal("foo".to_string())
        );
        assert_eq!(c.construct().as_str(),r#"AT+QATWAKEUP=1,"foo""#);
    }
}

