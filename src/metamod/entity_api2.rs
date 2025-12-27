use crate::{
    metamod::{abi, adapter, meta},
    util::log,
};

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
    pfnClientConnect: None,
    pfnClientDisconnect: None,
    pfnClientKill: None,
    pfnClientPutInServer: Some(client_put_in_server),
    pfnClientCommand: Some(client_command),
    pfnClientUserInfoChanged: None,
    pfnServerActivate: None,
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
    log::info("get_entity_api2");
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

    adapter::console_log("entity api2 loaded");

    1
}

extern "C" fn client_command(entity: *mut abi::edict_t) {
    let _ = entity;
    if let Some(api) = meta::ENG_FUNCS.get() {
        if let (Some(argv_fn), Some(argc_fn)) = (api.pfnCmd_Argv, api.pfnCmd_Argc) {
            let cmd = unsafe { argv_fn(0) };
            let arg = unsafe { argv_fn(1) };
            let argc = unsafe { argc_fn() };
            if !cmd.is_null() && !arg.is_null() {
                let str_cmd = meta::c_char_to_string(cmd);
                let str_arg = meta::c_char_to_string(arg);
                log::info(&format!("{} {}, argc: {}", str_cmd, str_arg, argc));
            }
            meta::set_meta_result(abi::META_RES_MRES_IGNORED);
            return;
        }
    }
}

extern "C" fn client_put_in_server(entity: *mut abi::edict_t) {
    if let Some(player_id) = meta::get_ent_index(entity) {
        adapter::console_log(&format!("player with id {} joined", player_id));
    }
}
