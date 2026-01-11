use super::abi;
use std::{
    ffi::{CStr, c_char},
    sync::OnceLock,
};

static PRINT_FORMAT: &CStr = c"%s";

static PLUGIN_IFVERS: &CStr = c"5:13";
static PLUGIN_NAME: &CStr = c"rust print";
static PLUGIN_VERSION: &CStr = c"1.0.0";
static PLUGIN_DATE: &CStr = c"26.12.2025";
static PLUGIN_AUTHOR: &CStr = c"AwIlL";
static PLUGIN_URL: &CStr = c"-";
static PLUGIN_LOGTAG: &CStr = c"RUST_PRINT";

pub static mut PLUGIN_INFO: abi::plugin_info_t = abi::plugin_info_t {
    ifvers: PLUGIN_IFVERS.as_ptr(),
    name: PLUGIN_NAME.as_ptr(),
    version: PLUGIN_VERSION.as_ptr(),
    date: PLUGIN_DATE.as_ptr(),
    author: PLUGIN_AUTHOR.as_ptr(),
    url: PLUGIN_URL.as_ptr(),
    logtag: PLUGIN_LOGTAG.as_ptr(),
    loadable: abi::PLUG_LOADTIME_PT_ANYTIME, // only for development, in future should be changed to PLUG_LOADTIME_PT_STARTUP
    unloadable: abi::PLUG_LOADTIME_PT_ANYTIME,
};

pub fn c_char_to_string(ptr: *const c_char) -> String {
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}

pub fn origin_from_ptr(origin: *const f32) -> Option<[f32; 3]> {
    if origin.is_null() {
        None
    } else {
        // SAFETY:
        // - pointer isn't null
        // - engine contract says it is exactly 3 f32
        unsafe {
            let slice = std::slice::from_raw_parts(origin, 3);
            Some([slice[0], slice[1], slice[2]])
        }
    }
}

pub static mut GAME_DLL_FUNCS: *const abi::gamedll_funcs_t = std::ptr::null();
pub static mut GLOBALS: *const abi::globalvars_t = std::ptr::null();

// meta globals

pub static mut META_GLOBALS: *mut abi::meta_globals_t = std::ptr::null_mut();

pub fn result_orig_ret<T: Copy>() -> T {
    unsafe { *((*META_GLOBALS).orig_ret as *mut T) }
}

pub fn set_result(result: i32) {
    unsafe {
        (*META_GLOBALS).mres = result as abi::META_RES;
    }
}

// meta funcs

pub static META_UTIL_FUNCS: OnceLock<abi::mutil_funcs_t> = OnceLock::new();

pub fn err_log(msg: &CStr) {
    if let Some(api) = META_UTIL_FUNCS.get() {
        if let Some(function) = api.pfnLogError {
            unsafe {
                function(&raw mut PLUGIN_INFO, PRINT_FORMAT.as_ptr(), msg.as_ptr());
            }
        }
    }
}

pub fn console_log(msg: &CStr) {
    if let Some(api) = META_UTIL_FUNCS.get() {
        if let Some(function) = api.pfnLogConsole {
            unsafe {
                function(&raw mut PLUGIN_INFO, PRINT_FORMAT.as_ptr(), msg.as_ptr());
            }
        }
    }
}

pub fn get_user_msg_id(msg_name: &CStr, size: *mut i32) -> Option<i32> {
    if let Some(api) = META_UTIL_FUNCS.get() {
        if let Some(function) = api.pfnGetUserMsgID {
            let result = unsafe { function(&raw mut PLUGIN_INFO, msg_name.as_ptr(), size) };
            if result == 0 {
                return None;
            }
            return Some(result);
        }
    }
    None
}

// engine funcs

pub static ENG_FUNCS: OnceLock<abi::enginefuncs_t> = OnceLock::new();

pub fn get_ent_index(entity: *mut abi::edict_t) -> Option<i32> {
    ENG_FUNCS
        .get()
        .map(|api| api.pfnIndexOfEdict)
        .flatten()
        .map(|f| unsafe { f(entity) })
}

pub fn get_ent_by_index(index: i32) -> Option<*mut abi::edict_t> {
    ENG_FUNCS
        .get()
        .map(|api| api.pfnPEntityOfEntIndex)
        .flatten()
        .map(|f| unsafe { f(index) })
}

pub fn alert(msg: &CStr) {
    if let Some(api) = ENG_FUNCS.get() {
        if let Some(function) = api.pfnAlertMessage {
            unsafe {
                function(
                    abi::ALERT_TYPE_at_console,
                    PRINT_FORMAT.as_ptr(),
                    msg.as_ptr(),
                )
            }
        }
    }
}
