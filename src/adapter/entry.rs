use crate::adapter::ham;

use super::command;

pub fn adapter_setup() {
    crate::plugin::plugin_precache();
}

pub fn adapter_init() {
    crate::plugin::plugin_init();
}

pub fn client_command(id: i32, args: Vec<String>) -> i32 {
    command::handle_client_command(id, &args).to_i32()
}

pub fn set_client_key_value(client_index: i32, info_buffer: String, key: String, value: String) {
    ham::set_client_key_value(client_index, info_buffer, key, value);
}

pub fn free_data() {
    ham::free_hooks();
    command::free_commands();
}
