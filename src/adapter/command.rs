use std::sync::RwLock;

use crate::{
    adapter::{api, convert},
    metamod::meta_util,
};

static COMMANDS: RwLock<Vec<Command>> = RwLock::new(Vec::new());

pub fn add_client_command(command: Command) {
    COMMANDS.write().unwrap().push(command);
}

pub fn handle_client_command(id: i32, arguments: &Vec<String>) -> i32 {
    let mut result = meta_util::RESULT_IGNORED;
    if let Ok(cmds) = COMMANDS.read() {
        let command = &arguments[0];
        let argument = &arguments[1];
        for cmd in cmds.iter() {
            if cmd.equal(command, argument) {
                let res = (cmd.callback)(id, arguments);
                if let api::Return::Supercede = res {
                    return meta_util::RESULT_SUPERCEDE;
                }
                let res = convert::convert_result(res);
                if res > result {
                    result = res
                }
            }
        }
    }

    result
}

pub struct Command {
    command: String,
    argument: Option<String>,
    callback: fn(i32, &Vec<String>) -> api::Return,
}

impl Command {
    pub fn new(
        command: String,
        argument: Option<String>,
        callback: fn(i32, &Vec<String>) -> api::Return,
    ) -> Command {
        Command {
            command,
            argument,
            callback,
        }
    }

    pub fn equal(&self, command: &str, argument: &str) -> bool {
        match self.argument.as_ref() {
            Some(arg) => self.command.eq(command) && arg.eq(argument),
            None => self.command.eq(command),
        }
    }
}
