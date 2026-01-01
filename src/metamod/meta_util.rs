use std::ffi::CStr;
use std::os::raw::c_int;
use std::ptr;

use super::abi;
use super::meta;
use super::msgs;

pub const PRINT_NOTIFY: i32 = 1;
pub const PRINT_CONSOLE: i32 = 2;
pub const PRINT_CHAT: i32 = 3;
pub const PRINT_CENTER: i32 = 4;

pub const RESULT_IGNORED: i32 = 1;
pub const RESULT_HANDLED: i32 = 2;
pub const RESULT_OVERRIDE: i32 = 3;
pub const RESULT_SUPERCEDE: i32 = 4;

pub fn client_say_text(entity: *mut abi::edict_t, msg_dest: c_int, msg: &CStr) {
    if let Some(msg_id) = unsafe { msgs::TEXT_MSG } {
        if !entity.is_null() {
            meta::message_begin(abi::MSG_ONE as i32, msg_id, ptr::null(), entity);
        } else {
            meta::message_begin(
                abi::MSG_BROADCAST as i32,
                msg_id,
                ptr::null(),
                ptr::null_mut(),
            );
        }
        meta::write_byte(msg_dest);
        // meta::write_string(cstr::cstr!("%s"));
        meta::write_string(msg);
        meta::message_end();
    }
}
