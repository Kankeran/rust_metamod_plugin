use super::abi;
use super::engine;
use super::entity_api2;
use super::meta;
use crate::adapter::bootstrap;
use crate::util::log;

#[unsafe(export_name = "GiveFnptrsToDll")]
pub extern "system" fn give_dll_pointers(
    funcs_from_engine: *mut abi::enginefuncs_t,
    globals_pointer: *mut abi::globalvars_t,
) {
    log::debug("GiveFnptrsToDll");

    if let Err(_) = meta::ENG_FUNCS.set(unsafe { *funcs_from_engine }) {
        log::debug("engine functions already setup");
    }

    unsafe {
        meta::GLOBALS = globals_pointer;
    }
}

#[unsafe(export_name = "Meta_Query")]
pub extern "C" fn meta_query(
    interface_version: *const ::std::os::raw::c_char,
    plugin_info: *mut *mut abi::plugin_info_t,
    meta_util_funcs: *mut abi::mutil_funcs_t,
) -> ::std::os::raw::c_int {
    log::debug("Meta_Query");
    unsafe {
        *plugin_info = &raw mut meta::PLUGIN_INFO;
    }

    if let Err(_) = meta::META_UTIL_FUNCS.set(unsafe { *meta_util_funcs }) {
        log::debug("meta functions already setup");
    }

    let plugin_required_version = meta::c_char_to_string(unsafe { meta::PLUGIN_INFO.ifvers });
    let engine_ifvers = meta::c_char_to_string(interface_version);

    if engine_ifvers != plugin_required_version {
        log::error(&format!("meta interface version mismatch, expected {}, got {}", &plugin_required_version, &engine_ifvers));
        return 0;
    }

    1
}

static META_FUNCTION_TABLE: abi::META_FUNCTIONS = abi::META_FUNCTIONS {
    pfnGetEntityAPI: None,
    pfnGetEntityAPI_Post: None,
    pfnGetEntityAPI2: Some(entity_api2::get_api),
    pfnGetEntityAPI2_Post: Some(entity_api2::get_api_post),
    pfnGetNewDLLFunctions: None,
    pfnGetNewDLLFunctions_Post: None,
    pfnGetEngineFunctions: Some(engine::get_functions),
    pfnGetEngineFunctions_Post: Some(engine::get_functions_post),
};

#[unsafe(export_name = "Meta_Attach")]
pub extern "C" fn meta_attach(
    now: abi::PLUG_LOADTIME,
    function_table: *mut abi::META_FUNCTIONS,
    meta_globals: *mut abi::meta_globals_t,
    gamedll_funcs: *mut abi::gamedll_funcs_t,
) -> ::std::os::raw::c_int {
    log::debug("Meta_Attach");
    if function_table.is_null() || meta_globals.is_null() || gamedll_funcs.is_null() {
        log::error("meta_attach has null pointers");
        return 0;
    }

    unsafe {
        meta::META_GLOBALS = meta_globals;
        *function_table = META_FUNCTION_TABLE;
        meta::GAME_DLL_FUNCS = gamedll_funcs;
    }

    bootstrap::load();

    1
}

#[allow(unused_variables)]
#[unsafe(export_name = "Meta_Detach")]
pub extern "C" fn meta_detach(
    now: abi::PLUG_LOADTIME,
    reason: abi::PL_UNLOAD_REASON,
) -> ::std::os::raw::c_int {
    log::debug("Meta_Detach");
    1
}
