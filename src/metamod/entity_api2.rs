//! [abi::META_FUNCTIONS::pfnGetEntityAPI2] and [abi::META_FUNCTIONS::pfnGetEntityAPI2_Post] implementations

use std::{cmp::max, ptr::null_mut, sync::LazyLock};

use cstr::cstr;

use crate::{
    metamod::{abi, adapter, entry, meta, meta_const},
    util::log,
};

use super::msgs;

static FUNCTION_TABLE: LazyLock<abi::DLL_FUNCTIONS> = LazyLock::new(|| { abi::DLL_FUNCTIONS {
    pfnSpawn: Some(spawn),
    pfnClientConnect: Some(client_connect),
    pfnClientCommand: Some(client_command),
    pfnServerActivate: Some(server_activate),
    ..Default::default()
}});

pub extern "C" fn get_api(
    function_table: *mut abi::DLL_FUNCTIONS,
    interface_version: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    log::debug("get_entity_api2");
    if function_table.is_null() {
        log::error("metamod function table is null");
        return 0;
    }
    if unsafe { *interface_version } != abi::INTERFACE_VERSION as i32 {
        log::error("half life interface version mismatch");
        return 0;
    }

    unsafe {
        *function_table = *FUNCTION_TABLE;
    }

    1
}

extern "C" fn spawn(_entity: *mut abi::edict_t) -> i32 {
    entry::meta_setup();

    meta::set_result(meta_const::RESULT_IGNORED);
    0
}

extern "C" fn client_connect(
    entity: *mut abi::edict_t,
    name: *const ::std::os::raw::c_char,
    _address: *const ::std::os::raw::c_char,
    _reject_reason: *mut ::std::os::raw::c_char,
) -> abi::qboolean {
    if let (Some(player_id), player_name) =
        (meta::get_ent_index(entity), meta::c_char_to_string(name))
    {
        adapter::console_debug(&format!(
            "player with id {} and name {} is connecting",
            player_id, player_name
        ));
    }
    meta::set_result(meta_const::RESULT_IGNORED);

    1
}

extern "C" fn client_command(entity: *mut abi::edict_t) {
    if let (Some(player_id), Some(api)) = (meta::get_ent_index(entity), meta::ENG_FUNCS.get()) {
        if let (Some(argv_fn), Some(argc_fn)) = (api.pfnCmd_Argv, api.pfnCmd_Argc) {
            let args_num = max(unsafe { argc_fn() }, 2); // need at least cmd and first argument, if first argument not exist then empty string is provided by engine
            let mut arguments: Vec<String> = Vec::with_capacity(args_num as usize);
            for n in 0..args_num {
                let arg = unsafe { argv_fn(n) };
                if !arg.is_null() {
                    let str_arg = meta::c_char_to_string(arg);
                    arguments.push(str_arg);
                }
            }

            meta::set_result(entry::client_command(player_id, arguments));
            return;
        }
    }
    meta::set_result(meta_const::RESULT_IGNORED);
}

extern "C" fn server_activate(
    _entity_list: *mut abi::edict_t,
    _entity_count: ::std::os::raw::c_int,
    _client_max: ::std::os::raw::c_int,
) {
    if let None = unsafe { msgs::TEXT_MSG } {
        unsafe { msgs::TEXT_MSG = meta::get_user_msg_id(cstr!("TextMsg"), null_mut()) };
    }
    else if let None = unsafe { msgs::BAR_TIME } {
        unsafe { msgs::BAR_TIME = meta::get_user_msg_id(cstr!("BarTime"), null_mut()) };
    }
    else if let None = unsafe { msgs::CUR_WEAPON } {
        unsafe { msgs::CUR_WEAPON = meta::get_user_msg_id(cstr!("CurWeapon"), null_mut()) };
    }
    else if let None = unsafe { msgs::DAMAGE } {
        unsafe { msgs::DAMAGE = meta::get_user_msg_id(cstr!("Damage"), null_mut()) };
    }
    else if let None = unsafe { msgs::DEATH_MSG } {
        unsafe { msgs::DEATH_MSG = meta::get_user_msg_id(cstr!("DeathMsg"), null_mut()) };
    }
    else if let None = unsafe { msgs::TEAM_INFO } {
        unsafe { msgs::TEAM_INFO = meta::get_user_msg_id(cstr!("TeamInfo"), null_mut()) };
    }
    else if let None = unsafe { msgs::WEAPON_LIST } {
        unsafe { msgs::WEAPON_LIST = meta::get_user_msg_id(cstr!("WeaponList"), null_mut()) };
    }
    else if let None = unsafe { msgs::MOTD } {
        unsafe { msgs::MOTD = meta::get_user_msg_id(cstr!("MOTD"), null_mut()) };
    }
    else if let None = unsafe { msgs::SERVER_NAME } {
        unsafe { msgs::SERVER_NAME = meta::get_user_msg_id(cstr!("ServerName"), null_mut()) };
    }
    else if let None = unsafe { msgs::HEALTH } {
        unsafe { msgs::HEALTH = meta::get_user_msg_id(cstr!("Health"), null_mut()) };
    }
    else if let None = unsafe { msgs::BATTERY } {
        unsafe { msgs::BATTERY = meta::get_user_msg_id(cstr!("Battery"), null_mut()) };
    }
    else if let None = unsafe { msgs::SHOW_MENU } {
        unsafe { msgs::SHOW_MENU = meta::get_user_msg_id(cstr!("ShowMenu"), null_mut()) };
    }
    else if let None = unsafe { msgs::SEND_AUDIO } {
        unsafe { msgs::SEND_AUDIO = meta::get_user_msg_id(cstr!("SendAudio"), null_mut()) };
    }
    else if let None = unsafe { msgs::AMMO_X } {
        unsafe { msgs::AMMO_X = meta::get_user_msg_id(cstr!("AmmoX"), null_mut()) };
    }
    else if let None = unsafe { msgs::SCORE_INFO } {
        unsafe { msgs::SCORE_INFO = meta::get_user_msg_id(cstr!("ScoreInfo"), null_mut()) };
    }
    else if let None = unsafe { msgs::VGUI_MENU } {
        unsafe { msgs::VGUI_MENU = meta::get_user_msg_id(cstr!("VGUIMenu"), null_mut()) };
    }
    else if let None = unsafe { msgs::AMMO_PICKUP } {
        unsafe { msgs::AMMO_PICKUP = meta::get_user_msg_id(cstr!("AmmoPickup"), null_mut()) };
    }
    else if let None = unsafe { msgs::WEAP_PICKUP } {
        unsafe { msgs::WEAP_PICKUP = meta::get_user_msg_id(cstr!("WeapPickup"), null_mut()) };
    }
    else if let None = unsafe { msgs::RESET_HUD } {
        unsafe { msgs::RESET_HUD = meta::get_user_msg_id(cstr!("ResetHUD"), null_mut()) };
    }
    else if let None = unsafe { msgs::ROUND_TIME } {
        unsafe { msgs::ROUND_TIME = meta::get_user_msg_id(cstr!("RoundTime"), null_mut()) };
    }
    else if let None = unsafe { msgs::SAY_TEXT } {
        unsafe { msgs::SAY_TEXT = meta::get_user_msg_id(cstr!("SayText"), null_mut()) };
    }
    else if let None = unsafe { msgs::INIT_HUD } {
        unsafe { msgs::INIT_HUD = meta::get_user_msg_id(cstr!("InitHUD"), null_mut()) };
    }
}

static FUNCTION_TABLE_POST: LazyLock<abi::DLL_FUNCTIONS> = LazyLock::new(|| {abi::DLL_FUNCTIONS {
    pfnClientPutInServer: Some(client_put_in_server_post),
    pfnServerActivate: Some(server_activate_post),
    ..Default::default()
}});

pub extern "C" fn get_api_post(
    function_table: *mut abi::DLL_FUNCTIONS,
    interface_version: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    log::debug("get_entity_api2_post");
    if function_table.is_null() {
        log::error("[POST] metamod function table is null");
        return 0;
    }
    if unsafe { *interface_version } != abi::INTERFACE_VERSION as i32 {
        log::error("[POST] half life interface version mismatch");
        return 0;
    }

    unsafe {
        *function_table = *FUNCTION_TABLE_POST;
    }

    1
}

extern "C" fn server_activate_post(
    _entity_list: *mut abi::edict_t,
    _entity_count: ::std::os::raw::c_int,
    _client_max: ::std::os::raw::c_int,
) {
    entry::meta_init();
}

extern "C" fn client_put_in_server_post(entity: *mut abi::edict_t) {
    if let Some(player_id) = meta::get_ent_index(entity) {
        adapter::console_debug(&format!("player with id {} joined", player_id));
    }
}
