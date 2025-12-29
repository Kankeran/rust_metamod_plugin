use std::ffi::CStr;
use std::os::raw::c_int;
use std::ptr;

use crate::util::log;

use super::abi;
use super::meta;
use super::msgs;

pub fn client_say_text(entity: *mut abi::edict_t, msg_dest: c_int, msg: &CStr) {
    if let Some(msg_id) = unsafe { msgs::TEXT_MSG } {
        if !entity.is_null() {
            meta::message_begin(abi::MSG_ONE as i32, msg_id, ptr::null(), entity);
        } else {
            meta::message_begin(abi::MSG_BROADCAST as i32, msg_id, ptr::null(), ptr::null_mut());
        }
        meta::write_byte(msg_dest);
        // meta::write_string(cstr::cstr!("%s"));
        meta::write_string(msg);
        meta::message_end();

    }
}
