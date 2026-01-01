//! [abi::META_FUNCTIONS::pfnGetEntityAPI2] and [abi::META_FUNCTIONS::pfnGetEntityAPI2_Post] implementations

use std::{ffi::CString, ptr::null_mut, result};

use cstr::cstr;

use crate::{
    metamod::{abi, adapter, entry, meta::{self, c_char_to_string}, meta_util},
    util::log,
};

use super::msgs;

static FUNCTION_TABLE: abi::DLL_FUNCTIONS = abi::DLL_FUNCTIONS {
    pfnGameInit: None,
    pfnSpawn: None,
    pfnThink: None,
    pfnUse: None,
    pfnTouch: None,
    pfnBlocked: None,
    pfnKeyValue: None,
    pfnSave: None,
    pfnRestore: None,
    pfnSetAbsBox: None,
    pfnSaveWriteFields: None,
    pfnSaveReadFields: None,
    pfnSaveGlobalState: None,
    pfnRestoreGlobalState: None,
    pfnResetGlobalState: None,
    pfnClientConnect: Some(client_connect),
    pfnClientDisconnect: None,
    pfnClientKill: None,
    pfnClientPutInServer: None,
    pfnClientCommand: Some(client_command),
    pfnClientUserInfoChanged: None,
    pfnServerActivate: Some(server_activate),
    pfnServerDeactivate: None,
    pfnPlayerPreThink: None,
    pfnPlayerPostThink: None,
    pfnStartFrame: None,
    pfnParmsNewLevel: None,
    pfnParmsChangeLevel: None,
    pfnGetGameDescription: None,
    pfnPlayerCustomization: None,
    pfnSpectatorConnect: None,
    pfnSpectatorDisconnect: None,
    pfnSpectatorThink: None,
    pfnSys_Error: None,
    pfnPM_Move: None,
    pfnPM_Init: None,
    pfnPM_FindTextureType: None,
    pfnSetupVisibility: None,
    pfnUpdateClientData: None,
    pfnAddToFullPack: None,
    pfnCreateBaseline: None,
    pfnRegisterEncoders: None,
    pfnGetWeaponData: None,
    pfnCmdStart: None,
    pfnCmdEnd: None,
    pfnConnectionlessPacket: None,
    pfnGetHullBounds: None,
    pfnCreateInstancedBaselines: None,
    pfnInconsistentFile: None,
    pfnAllowLagCompensation: None,
};

pub extern "C" fn get_api(
    function_table: *mut abi::DLL_FUNCTIONS,
    interface_version: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    log::debug("get_entity_api2");
    if function_table.is_null() {
        adapter::alert("something went wrong");
        return 0;
    }
    if unsafe { *interface_version } != abi::INTERFACE_VERSION as i32 {
        adapter::alert("something went wrong");
        return 0;
    }

    unsafe {
        *function_table = FUNCTION_TABLE;
    }

    adapter::console_debug("entity api2 loaded");

    1
}

extern "C" fn client_connect(
    entity: *mut abi::edict_t,
    name: *const ::std::os::raw::c_char,
    _address: *const ::std::os::raw::c_char,
    _reject_reason: *mut ::std::os::raw::c_char,
) -> abi::qboolean {
    if let (Some(player_id), player_name) = (meta::get_ent_index(entity), c_char_to_string(name)) {
        adapter::console_debug(&format!("player with id {} and name {} is connecting", player_id, player_name));
    }
    meta::set_result(abi::META_RES_MRES_IGNORED);

    1
}

extern "C" fn client_command(entity: *mut abi::edict_t) {

    if let (Some(player_id), Some(api)) = (meta::get_ent_index(entity), meta::ENG_FUNCS.get()) {
        if let (Some(argv_fn), Some(argc_fn)) = (api.pfnCmd_Argv, api.pfnCmd_Argc) {
            let args_num = unsafe { argc_fn() };
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
}

extern "C" fn server_activate(
    _entity_list: *mut abi::edict_t,
    _entity_count: ::std::os::raw::c_int,
    _client_max: ::std::os::raw::c_int,
) {
    if let None = unsafe { msgs::TEXT_MSG } {
        unsafe { msgs::TEXT_MSG = meta::get_user_msg_id(cstr!("TextMsg"), null_mut()) };
    }
}

static FUNCTION_TABLE_POST: abi::DLL_FUNCTIONS = abi::DLL_FUNCTIONS {
    pfnGameInit: None,
    pfnSpawn: None,
    pfnThink: None,
    pfnUse: None,
    pfnTouch: None,
    pfnBlocked: None,
    pfnKeyValue: None,
    pfnSave: None,
    pfnRestore: None,
    pfnSetAbsBox: None,
    pfnSaveWriteFields: None,
    pfnSaveReadFields: None,
    pfnSaveGlobalState: None,
    pfnRestoreGlobalState: None,
    pfnResetGlobalState: None,
    pfnClientConnect: None,
    pfnClientDisconnect: None,
    pfnClientKill: None,
    pfnClientPutInServer: Some(client_put_in_server_post),
    pfnClientCommand: None,
    pfnClientUserInfoChanged: None,
    pfnServerActivate: Some(server_activate_post),
    pfnServerDeactivate: None,
    pfnPlayerPreThink: None,
    pfnPlayerPostThink: None,
    pfnStartFrame: None,
    pfnParmsNewLevel: None,
    pfnParmsChangeLevel: None,
    pfnGetGameDescription: None,
    pfnPlayerCustomization: None,
    pfnSpectatorConnect: None,
    pfnSpectatorDisconnect: None,
    pfnSpectatorThink: None,
    pfnSys_Error: None,
    pfnPM_Move: None,
    pfnPM_Init: None,
    pfnPM_FindTextureType: None,
    pfnSetupVisibility: None,
    pfnUpdateClientData: None,
    pfnAddToFullPack: None,
    pfnCreateBaseline: None,
    pfnRegisterEncoders: None,
    pfnGetWeaponData: None,
    pfnCmdStart: None,
    pfnCmdEnd: None,
    pfnConnectionlessPacket: None,
    pfnGetHullBounds: None,
    pfnCreateInstancedBaselines: None,
    pfnInconsistentFile: None,
    pfnAllowLagCompensation: None,
};

pub extern "C" fn get_api_post(
    function_table: *mut abi::DLL_FUNCTIONS,
    interface_version: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    log::debug("get_entity_api2_post");
    if function_table.is_null() {
        adapter::alert("something went wrong");
        return 0;
    }
    if unsafe { *interface_version } != abi::INTERFACE_VERSION as i32 {
        adapter::alert("something went wrong");
        return 0;
    }

    unsafe {
        *function_table = FUNCTION_TABLE_POST;
    }

    adapter::console_debug("entity api2 post loaded");

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
