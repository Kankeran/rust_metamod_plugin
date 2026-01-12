use crate::adapter::{common_types::Return, menu};
use std::sync::Mutex;

static COMMANDS: Mutex<Vec<Command>> = Mutex::new(Vec::new());

pub fn add_client_command(command: Command) {
    COMMANDS.lock().unwrap().push(command);
}

pub fn handle_client_command(id: i32, arguments: &Vec<String>) -> Return {
    let mut result = Return::Ignored;
    let command = &arguments[0];
    let argument = &arguments[1];
    if let Ok(cmds) = COMMANDS.lock() {
        for cmd in cmds.iter() {
            if cmd.equal(command, argument) {
                let res = (cmd.callback)(id, arguments);
                if let Return::Supercede = res {
                    return Return::Supercede;
                }
                if result.lt(&res) {
                    result = res
                }
            }
        }
    }

    menu::handle_menu_select(id, command, argument);

    result
}

type CommandCallback = Box<dyn Fn(i32, &Vec<String>) -> Return + Send + Sync + 'static>;

pub struct Command {
    command: String,
    argument: Option<String>,
    callback: CommandCallback,
}

impl Command {
    pub fn new(command: String, argument: Option<String>, callback: CommandCallback) -> Command {
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
