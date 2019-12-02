use crate::util::TCommandType::{ExtTest, Basic, SPar, ExtRead, ExtWrite, ExtExec};

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
        match get_cmd_type(raw_str.as_ref()) {
            Some(Basic) => build_cmd_basic(raw_str),
            Some(SPar) => build_cmd_spar(raw_str),
            Some(ExtExec) => build_cmd_ext_exec(raw_str),
            Some(ExtWrite) => build_cmd_ext_exec(raw_str),
            _ => None
        }
    }
}


fn get_cmd_type(raw_str: &str) -> Option<TCommandType> {
    // test if it is extended  AT command
    if let Some(e) = raw_str.find("AT+") {
        if raw_str.find("=?").is_some() {
            return Some(ExtTest);
        } else if raw_str.find("?").is_some() {
            return Some(ExtRead);
        } else if raw_str.find("=").is_some() {
            return Some(ExtWrite);
        }
        return Some(ExtExec);
    }

    // test if it is extended  SParameter command
    if let Some(e) = raw_str.find("ATS") {
        if e == 0 {
            return Some(SPar);
        }
    }

    // test if it is Basic AT Command
    if let Some(e) = raw_str.find("AT") {
        if e == 0 {
            return Some(Basic);
        }
    }

    return None;
}

fn build_cmd_basic(command: &'static mut str) -> Option<Command> {
    return Some(Command {
        cmd_type: Basic,
        base: command,
        identifier: None,
        parameters: None,
    });
}

fn build_cmd_spar(command: &'static mut str) -> Option<Command> {
    return Some(Command {
        cmd_type: SPar,
        base: command,
        identifier: None,
        parameters: None,
    });
}

fn build_cmd_ext_exec(command: &'static mut str) -> Option<Command> {
    let s = command.split("+");
    let h = s.clone().nth(0);
    let t = s.clone().nth(1);
    match (h, t) {
        (Some(head), Some(tail)) =>
            Some(Command {
                cmd_type: ExtExec,
                base: head,
                identifier: Some(tail),
                parameters: None,
            }),
        (_, _) => None
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_cmd_type() {
        assert_eq!(get_cmd_type("AT+QRST=?"), Some(ExtTest));
        assert_eq!(get_cmd_type("AT+QRST=1"), Some(ExtWrite));
        assert_eq!(get_cmd_type("AT+QSPCHSC?"), Some(ExtRead));
        assert_eq!(get_cmd_type("AT+QRELLOCK"), Some(ExtExec));
        assert_eq!(get_cmd_type("ATE"), Some(Basic));
        assert_eq!(get_cmd_type("AT"), Some(Basic));
        assert_eq!(get_cmd_type("ATS"), Some(SPar));
        assert_eq!(get_cmd_type("TATP"), None);
    }
}

