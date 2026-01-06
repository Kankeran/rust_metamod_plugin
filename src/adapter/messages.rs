use crate::{
    adapter::{api, convert},
    metamod::{meta_api, meta_const},
};

pub struct TextMessage {
    id: Option<i32>,
    mode: api::PrintMode,
    msg: String,
}

impl TextMessage {
    pub fn new(id: Option<i32>, mode: api::PrintMode, msg: String) -> Self {
        TextMessage { id, mode, msg }
    }

    pub fn send(&self) {
        if let Some(msg_id) = meta_api::get_text_msg_id() {
            let point = self.msg.floor_char_boundary(188);
            let (msg, _) = self.msg.split_at(point);
            let entity = meta_api::get_ent_by_index_option(self.id);
            let msg_dest = if let None = entity {
                meta_const::MSG_BROADCAST
            } else {
                meta_const::MSG_ONE
            };
            meta_api::message_begin(msg_dest, msg_id, None, entity.as_ref());
            meta_api::write_byte(convert::print_mode(&self.mode));
            meta_api::write_string(msg);
            meta_api::message_end();
        }
    }
}

pub struct ShowMenuMessage {
    id: i32,
    keys: i32,
    time: i32, // seconds, -1 = infinity
    buf: String,
}

impl ShowMenuMessage {
    pub fn new(id: i32, keys: i32, time: i32, buf: String) -> Self {
        ShowMenuMessage {
            id,
            keys,
            time,
            buf,
        }
    }

    pub fn send(&self) {
        if let Some(msg_id) = meta_api::get_show_menu_id() {
            let entity = meta_api::get_ent_by_index(self.id);
            if let None = entity {
                return;
            }
            let mut msg;
            let mut next_msg = self.buf.as_str();
            loop {
                let point = next_msg.floor_char_boundary(175);
                (msg, next_msg) = next_msg.split_at(point);
                let has_more = if next_msg.len() > 0 { 1 } else { 0 };
                meta_api::message_begin(meta_const::MSG_ONE, msg_id, None, entity.as_ref());
                meta_api::write_short(self.keys);
                meta_api::write_char(self.time);
                meta_api::write_byte(has_more);
                meta_api::write_string(msg);
                meta_api::message_end();
                if next_msg.len() == 0 {
                    break;
                }
            }
        }
    }
}
