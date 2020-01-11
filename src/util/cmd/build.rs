use super::TCommandType::*;
use super::Command;

pub fn build_cmd_basic(command: &'static mut str) -> Option<Command> {
    return Some(Command {
        cmd_type: Basic,
        base: command,
        identifier: None,
        parameters: None,
    });
}

pub fn build_cmd_spar(command: &'static mut str) -> Option<Command> {
    return Some(Command {
        cmd_type: SPar,
        base: command,
        identifier: None,
        parameters: None,
    });
}

pub fn build_cmd_ext_exec(command: &'static mut str) -> Option<Command> {
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