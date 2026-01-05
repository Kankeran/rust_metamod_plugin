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
    pub fn new(id: Option<i32>, mode: api::PrintMode, msg: String) -> TextMessage {
        TextMessage { id, mode, msg }
    }

    pub fn send(&self) {
        if let Some(msg_id) = meta_api::get_text_msg_id() {
            let point = self.msg.floor_char_boundary(188);
            let (msg, _) = self.msg.split_at(point);
            let entity = meta_api::get_ent_by_index(self.id);
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
