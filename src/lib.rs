use std::io::Write;
use std::{
    ffi::{CStr, CString, c_char},
    fs::OpenOptions,
    sync::OnceLock,
};
use cstr::cstr;

#[allow(dead_code, nonstandard_style)]
pub mod metamod;

static ENG_FUNCS: OnceLock<metamod::enginefuncs_t> = OnceLock::new();
static META_UTIL_FUNCS: OnceLock<metamod::mutil_funcs_t> = OnceLock::new();
static mut GAME_DLL_FUNCS: *const metamod::gamedll_funcs_t = std::ptr::null();
static mut GLOBALS: *const metamod::globalvars_t = std::ptr::null();
static mut META_GLOBALS: *const metamod::meta_globals_t = std::ptr::null();

fn file_log(msg: &str) {
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("rust_plugin.log")
    {
        let _ = writeln!(f, "{msg}");
    }
}

fn file_log_error(msg: &str) {
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("rust_plugin_error.log")
    {
        let _ = writeln!(f, "{msg}");
    }
}

pub(crate) fn globals() -> &'static metamod::globalvars_t {
    unsafe { &*GLOBALS }
}

pub(crate) fn game_dll_funcs() -> &'static metamod::gamedll_funcs_t {
    unsafe { &*GAME_DLL_FUNCS }
}

pub(crate) fn meta_globals() -> &'static metamod::meta_globals_t {
    unsafe { &*META_GLOBALS }
}

static PRINT_FORMAT: &CStr = cstr!("%s");

fn err_log(msg: &str) {
    if let Some(api) = META_UTIL_FUNCS.get() {
        if let Some(log_fn) = api.pfnLogError {
            if let Ok(cmsg) = CString::new(msg) {
                unsafe {
                    log_fn(
                        &raw mut PLUGIN_INFO,
                        PRINT_FORMAT.as_ptr(),
                        cmsg.as_ptr(),
                    );
                }
            }
        }
    }
}

fn alert(msg: &str) {
    if let Some(api) = ENG_FUNCS.get() {
        if let Some(log_fn) = api.pfnAlertMessage {
            if let Ok(cmsg) = CString::new(msg) {
                unsafe {
                    log_fn(
                        metamod::ALERT_TYPE_at_logged,
                        PRINT_FORMAT.as_ptr(),
                        cmsg.as_ptr(),
                    )
                }
            }
        }
    }
}

#[unsafe(export_name = "GiveFnptrsToDll")]
pub extern "system" fn give_dll_pointers(
    funcs_from_engine: *mut metamod::enginefuncs_t,
    globals_pointer: *mut metamod::globalvars_t,
) {
    file_log("GiveFnptrsToDll");

    ENG_FUNCS.set(unsafe { *funcs_from_engine });

    unsafe {
        GLOBALS = globals_pointer;
    }
}

static PLUGIN_IFVERS: &CStr = cstr!("5:13");
static PLUGIN_NAME: &CStr = cstr!("rust print");
static PLUGIN_VERSION: &CStr = cstr!("1.0.0");
static PLUGIN_DATE: &CStr = cstr!("26.12.2025");
static PLUGIN_AUTHOR: &CStr = cstr!("AwIlL");
static PLUGIN_URL: &CStr = cstr!("-");
static PLUGIN_LOGTAG: &CStr = cstr!("RUST_PRINT");

static mut PLUGIN_INFO: metamod::plugin_info_t = metamod::plugin_info_t {
    ifvers: PLUGIN_IFVERS.as_ptr(),
    name: PLUGIN_NAME.as_ptr(),
    version: PLUGIN_VERSION.as_ptr(),
    date: PLUGIN_DATE.as_ptr(),
    author: PLUGIN_AUTHOR.as_ptr(),
    url: PLUGIN_URL.as_ptr(),
    logtag: PLUGIN_LOGTAG.as_ptr(),
    loadable: metamod::PLUG_LOADTIME_PT_ANYTIME,
    unloadable: metamod::PLUG_LOADTIME_PT_ANYPAUSE,
};

#[unsafe(export_name = "Meta_Query")]
pub extern "C" fn meta_query(
    interface_version: *const ::std::os::raw::c_char,
    plugin_info: *mut *mut metamod::plugin_info_t,
    meta_util_funcs: *mut metamod::mutil_funcs_t,
) -> ::std::os::raw::c_int {
    file_log("Meta_Query");
    unsafe {
        *plugin_info = &raw mut PLUGIN_INFO;
    }

    META_UTIL_FUNCS.set(unsafe { *meta_util_funcs });

    let plugin_required_version = c_char_to_string(unsafe{PLUGIN_INFO.ifvers});
    let engine_ifvers = c_char_to_string(interface_version);

    if engine_ifvers != plugin_required_version {
        alert("something went wrong");
        return 0;
    }

    1
}

static META_FUNCTION_TABLE: metamod::META_FUNCTIONS = metamod::META_FUNCTIONS {
    pfnGetEntityAPI: None,
    pfnGetEntityAPI_Post: None,
    pfnGetEntityAPI2: Some(get_entity_api2),
    pfnGetEntityAPI2_Post: None,
    pfnGetNewDLLFunctions: None,
    pfnGetNewDLLFunctions_Post: None,
    pfnGetEngineFunctions: Some(get_engine_functions),
    pfnGetEngineFunctions_Post: None,
};

#[unsafe(export_name = "Meta_Attach")]
pub extern "C" fn meta_attach(
    now: metamod::PLUG_LOADTIME,
    function_table: *mut metamod::META_FUNCTIONS,
    meta_globals: *mut metamod::meta_globals_t,
    gamedll_funcs: *mut metamod::gamedll_funcs_t,
) -> ::std::os::raw::c_int {
    file_log("Meta_Attach");
    if function_table.is_null() || meta_globals.is_null() || gamedll_funcs.is_null() {
        alert("something went wrong");
        return 0;
    }

    unsafe {
        META_GLOBALS = meta_globals;
        *function_table = META_FUNCTION_TABLE;
        GAME_DLL_FUNCS = gamedll_funcs;
    }

    1
}

#[allow(unused_variables)]
#[unsafe(export_name = "Meta_Detach")]
pub extern "C" fn meta_detach(
    now: metamod::PLUG_LOADTIME,
    reason: metamod::PL_UNLOAD_REASON,
) -> ::std::os::raw::c_int {
    file_log("Meta_Detach");
    1
}

fn c_char_to_string(ptr: *const c_char) -> String {
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}

// fn send_to_engine(msg: &str) {
//     let cmsg = CString::new(msg)
//         .expect("string contains NUL");

//     unsafe {
//         engine_log(cmsg.as_ptr());
//     }
//     // cmsg żyje do końca funkcji → OK
// }

static FUNCTION_TABLE: metamod::DLL_FUNCTIONS = metamod::DLL_FUNCTIONS {
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
    pfnClientPutInServer: None,
    pfnClientCommand: None,
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

extern "C" fn get_entity_api2(
    function_table: *mut metamod::DLL_FUNCTIONS,
    interface_version: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    file_log("get_entity_api2");
    if function_table.is_null() {
        alert("something went wrong");
        return 0;
    }
    if unsafe { *interface_version } != metamod::INTERFACE_VERSION as i32 {
        alert("something went wrong");
        return 0;
    }

    unsafe {
        *function_table = FUNCTION_TABLE;
    }

    1
}

static META_ENG_FUNCS: metamod::enginefuncs_t = metamod::enginefuncs_t {
    pfnPrecacheModel: None,
    pfnPrecacheSound: None,
    pfnSetModel: None,
    pfnModelIndex: None,
    pfnModelFrames: None,
    pfnSetSize: None,
    pfnChangeLevel: None,
    pfnGetSpawnParms: None,
    pfnSaveSpawnParms: None,
    pfnVecToYaw: None,
    pfnVecToAngles: None,
    pfnMoveToOrigin: None,
    pfnChangeYaw: None,
    pfnChangePitch: None,
    pfnFindEntityByString: None,
    pfnGetEntityIllum: None,
    pfnFindEntityInSphere: None,
    pfnFindClientInPVS: None,
    pfnEntitiesInPVS: None,
    pfnMakeVectors: None,
    pfnAngleVectors: None,
    pfnCreateEntity: None,
    pfnRemoveEntity: None,
    pfnCreateNamedEntity: None,
    pfnMakeStatic: None,
    pfnEntIsOnFloor: None,
    pfnDropToFloor: None,
    pfnWalkMove: None,
    pfnSetOrigin: None,
    pfnEmitSound: None,
    pfnEmitAmbientSound: None,
    pfnTraceLine: None,
    pfnTraceToss: None,
    pfnTraceMonsterHull: None,
    pfnTraceHull: None,
    pfnTraceModel: None,
    pfnTraceTexture: None,
    pfnTraceSphere: None,
    pfnGetAimVector: None,
    pfnServerCommand: None,
    pfnServerExecute: None,
    pfnClientCommand: None,
    pfnParticleEffect: None,
    pfnLightStyle: None,
    pfnDecalIndex: None,
    pfnPointContents: None,
    pfnMessageBegin: None,
    pfnMessageEnd: None,
    pfnWriteByte: None,
    pfnWriteChar: None,
    pfnWriteShort: None,
    pfnWriteLong: None,
    pfnWriteAngle: None,
    pfnWriteCoord: None,
    pfnWriteString: None,
    pfnWriteEntity: None,
    pfnCVarRegister: None,
    pfnCVarGetFloat: None,
    pfnCVarGetString: None,
    pfnCVarSetFloat: None,
    pfnCVarSetString: None,
    pfnAlertMessage: None,
    pfnEngineFprintf: None,
    pfnPvAllocEntPrivateData: None,
    pfnPvEntPrivateData: None,
    pfnFreeEntPrivateData: None,
    pfnSzFromIndex: None,
    pfnAllocString: None,
    pfnGetVarsOfEnt: None,
    pfnPEntityOfEntOffset: None,
    pfnEntOffsetOfPEntity: None,
    pfnIndexOfEdict: None,
    pfnPEntityOfEntIndex: None,
    pfnFindEntityByVars: None,
    pfnGetModelPtr: None,
    pfnRegUserMsg: None,
    pfnAnimationAutomove: None,
    pfnGetBonePosition: None,
    pfnFunctionFromName: None,
    pfnNameForFunction: None,
    pfnClientPrintf: None,
    pfnServerPrint: None,
    pfnCmd_Args: None,
    pfnCmd_Argv: None,
    pfnCmd_Argc: None,
    pfnGetAttachment: None,
    pfnCRC32_Init: None,
    pfnCRC32_ProcessBuffer: None,
    pfnCRC32_ProcessByte: None,
    pfnCRC32_Final: None,
    pfnRandomLong: None,
    pfnRandomFloat: None,
    pfnSetView: None,
    pfnTime: None,
    pfnCrosshairAngle: None,
    pfnLoadFileForMe: None,
    pfnFreeFile: None,
    pfnEndSection: None,
    pfnCompareFileTime: None,
    pfnGetGameDir: None,
    pfnCvar_RegisterVariable: None,
    pfnFadeClientVolume: None,
    pfnSetClientMaxspeed: None,
    pfnCreateFakeClient: None,
    pfnRunPlayerMove: None,
    pfnNumberOfEntities: None,
    pfnGetInfoKeyBuffer: None,
    pfnInfoKeyValue: None,
    pfnSetKeyValue: None,
    pfnSetClientKeyValue: None,
    pfnIsMapValid: None,
    pfnStaticDecal: None,
    pfnPrecacheGeneric: None,
    pfnGetPlayerUserId: None,
    pfnBuildSoundMsg: None,
    pfnIsDedicatedServer: None,
    pfnCVarGetPointer: None,
    pfnGetPlayerWONId: None,
    pfnInfo_RemoveKey: None,
    pfnGetPhysicsKeyValue: None,
    pfnSetPhysicsKeyValue: None,
    pfnGetPhysicsInfoString: None,
    pfnPrecacheEvent: None,
    pfnPlaybackEvent: None,
    pfnSetFatPVS: None,
    pfnSetFatPAS: None,
    pfnCheckVisibility: None,
    pfnDeltaSetField: None,
    pfnDeltaUnsetField: None,
    pfnDeltaAddEncoder: None,
    pfnGetCurrentPlayer: None,
    pfnCanSkipPlayer: None,
    pfnDeltaFindField: None,
    pfnDeltaSetFieldByIndex: None,
    pfnDeltaUnsetFieldByIndex: None,
    pfnSetGroupMask: None,
    pfnCreateInstancedBaseline: None,
    pfnCvar_DirectSet: None,
    pfnForceUnmodified: None,
    pfnGetPlayerStats: None,
    pfnAddServerCommand: None,
    pfnVoice_GetClientListening: None,
    pfnVoice_SetClientListening: None,
    pfnGetPlayerAuthId: None,
    pfnSequenceGet: None,
    pfnSequencePickSentence: None,
    pfnGetFileSize: None,
    pfnGetApproxWavePlayLen: None,
    pfnIsCareerMatch: None,
    pfnGetLocalizedStringLength: None,
    pfnRegisterTutorMessageShown: None,
    pfnGetTimesTutorMessageShown: None,
    pfnProcessTutorMessageDecayBuffer: None,
    pfnConstructTutorMessageDecayBuffer: None,
    pfnResetTutorMessageDecayData: None,
    pfnQueryClientCvarValue: None,
    pfnQueryClientCvarValue2: None,
    pfnCheckParm: None,
};

extern "C" fn get_engine_functions(
    functions_from_engine: *mut metamod::enginefuncs_t,
    interface_version: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    file_log("get_engine_functions");
    if functions_from_engine.is_null() {
        alert("something went wrong");
        return 0;
    }
    if unsafe { *interface_version } != metamod::ENGINE_INTERFACE_VERSION as i32 {
        alert("something went wrong");
        return 0;
    }

    unsafe {
        *functions_from_engine = META_ENG_FUNCS;
    }

    1
}
