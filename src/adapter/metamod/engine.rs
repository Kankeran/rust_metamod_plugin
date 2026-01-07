//! [abi::META_FUNCTIONS::pfnGetEngineFunctions] and [abi::META_FUNCTIONS::pfnGetEngineFunctions_Post] implementations

use super::{abi, entry, meta, meta_const, msgs};
use crate::util::log;
use std::sync::LazyLock;

static ENG_FUNCS: LazyLock<abi::enginefuncs_t> = LazyLock::new(|| abi::enginefuncs_t {
    pfnMessageBegin: Some(message_begin),
    pfnMessageEnd: Some(message_end),
    pfnWriteByte: Some(write_byte),
    pfnWriteChar: Some(write_char),
    pfnWriteShort: Some(write_short),
    pfnWriteLong: Some(write_long),
    pfnWriteAngle: Some(write_angle),
    pfnWriteCoord: Some(write_coord),
    pfnWriteString: Some(write_string),
    pfnWriteEntity: Some(write_entity),
    ..Default::default()
});

pub extern "C" fn get_functions(
    functions_from_engine: *mut abi::enginefuncs_t,
    interface_version: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    log::debug("get_engine_functions");
    if functions_from_engine.is_null() {
        log::error("engine functions is null");
        return 0;
    }
    if unsafe { *interface_version } != abi::ENGINE_INTERFACE_VERSION as i32 {
        log::error("engine interface version mismatch");
        return 0;
    }

    unsafe {
        *functions_from_engine = *ENG_FUNCS;
    }

    1
}

extern "C" fn message_begin(
    msg_dest: ::std::os::raw::c_int,
    msg_type: ::std::os::raw::c_int,
    origin: *const f32,
    ed: *mut abi::edict_t,
) {
    meta::set_result(entry::message_begin(msg_dest, msg_type, origin, ed));
}

extern "C" fn write_byte(value: ::std::os::raw::c_int) {
    meta::set_result(entry::write_byte(value));
}

extern "C" fn write_char(value: ::std::os::raw::c_int) {
    meta::set_result(entry::write_char(value));
}

extern "C" fn write_short(value: ::std::os::raw::c_int) {
    meta::set_result(entry::write_short(value));
}

extern "C" fn write_long(value: ::std::os::raw::c_int) {
    meta::set_result(entry::write_long(value));
}

extern "C" fn write_angle(value: f32) {
    meta::set_result(entry::write_angle(value));
}

extern "C" fn write_coord(value: f32) {
    meta::set_result(entry::write_coord(value));
}

extern "C" fn write_string(value: *const ::std::os::raw::c_char) {
    meta::set_result(entry::write_string(value));
}

extern "C" fn write_entity(value: ::std::os::raw::c_int) {
    meta::set_result(entry::write_entity(value));
}

extern "C" fn message_end() {
    meta::set_result(entry::message_end());
}

static ENG_FUNCS_POST: LazyLock<abi::enginefuncs_t> = LazyLock::new(|| abi::enginefuncs_t {
    pfnRegUserMsg: Some(reg_user_msg_post),
    pfnMessageBegin: Some(message_begin_post),
    pfnMessageEnd: Some(message_end_post),
    pfnWriteByte: Some(write_byte_post),
    pfnWriteChar: Some(write_char_post),
    pfnWriteShort: Some(write_short_post),
    pfnWriteLong: Some(write_long_post),
    pfnWriteAngle: Some(write_angle_post),
    pfnWriteCoord: Some(write_coord_post),
    pfnWriteString: Some(write_string_post),
    pfnWriteEntity: Some(write_entity_post),
    ..Default::default()
});

pub extern "C" fn get_functions_post(
    functions_from_engine: *mut abi::enginefuncs_t,
    interface_version: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    log::debug("get_engine_functions");
    if functions_from_engine.is_null() {
        log::error("engine functions is null");
        return 0;
    }
    if unsafe { *interface_version } != abi::ENGINE_INTERFACE_VERSION as i32 {
        log::error("engine interface version mismatch");
        return 0;
    }

    unsafe {
        *functions_from_engine = *ENG_FUNCS_POST;
    }

    1
}

extern "C" fn reg_user_msg_post(
    name: *const ::std::os::raw::c_char,
    _size: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    let name = meta::c_char_to_string(name);

    match name.as_str() {
        "TextMsg" => unsafe { msgs::TEXT_MSG = Some(meta::result_orig_ret()) },
        "BarTime" => unsafe { msgs::BAR_TIME = Some(meta::result_orig_ret()) },
        "CurWeapon" => unsafe { msgs::CUR_WEAPON = Some(meta::result_orig_ret()) },
        "Damage" => unsafe { msgs::DAMAGE = Some(meta::result_orig_ret()) },
        "DeathMsg" => unsafe { msgs::DEATH_MSG = Some(meta::result_orig_ret()) },
        "TeamInfo" => unsafe { msgs::TEAM_INFO = Some(meta::result_orig_ret()) },
        "WeaponList" => unsafe { msgs::WEAPON_LIST = Some(meta::result_orig_ret()) },
        "MOTD" => unsafe { msgs::MOTD = Some(meta::result_orig_ret()) },
        "ServerName" => unsafe { msgs::SERVER_NAME = Some(meta::result_orig_ret()) },
        "Health" => unsafe { msgs::HEALTH = Some(meta::result_orig_ret()) },
        "Battery" => unsafe { msgs::BATTERY = Some(meta::result_orig_ret()) },
        "ShowMenu" => unsafe { msgs::SHOW_MENU = Some(meta::result_orig_ret()) },
        "SendAudio" => unsafe { msgs::SEND_AUDIO = Some(meta::result_orig_ret()) },
        "AmmoX" => unsafe { msgs::AMMO_X = Some(meta::result_orig_ret()) },
        "ScoreInfo" => unsafe { msgs::SCORE_INFO = Some(meta::result_orig_ret()) },
        "VGUIMenu" => unsafe { msgs::VGUI_MENU = Some(meta::result_orig_ret()) },
        "AmmoPickup" => unsafe { msgs::AMMO_PICKUP = Some(meta::result_orig_ret()) },
        "WeapPickup" => unsafe { msgs::WEAP_PICKUP = Some(meta::result_orig_ret()) },
        "ResetHUD" => unsafe { msgs::RESET_HUD = Some(meta::result_orig_ret()) },
        "RoundTime" => unsafe { msgs::ROUND_TIME = Some(meta::result_orig_ret()) },
        "SayText" => unsafe { msgs::SAY_TEXT = Some(meta::result_orig_ret()) },
        "InitHUD" => unsafe { msgs::INIT_HUD = Some(meta::result_orig_ret()) },
        _ => (),
    };

    meta::set_result(meta_const::RESULT_IGNORED);
    0
}

extern "C" fn message_begin_post(
    msg_dest: ::std::os::raw::c_int,
    msg_type: ::std::os::raw::c_int,
    origin: *const f32,
    ed: *mut abi::edict_t,
) {
    meta::set_result(entry::message_begin_post(msg_dest, msg_type, origin, ed))
}

extern "C" fn write_byte_post(value: ::std::os::raw::c_int) {
    meta::set_result(entry::write_byte_post(value));
}

extern "C" fn write_char_post(value: ::std::os::raw::c_int) {
    meta::set_result(entry::write_char_post(value));
}

extern "C" fn write_short_post(value: ::std::os::raw::c_int) {
    meta::set_result(entry::write_short_post(value));
}

extern "C" fn write_long_post(value: ::std::os::raw::c_int) {
    meta::set_result(entry::write_long_post(value));
}

extern "C" fn write_angle_post(value: f32) {
    meta::set_result(entry::write_angle_post(value));
}

extern "C" fn write_coord_post(value: f32) {
    meta::set_result(entry::write_coord_post(value));
}

extern "C" fn write_string_post(value: *const ::std::os::raw::c_char) {
    meta::set_result(entry::write_string_post(value));
}

extern "C" fn write_entity_post(value: ::std::os::raw::c_int) {
    meta::set_result(entry::write_entity_post(value));
}

extern "C" fn message_end_post() {
    meta::set_result(entry::message_end_post());
}
