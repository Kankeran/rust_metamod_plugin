use std::ffi::CString;

use crate::{
    adapter::{command, entry},
    metamod::{meta, meta_util},
};

pub enum Return {
    Ignored,
    Handled,
    HandledMain,
    Override,
    Supercede,
}

pub fn setup_entry(init: fn(), precache: fn()) -> Result<(), fn()> {
    entry::INIT_FUNC.set(init)?;
    entry::PRECACHE_FUNC.set(precache)
}

pub fn register_client_command(
    command: String,
    argument: Option<String>,
    callback: fn(i32, &Vec<String>) -> Return,
) {
    command::add_client_command(command::Command::new(command, argument, callback))
}

pub enum PrintMode {
    PrintNotify,
    PrintConsole,
    PrintChat,
    PrintCenter,
}

pub fn client_print(id: i32, mode: PrintMode, msg: &str) {
    if let Some(entity) = meta::get_ent_by_index(id) {
        let point = msg.floor_char_boundary(188);
        let (msg, _) = msg.split_at(point);
        if let Ok(cmsg) = CString::new(msg) {
            let msg_dest = match mode {
                PrintMode::PrintNotify => meta_util::PRINT_NOTIFY,
                PrintMode::PrintConsole => meta_util::PRINT_CONSOLE,
                PrintMode::PrintChat => meta_util::PRINT_CHAT,
                PrintMode::PrintCenter => meta_util::PRINT_CENTER,
            };
            meta_util::client_say_text(entity, msg_dest, cmsg.as_c_str());
        }
    }
}

pub fn console_log(msg: &str) {
    if let Ok(cmsg) = CString::new(msg) {
        meta::console_log(cmsg.as_c_str());
    }
}

pub fn console_debug(msg: &str) {
    #[cfg(debug_assertions)]
    if let Ok(cmsg) = CString::new(msg) {
        meta::console_log(cmsg.as_c_str());
    }
}
