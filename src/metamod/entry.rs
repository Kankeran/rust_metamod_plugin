use std::sync::OnceLock;

use crate::metamod::{abi, meta};

pub struct EntryFuncs {
    init: fn(),
    setup: fn(),
    client_command: fn(i32, Vec<String>) -> i32,
}

impl EntryFuncs {
    pub fn new(init: fn(), setup: fn(), client_command: fn(i32, Vec<String>)->i32) -> EntryFuncs {
        EntryFuncs {
            init,
            setup,
            client_command,
        }
    }
}

pub static ENTRY_FUNCS: OnceLock<EntryFuncs> = OnceLock::new();

pub static INIT_FUNC: OnceLock<fn()> = OnceLock::new();
pub static SETUP_FUNC: OnceLock<fn()> = OnceLock::new();
pub static CLIENT_COMMAND_FUNC: OnceLock<fn(i32, Vec<String>)> = OnceLock::new();

pub fn meta_init() {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        (entry_funcs.init)()
    }
}

pub fn meta_setup() {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        (entry_funcs.setup)()
    }
}

pub fn client_command(id: i32, args: Vec<String>) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
       return (entry_funcs.client_command)(id, args)
    }

    abi::META_RES_MRES_IGNORED
}
