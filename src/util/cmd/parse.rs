use super::TCommandType;
use super::TCommandType::*;

pub fn get_cmd_type(raw_str: &str) -> Option<TCommandType> {
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_type() {
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
