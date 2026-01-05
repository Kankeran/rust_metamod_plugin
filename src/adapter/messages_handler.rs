use std::sync::Mutex;

use crate::{
    adapter::{
        api::{self, BlockMode},
        convert,
    },
    metamod::{meta_api, meta_const},
    util::log,
};

static MSG: Mutex<Option<RawMessage>> = Mutex::new(None);
static mut MSG_BLOCKS: [BlockMode; 256] = [BlockMode::BlockNone; 256];
// TODO: change login when registration messages will be possible, bool for now
static mut MSG_HOOKS: [bool; 256] = [false; 256];

static mut MSG_TYPE_CURRENT: i32 = 0;
static mut HOOK_CURRENT: bool = false;
static mut BLOCK_CURRENT: bool = false;

pub fn handle_msg(msg_type: api::UserMsgs) {
    if let Some(msg_type) = convert::user_msg_id(&msg_type) {

        unsafe {MSG_HOOKS[msg_type as usize] = true}
    }
}

#[derive(Debug)]
pub struct RawMessage {
    msg_dest: i32,
    msg_type: i32,
    origin: Option<[f32; 3]>,
    ent: Option<meta_api::EdictPtr>,
    data: Vec<MessageValue>,
}

#[derive(Debug)]
enum MessageValue {
    Byte(i32),
    Char(i32),
    Short(i32),
    Long(i32),
    Angle(f32),
    Coord(f32),
    String(String),
    Entity(i32),
}

impl RawMessage {
    fn new(
        msg_dest: i32,
        msg_type: i32,
        origin: Option<[f32; 3]>,
        ent: Option<meta_api::EdictPtr>,
    ) -> Self {
        Self {
            msg_dest,
            msg_type,
            origin,
            ent,
            data: Vec::new(),
        }
    }

    fn add_data(&mut self, value: MessageValue) {
        self.data.push(value);
    }

    fn handle_message(self) {
        // parse to specific message and handle callbacks here :)
        log::debug(&format!("msg: {:?}", self));
        self.send();
    }

    fn send(&self) {
        meta_api::message_begin(self.msg_dest, self.msg_type, self.origin, self.ent.as_ref());

        for value in self.data.iter() {
            match value {
                MessageValue::Byte(value) => meta_api::write_byte(*value),
                MessageValue::Char(value) => meta_api::write_char(*value),
                MessageValue::Short(value) => meta_api::write_short(*value),
                MessageValue::Long(value) => meta_api::write_long(*value),
                MessageValue::Angle(value) => meta_api::write_angle(*value),
                MessageValue::Coord(value) => meta_api::write_coord(*value),
                MessageValue::String(value) => meta_api::write_string(value),
                MessageValue::Entity(value) => meta_api::write_entity(*value),
            };
        }

        meta_api::message_end();
    }
}

pub fn message_begin(
    msg_dest: i32,
    msg_type: i32,
    origin: Option<[f32; 3]>,
    ent: Option<meta_api::EdictPtr>,
) -> i32 {
    let id = if let Some(id) = meta_api::get_ent_index(ent.as_ref()) {
        id
    } else {
        0
    };
    api::console_debug(&format!(
        "message_begin | msg_type {:?} ({}) | ent {:?}",
        convert::user_msg(msg_type),
        msg_type,
        id
    ));
    if let BlockMode::BlockAll | BlockMode::BlockOne = unsafe { MSG_BLOCKS[msg_type as usize] } {
        log::debug("block message");
        unsafe {
            BLOCK_CURRENT = true;
            MSG_TYPE_CURRENT = msg_type;
        }

        meta_const::RESULT_SUPERCEDE
    } else if unsafe { MSG_HOOKS[msg_type as usize] } {
        log::debug("hook message");
        unsafe {
            HOOK_CURRENT = true;
            MSG_TYPE_CURRENT = msg_type;
        }
        let mut msg = MSG.lock().unwrap();
        *msg = Some(RawMessage::new(msg_dest, msg_type, origin, ent));

        meta_const::RESULT_SUPERCEDE
    } else {
        meta_const::RESULT_IGNORED
    }
}

pub fn write_byte(value: i32) -> i32 {
    api::console_debug(&format!("write_byte | value {:?}", value));

    if unsafe { BLOCK_CURRENT } {
        meta_const::RESULT_SUPERCEDE
    } else if unsafe { HOOK_CURRENT } {
        let mut msg = MSG.lock().unwrap();
        msg.as_mut().unwrap().add_data(MessageValue::Byte(value));
        meta_const::RESULT_SUPERCEDE
    } else {
        meta_const::RESULT_IGNORED
    }
}

pub fn write_char(value: i32) -> i32 {
    api::console_debug(&format!("write_char | value {:?}", value));

    if unsafe { BLOCK_CURRENT } {
        meta_const::RESULT_SUPERCEDE
    } else if unsafe { HOOK_CURRENT } {
        let mut msg = MSG.lock().unwrap();
        msg.as_mut().unwrap().add_data(MessageValue::Char(value));
        meta_const::RESULT_SUPERCEDE
    } else {
        meta_const::RESULT_IGNORED
    }
}

pub fn write_short(value: i32) -> i32 {
    api::console_debug(&format!("write_short | value {:?}", value));

    if unsafe { BLOCK_CURRENT } {
        meta_const::RESULT_SUPERCEDE
    } else if unsafe { HOOK_CURRENT } {
        let mut msg = MSG.lock().unwrap();
        msg.as_mut().unwrap().add_data(MessageValue::Short(value));
        meta_const::RESULT_SUPERCEDE
    } else {
        meta_const::RESULT_IGNORED
    }
}

pub fn write_long(value: i32) -> i32 {
    api::console_debug(&format!("write_long | value {:?}", value));

    if unsafe { BLOCK_CURRENT } {
        meta_const::RESULT_SUPERCEDE
    } else if unsafe { HOOK_CURRENT } {
        let mut msg = MSG.lock().unwrap();
        msg.as_mut().unwrap().add_data(MessageValue::Long(value));
        meta_const::RESULT_SUPERCEDE
    } else {
        meta_const::RESULT_IGNORED
    }
}

pub fn write_angle(value: f32) -> i32 {
    api::console_debug(&format!("write_angle | value {:?}", value));

    if unsafe { BLOCK_CURRENT } {
        meta_const::RESULT_SUPERCEDE
    } else if unsafe { HOOK_CURRENT } {
        let mut msg = MSG.lock().unwrap();
        msg.as_mut().unwrap().add_data(MessageValue::Angle(value));
        meta_const::RESULT_SUPERCEDE
    } else {
        meta_const::RESULT_IGNORED
    }
}

pub fn write_coord(value: f32) -> i32 {
    api::console_debug(&format!("write_coord | value {:?}", value));

    if unsafe { BLOCK_CURRENT } {
        meta_const::RESULT_SUPERCEDE
    } else if unsafe { HOOK_CURRENT } {
        let mut msg = MSG.lock().unwrap();
        msg.as_mut().unwrap().add_data(MessageValue::Coord(value));
        meta_const::RESULT_SUPERCEDE
    } else {
        meta_const::RESULT_IGNORED
    }
}

pub fn write_string(value: String) -> i32 {
    api::console_debug(&format!("write_string | value {:?}", value));

    if unsafe { BLOCK_CURRENT } {
        meta_const::RESULT_SUPERCEDE
    } else if unsafe { HOOK_CURRENT } {
        let mut msg = MSG.lock().unwrap();
        msg.as_mut().unwrap().add_data(MessageValue::String(value));
        meta_const::RESULT_SUPERCEDE
    } else {
        meta_const::RESULT_IGNORED
    }
}

pub fn write_entity(value: i32) -> i32 {
    api::console_debug(&format!("write_entity | value {:?}", value));

    if unsafe { BLOCK_CURRENT } {
        meta_const::RESULT_SUPERCEDE
    } else if unsafe { HOOK_CURRENT } {
        let mut msg = MSG.lock().unwrap();
        msg.as_mut().unwrap().add_data(MessageValue::Entity(value));
        meta_const::RESULT_SUPERCEDE
    } else {
        meta_const::RESULT_IGNORED
    }
}

pub fn message_end() -> i32 {
    api::console_debug("message_end");

    if unsafe { BLOCK_CURRENT } {
        unsafe {
            BLOCK_CURRENT = false;
        }
        if let BlockMode::BlockOne = unsafe { MSG_BLOCKS[MSG_TYPE_CURRENT as usize] } {
            unsafe {
                MSG_BLOCKS[MSG_TYPE_CURRENT as usize] = BlockMode::BlockNone;
            }
        }
        meta_const::RESULT_SUPERCEDE
    } else if unsafe { HOOK_CURRENT } {
        unsafe {
            HOOK_CURRENT = false;
        }

        MSG.lock().unwrap().take().unwrap().handle_message();

        meta_const::RESULT_SUPERCEDE
    } else {
        meta_const::RESULT_IGNORED
    }
}

pub fn message_begin_post(
    msg_dest: i32,
    msg_type: i32,
    origin: Option<[f32; 3]>,
    ent: Option<meta_api::EdictPtr>,
) -> i32 {
    // api::console_debug("message_begin_post");

    meta_const::RESULT_IGNORED
}

pub fn write_byte_post(value: i32) -> i32 {
    // api::console_debug("write_byte_post");

    meta_const::RESULT_IGNORED
}

pub fn write_char_post(value: i32) -> i32 {
    // api::console_debug("write_char_post");

    meta_const::RESULT_IGNORED
}

pub fn write_short_post(value: i32) -> i32 {
    // api::console_debug("write_short_post");

    meta_const::RESULT_IGNORED
}

pub fn write_long_post(value: i32) -> i32 {
    // api::console_debug("write_long_post");

    meta_const::RESULT_IGNORED
}

pub fn write_angle_post(value: f32) -> i32 {
    // api::console_debug("write_angle_post");

    meta_const::RESULT_IGNORED
}

pub fn write_coord_post(value: f32) -> i32 {
    // api::console_debug("write_coord_post");

    meta_const::RESULT_IGNORED
}

pub fn write_string_post(value: String) -> i32 {
    // api::console_debug("write_string_post");

    meta_const::RESULT_IGNORED
}

pub fn write_entity_post(value: i32) -> i32 {
    // api::console_debug("write_entity_post");

    meta_const::RESULT_IGNORED
}

pub fn message_end_post() -> i32 {
    // api::console_debug("message_end_post");

    meta_const::RESULT_IGNORED
}
