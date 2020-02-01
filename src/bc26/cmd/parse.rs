use crate::constant::errtype::ErrCode;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::{Command, Response, Standard};

impl Command {
    pub fn parse_line(payload: &str) -> Response {
        let payload = payload
            .chars()
            .filter(|e| *e != '\r' && *e != '\n' && *e != '"')
            .collect::<String>();

        if payload.len() == 0 {
            return Response::Empty;
        }
        if payload.starts_with('+') {
            if let Some(idx) = payload.find(':') {
                let (first, last) = payload.get(1..).unwrap().split_at(idx);
                let key = first.chars().take(first.len() - 1).collect::<String>();
                if key == String::from("CME ERROR") {
                    return Response::Error(match last.trim().parse::<u32>() {
                        Err(_) => None,
                        Ok(code) => Some(ErrCode::from(code)),
                    });
                } else {
                    return Response::Standard(Standard {
                        key: key,
                        parameter: last
                            .trim()
                            .split(",")
                            .map(|e| e.to_string())
                            .collect::<Vec<String>>(),
                    });
                }
            }
        }

        if payload == "OK" {
            return Response::OK;
        }
        if payload == "Error" {
            return Response::Error(None);
        }
        return Response::Error(None);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[cfg(test)]
    use hex::decode;
    use std::fmt;

    #[test]
    fn test_parse_standard() {
        let mut r = Standard {
            key: "asd".to_string(),
            parameter: vec!["foo1".to_string(), "foo2".to_string(), "foo3".to_string()],
        };
        assert_eq!(
            Command::parse_line(r#"+asd:"foo1","foo2",foo3"#),
            Response::Standard(r)
        );

        r = Standard {
            key: "asd".to_string(),
            parameter: vec!["foo1".to_string(), "foo2".to_string(), "foo3".to_string()],
        };
        assert_eq!(
            Command::parse_line("+asd:foo1,foo2,\"foo3\"\r\n"),
            Response::Standard(r)
        );

        r = Standard {
            key: "CESQ".to_string(),
            parameter: vec!["36", "99", "255", "255", "12", "53"]
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
        };
        assert_eq!(
            Command::parse_line("+CESQ: 36,99,255,255,12,53\r\n"),
            Response::Standard(r)
        );

        r = Standard {
            key: "QENG".to_string(),
            parameter: vec![
                "0", "2506", "2", "62", "6923252", "-84", "-10", "-74", "2", "5", "69C9", "0", "90",
            ]
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>(),
        };
        assert_eq!(
            Command::parse_line(r#"+QENG: 0,2506,2,62,"6923252",-84,-10,-74,2,5,"69C9",0,90"#),
            Response::Standard(r)
        );
    }
    #[test]
    fn test_parse_ok() {
        assert_eq!(Command::parse_line("OK"), Response::OK);
        assert_eq!(Command::parse_line("OK\r\n"), Response::OK);
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(Command::parse_line("Error"), Response::Error(None));
        assert_eq!(Command::parse_line("Error\r\n"), Response::Error(None));
        assert_eq!(Command::parse_line("+CME ERROR: 14"), Response::Error(Some(ErrCode::USIMBusy)));
        assert_eq!(Command::parse_line("+CME ERROR: 99"), Response::Error(Some(ErrCode::MetaError)));
    }

    #[test]
    fn test_realcase_parse_CIMI() {
        //Response of AT+CIMI
        match decode("0d0a3436303034323333373530393837350d0a0d0a4f4b0d0a") {
            Ok(bytes) => {
                let resps = std::str::from_utf8(bytes.as_slice()).unwrap();
                let mut line = resps.split("\r\n");
                assert_eq!(Command::parse_line(line.next().unwrap()), Response::Empty);
                assert_eq!(
                    Command::parse_line(line.next().unwrap()),
                    Response::Genric("460042337509875".to_string())
                );
                assert_eq!(Command::parse_line(line.next().unwrap()), Response::Empty);
                assert_eq!(Command::parse_line(line.next().unwrap()), Response::OK);
            }
            Err(_) => panic!("wrong"),
        }
    }

    #[test]
    fn test_realcase_parse_sng() {
        match decode("0d0a2b4347534e3a203836363937313033303339333634340d0a0d0a4f4b0d0a") {
            Ok(bytes) => {
                let resp = std::str::from_utf8(&bytes).unwrap();
                let mut lines = resp
                    .split("\r\n")
                    .filter(|e| e.len() > 0)
                    .collect::<Vec<&str>>();
                assert_eq!(
                    Command::parse_line(lines[0]),
                    Response::Standard(Standard {
                        key: "CGSN".to_string(),
                        parameter: vec!["866971030393644".to_string()]
                    })
                );
                assert_eq!(Command::parse_line(lines[1]), Response::OK);
            }
            Err(_) => {
                panic!("wrong");
            }
        }
    }
}
