use std::{ffi::CString, ptr};

use crate::{
    adapter::{api, convert},
    metamod::{meta, meta_api, meta_const},
};

pub struct TextMessage {
    id: Option<i32>,
    mode: api::PrintMode,
    msg: String,
}

impl TextMessage {
    pub fn new(id: Option<i32>, mode: api::PrintMode, msg: String) -> TextMessage {
        TextMessage { id, mode, msg }
    }

    pub fn send(&self) {
        if let Some(msg_id) = meta_api::get_text_msg_id() {
            let point = self.msg.floor_char_boundary(188);
            let (msg, _) = self.msg.split_at(point);
            if let Ok(cmsg) = CString::new(msg) {
                if let Some(id) = self.id {
                    let entity = meta::get_ent_by_index(id).unwrap();
                    meta::message_begin(meta_const::MSG_ONE as i32, msg_id, ptr::null(), entity);
                } else {
                    meta::message_begin(
                        meta_const::MSG_BROADCAST as i32,
                        msg_id,
                        ptr::null(),
                        ptr::null_mut(),
                    );
                }
                meta::write_byte(convert::print_mode(&self.mode));
                meta::write_string(cmsg.as_c_str());
                meta::message_end();
            }
        }
    }
}
