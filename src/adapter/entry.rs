use super::command;
use std::sync::OnceLock;

pub static INIT_FUNC: OnceLock<fn()> = OnceLock::new();
pub static PRECACHE_FUNC: OnceLock<fn()> = OnceLock::new();

pub fn adapter_setup() {
    if let Some(precache) = PRECACHE_FUNC.get() {
        precache()
    }
}

pub fn adapter_init() {
    if let Some(init) = INIT_FUNC.get() {
        init()
    }
}

pub fn client_command(id: i32, args: Vec<String>) -> i32 {
    command::handle_client_command(id, &args)
}
