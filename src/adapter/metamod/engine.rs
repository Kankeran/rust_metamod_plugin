//! [abi::META_FUNCTIONS::pfnGetEngineFunctions] and [abi::META_FUNCTIONS::pfnGetEngineFunctions_Post] implementations

use super::{abi, entry, meta, meta_const, msgs};
use crate::util::log;
use std::sync::LazyLock;

static ENG_FUNCS: LazyLock<abi::enginefuncs_t> = LazyLock::new(|| abi::enginefuncs_t {
    pfnMessageBegin: Some(message_begin),
    pfnWriteByte: Some(write_byte),
    pfnWriteChar: Some(write_char),
    pfnWriteShort: Some(write_short),
    pfnWriteLong: Some(write_long),
    pfnWriteAngle: Some(write_angle),
    pfnWriteCoord: Some(write_coord),
    pfnWriteString: Some(write_string),
    pfnWriteEntity: Some(write_entity),
    pfnMessageEnd: Some(message_end),
    pfnSetClientKeyValue: Some(set_client_key_value),
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

extern "C" fn set_client_key_value(
    client_index: ::std::os::raw::c_int,
    info_buffer: *mut ::std::os::raw::c_char,
    key: *const ::std::os::raw::c_char,
    value: *const ::std::os::raw::c_char,
) {
    let info_buffer = meta::c_char_to_string(info_buffer);
    let key = meta::c_char_to_string(key);
    let value = meta::c_char_to_string(value);

    crate::adapter::entry::set_client_key_value(client_index, info_buffer, key, value);
    meta::set_result(meta_const::RESULT_IGNORED);
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
    log::debug("get_engine_functions_post");
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
        "VoiceMask" => unsafe { msgs::VOICE_MASK = Some(meta::result_orig_ret()) },
        "ReqState" => unsafe { msgs::REQ_STATE = Some(meta::result_orig_ret()) },
        "Geiger" => unsafe { msgs::GEIGER = Some(meta::result_orig_ret()) },
        "Flashlight" => unsafe { msgs::FLASHLIGHT = Some(meta::result_orig_ret()) },
        "FlashBat" => unsafe { msgs::FLASH_BAT = Some(meta::result_orig_ret()) },
        "Train" => unsafe { msgs::TRAIN = Some(meta::result_orig_ret()) },
        "HudTextPro" => unsafe { msgs::HUD_TEXT_PRO = Some(meta::result_orig_ret()) },
        "HudText" => unsafe { msgs::HUD_TEXT = Some(meta::result_orig_ret()) },
        "ViewMode" => unsafe { msgs::VIEW_MODE = Some(meta::result_orig_ret()) },
        "GameTitle" => unsafe { msgs::GAME_TITLE = Some(meta::result_orig_ret()) },
        "ScoreAttrib" => unsafe { msgs::SCORE_ATTRIB = Some(meta::result_orig_ret()) },
        "TeamScore" => unsafe { msgs::TEAM_SCORE = Some(meta::result_orig_ret()) },
        "GameMode" => unsafe { msgs::GAME_MODE = Some(meta::result_orig_ret()) },
        "ItemPickup" => unsafe { msgs::ITEM_PICKUP = Some(meta::result_orig_ret()) },
        "HideWeapon" => unsafe { msgs::HIDE_WEAPON = Some(meta::result_orig_ret()) },
        "SetFOV" => unsafe { msgs::SET_FOV = Some(meta::result_orig_ret()) },
        "ScreenShake" => unsafe { msgs::SCREEN_SHAKE = Some(meta::result_orig_ret()) },
        "ScreenFade" => unsafe { msgs::SCREEN_FADE = Some(meta::result_orig_ret()) },
        "Money" => unsafe { msgs::MONEY = Some(meta::result_orig_ret()) },
        "ArmorType" => unsafe { msgs::ARMOR_TYPE = Some(meta::result_orig_ret()) },
        "BlinkAcct" => unsafe { msgs::BLINK_ACCT = Some(meta::result_orig_ret()) },
        "StatusValue" => unsafe { msgs::STATUS_VALUE = Some(meta::result_orig_ret()) },
        "StatusText" => unsafe { msgs::STATUS_TEXT = Some(meta::result_orig_ret()) },
        "StatusIcon" => unsafe { msgs::STATUS_ICON = Some(meta::result_orig_ret()) },
        "ReloadSound" => unsafe { msgs::RELOAD_SOUND = Some(meta::result_orig_ret()) },
        "Crosshair" => unsafe { msgs::CROSSHAIR = Some(meta::result_orig_ret()) },
        "NVGToggle" => unsafe { msgs::NVG_TOGGLE = Some(meta::result_orig_ret()) },
        "Radar" => unsafe { msgs::RADAR = Some(meta::result_orig_ret()) },
        "Spectator" => unsafe { msgs::SPECTATOR = Some(meta::result_orig_ret()) },
        "TutorText" => unsafe { msgs::TUTOR_TEXT = Some(meta::result_orig_ret()) },
        "TutorLine" => unsafe { msgs::TUTOR_LINE = Some(meta::result_orig_ret()) },
        "TutorState" => unsafe { msgs::TUTOR_STATE = Some(meta::result_orig_ret()) },
        "TutorClose" => unsafe { msgs::TUTOR_CLOSE = Some(meta::result_orig_ret()) },
        "AllowSpec" => unsafe { msgs::ALLOW_SPEC = Some(meta::result_orig_ret()) },
        "BombDrop" => unsafe { msgs::BOMB_DROP = Some(meta::result_orig_ret()) },
        "BombPickup" => unsafe { msgs::BOMB_PICKUP = Some(meta::result_orig_ret()) },
        "ClCorpse" => unsafe { msgs::CL_CORPSE = Some(meta::result_orig_ret()) },
        "HostagePos" => unsafe { msgs::HOSTAGE_POS = Some(meta::result_orig_ret()) },
        "HostageK" => unsafe { msgs::HOSTAGE_K = Some(meta::result_orig_ret()) },
        "HLTV" => unsafe { msgs::HLTV = Some(meta::result_orig_ret()) },
        "SpecHealth" => unsafe { msgs::SPEC_HEALTH = Some(meta::result_orig_ret()) },
        "ForceCam" => unsafe { msgs::FORCE_CAM = Some(meta::result_orig_ret()) },
        "ADStop" => unsafe { msgs::AD_STOP = Some(meta::result_orig_ret()) },
        "ReceiveW" => unsafe { msgs::RECEIVE_W = Some(meta::result_orig_ret()) },
        "CZCareer" => unsafe { msgs::CZ_CAREER = Some(meta::result_orig_ret()) },
        "CZCareerHUD" => unsafe { msgs::CZ_CAREER_HUD = Some(meta::result_orig_ret()) },
        "ShadowIdx" => unsafe { msgs::SHADOW_IDX = Some(meta::result_orig_ret()) },
        "TaskTime" => unsafe { msgs::TASK_TIME = Some(meta::result_orig_ret()) },
        "Scenario" => unsafe { msgs::SCENARIO = Some(meta::result_orig_ret()) },
        "BotVoice" => unsafe { msgs::BOT_VOICE = Some(meta::result_orig_ret()) },
        "BuyClose" => unsafe { msgs::BUY_CLOSE = Some(meta::result_orig_ret()) },
        "SpecHealth2" => unsafe { msgs::SPEC_HEALTH2 = Some(meta::result_orig_ret()) },
        "BarTime2" => unsafe { msgs::BAR_TIME2 = Some(meta::result_orig_ret()) },
        "ItemStatus" => unsafe { msgs::ITEM_STATUS = Some(meta::result_orig_ret()) },
        "Location" => unsafe { msgs::LOCATION = Some(meta::result_orig_ret()) },
        "BotProgress" => unsafe { msgs::BOT_PROGRESS = Some(meta::result_orig_ret()) },
        "Brass" => unsafe { msgs::BRASS = Some(meta::result_orig_ret()) },
        "Fog" => unsafe { msgs::FOG = Some(meta::result_orig_ret()) },
        "ShowTimer" => unsafe { msgs::SHOW_TIMER = Some(meta::result_orig_ret()) },
        "HudTextArgs" => unsafe { msgs::HUD_TEXT_ARGS = Some(meta::result_orig_ret()) },
        msg_name => log::debug(&format!(
            "message name: {msg_name} | ID: {}",
            meta::result_orig_ret::<i32>()
        )),
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
