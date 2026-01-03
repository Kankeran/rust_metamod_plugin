use std::ffi::CString;

use crate::{
    adapter::{command, entry, messages::TextMessage},
    metamod::{meta},
};

pub enum Return {
    Ignored,
    Handled,
    HandledMain,
    Override,
    Supercede,
}

pub enum PrintMode {
    PrintNotify,
    PrintConsole,
    PrintChat,
    PrintCenter,
}

#[derive(Debug)]
pub enum UserMsgs {
    TextMsg,
    BarTime,
    CurWeapon,
    Damage,
    DeathMsg,
    TeamInfo,
    WeaponList,
    MOTD,
    ServerName,
    Health,
    Battery,
    ShowMenu,
    SendAudio,
    AmmoX,
    ScoreInfo,
    VguiMenu,
    AmmoPickup,
    WeapPickup,
    ResetHud,
    RoundTime,
    SayText,
    InitHud,
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

pub fn client_print(id: Option<i32>, mode: PrintMode, msg: &str) {
    TextMessage::new(id, mode, msg.to_owned()).send();
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
