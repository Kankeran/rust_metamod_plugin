use std::ffi::CString;

use super::metamod::{meta};
use super::{
    command, entry,
    messages::{DHudMessage, HudMessage, TextMessage},
};

pub use super::messages::Color;
pub use super::messages::HudChannel;
pub use super::messages::HudStyle;
pub use super::messages::Point;
pub use super::messages_handler::block_msg;
// pub use meta_api::EdictPtr;
pub use super::common_types::Return;
pub use super::common_types::BlockMode;
pub use super::common_types::PrintMode;
pub use super::common_types::UserMsgs;
pub use super::menu::show_menu;
pub use super::menu::Menu;
pub use super::menu::NumKeys;
pub use super::text_message_handler::register_text_message;

pub fn setup_entry(init: fn(), precache: fn()) -> Result<(), fn()> {
    entry::INIT_FUNC.set(init)?;
    entry::PRECACHE_FUNC.set(precache)
}

pub fn register_client_command(
    command: String,
    argument: Option<String>,
    callback: Box<dyn Fn(i32, &Vec<String>) -> Return + Send + Sync>,
) {
    command::add_client_command(command::Command::new(command, argument, callback))
}

pub fn client_print(id: Option<i32>, mode: PrintMode, msg: String) {
    TextMessage::new(id, mode, msg).send();
}

pub fn show_hud_message(id: Option<i32>, style: HudStyle, channel: HudChannel, message: String) {
    HudMessage::new(id, style, channel, message).send();
}

pub fn show_dhud_message(id: Option<i32>, style: HudStyle, message: String) {
    DHudMessage::new(id, style, message).send();
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
