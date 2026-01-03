use super::abi;
use cstr::cstr;
use std::{
    ffi::{CStr, c_char},
    sync::OnceLock,
};

static PRINT_FORMAT: &CStr = cstr!("%s");

static PLUGIN_IFVERS: &CStr = cstr!("5:13");
static PLUGIN_NAME: &CStr = cstr!("rust print");
static PLUGIN_VERSION: &CStr = cstr!("1.0.0");
static PLUGIN_DATE: &CStr = cstr!("26.12.2025");
static PLUGIN_AUTHOR: &CStr = cstr!("AwIlL");
static PLUGIN_URL: &CStr = cstr!("-");
static PLUGIN_LOGTAG: &CStr = cstr!("RUST_PRINT");

pub static mut PLUGIN_INFO: abi::plugin_info_t = abi::plugin_info_t {
    ifvers: PLUGIN_IFVERS.as_ptr(),
    name: PLUGIN_NAME.as_ptr(),
    version: PLUGIN_VERSION.as_ptr(),
    date: PLUGIN_DATE.as_ptr(),
    author: PLUGIN_AUTHOR.as_ptr(),
    url: PLUGIN_URL.as_ptr(),
    logtag: PLUGIN_LOGTAG.as_ptr(),
    loadable: abi::PLUG_LOADTIME_PT_ANYTIME,
    unloadable: abi::PLUG_LOADTIME_PT_ANYPAUSE,
};

pub fn c_char_to_string(ptr: *const c_char) -> String {
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
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

pub fn get_user_msg_id(
    msg_name: &CStr,
    size: *mut ::std::os::raw::c_int,
) -> Option<::std::os::raw::c_int> {
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

pub fn message_begin(
    msg_dest: ::std::os::raw::c_int,
    msg_type: ::std::os::raw::c_int,
    origin: *const f32,
    ed: *mut abi::edict_t,
) {
    if let Some(api) = ENG_FUNCS.get() {
        if let Some(function) = api.pfnMessageBegin {
            unsafe { function(msg_dest, msg_type, origin, ed) }
        }
    }
}

pub fn message_end() {
    if let Some(api) = ENG_FUNCS.get() {
        if let Some(function) = api.pfnMessageEnd {
            unsafe { function() }
        }
    }
}

pub fn write_byte(value: ::std::os::raw::c_int) {
    if let Some(api) = ENG_FUNCS.get() {
        if let Some(function) = api.pfnWriteByte {
            unsafe { function(value) }
        }
    }
}

pub fn write_char(value: ::std::os::raw::c_int) {
    if let Some(api) = ENG_FUNCS.get() {
        if let Some(function) = api.pfnWriteChar {
            unsafe { function(value) }
        }
    }
}

pub fn write_short(value: ::std::os::raw::c_int) {
    if let Some(api) = ENG_FUNCS.get() {
        if let Some(function) = api.pfnWriteShort {
            unsafe { function(value) }
        }
    }
}

pub fn write_long(value: ::std::os::raw::c_int) {
    if let Some(api) = ENG_FUNCS.get() {
        if let Some(function) = api.pfnWriteLong {
            unsafe { function(value) }
        }
    }
}

pub fn write_angle(value: f32) {
    if let Some(api) = ENG_FUNCS.get() {
        if let Some(function) = api.pfnWriteAngle {
            unsafe { function(value) }
        }
    }
}

pub fn write_coord(value: f32) {
    if let Some(api) = ENG_FUNCS.get() {
        if let Some(function) = api.pfnWriteCoord {
            unsafe { function(value) }
        }
    }
}

pub fn write_string(value: &CStr) {
    if let Some(api) = ENG_FUNCS.get() {
        if let Some(function) = api.pfnWriteString {
            unsafe { function(value.as_ptr()) }
        }
    }
}

pub fn write_entity(value: ::std::os::raw::c_int) {
    if let Some(api) = ENG_FUNCS.get() {
        if let Some(function) = api.pfnWriteEntity {
            unsafe { function(value) }
        }
    }
}
