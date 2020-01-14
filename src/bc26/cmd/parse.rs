use alloc::string::{String,ToString};
use alloc::vec::{Vec};

use super::{Response,Command,Standard};

impl Command{
    pub fn parse_line(payload:&str)->Response{
        let payload = payload.chars()
                        .filter(|e| *e!='\r'&&*e!='\n'&&*e!='"')
                        .collect::<String>();

        if payload.starts_with('+'){
            if let Some(idx)= payload.find(':') {
                let (first,last) =payload.get(1..).unwrap().split_at(idx);
                return Response::Standard(Standard{
                    key:first.chars().take(first.len()-1).collect::<String>(),
                    parameter:last.trim().split(",")
                                .map(|e|e.to_string())
                                .collect::<Vec<String>>()
                })
            }
        }

        if payload=="OK"{
            return Response::OK
        }
        if payload=="Error"{
            return Response::Error
        }
        Response::Genric(payload.to_string())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_standard() {
        let mut r = Standard{
            key:"asd".to_string(),
            parameter:vec![ "foo1".to_string(), "foo2".to_string(), "foo3".to_string(), ]};
        assert_eq!(Command::parse_line(r#"+asd:"foo1","foo2",foo3"#),Response::Standard(r));

        r = Standard{
            key:"asd".to_string(),
            parameter:vec![ "foo1".to_string(), "foo2".to_string(), "foo3".to_string(), ] };
        assert_eq!(Command::parse_line("+asd:foo1,foo2,\"foo3\"\r\n"),Response::Standard(r));

        r = Standard{ key:"CESQ".to_string(),
            parameter:vec![ "36","99","255","255","12","53"]
            .iter().map(|e|e.to_string()).collect::<Vec<String>>() };
        assert_eq!(Command::parse_line("+CESQ: 36,99,255,255,12,53\r\n"),Response::Standard(r));

        r = Standard{
            key:"QENG".to_string(),
            parameter:vec![ "0","2506","2","62","6923252",
            "-84","-10","-74","2","5","69C9","0","90"
            ]
            .iter().map(|e|e.to_string()).collect::<Vec<String>>() };
        assert_eq!(Command::parse_line(r#"+QENG: 0,2506,2,62,"6923252",-84,-10,-74,2,5,"69C9",0,90"#)
        ,Response::Standard(r));
    }
    #[test]
    fn test_parse_ok(){
        assert_eq!(Command::parse_line("OK"),Response::OK);
        assert_eq!(Command::parse_line("OK\r\n"),Response::OK);
    }

    #[test]
    fn test_parse_error(){
        assert_eq!(Command::parse_line("Error"),Response::Error);
        assert_eq!(Command::parse_line("Error\r\n"),Response::Error);
    }
    #[test]
    fn test_parse_Genric(){
        assert_eq!(Command::parse_line("460012345678969"),Response::Genric("460012345678969".to_string()));
        assert_eq!(Command::parse_line("460012345678969\r\n"),Response::Genric("460012345678969".to_string()));
    }
    
}
