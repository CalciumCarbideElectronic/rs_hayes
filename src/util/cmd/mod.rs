pub mod parse;
pub mod build;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum TCommandType {
    Basic,
    SPar,
    ExtTest,
    ExtRead,
    ExtWrite,
    ExtExec,
}

#[derive( PartialEq)]
pub struct Command {
    pub cmd_type: TCommandType,
    pub base: &'static str,
    pub identifier: Option<&'static str>,
    pub parameters: Option<&'static str>,
}

impl Command{
    pub fn from_string(raw_str:&'static mut str)->Option<Command>{
        match parse::get_cmd_type(raw_str.as_ref()) {
            Some(Basic) => build::build_cmd_basic(raw_str),
            Some(SPar) => build::build_cmd_spar(raw_str),
            Some(ExtExec) => build::build_cmd_ext_exec(raw_str),
            Some(ExtWrite) => build::build_cmd_ext_exec(raw_str),
            _ => None
        }
    }
}

