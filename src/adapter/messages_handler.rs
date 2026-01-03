use crate::{
    adapter::{api, convert},
    metamod::{abi, meta, meta_const},
};

struct RawMessage {}

pub fn message_begin(
    msg_dest: i32,
    msg_type: i32,
    origin: *const f32,
    ed: *mut abi::edict_t,
) -> i32 {
    api::console_debug("message_begin");
    api::console_debug(&format!("msg_type {:?}", convert::user_msg(msg_type)));
    if !ed.is_null() {
        api::console_debug(&format!("ent id {}", meta::get_ent_index(ed).unwrap()));
    }

    meta_const::RESULT_IGNORED
}

pub fn write_byte(value: i32) -> i32 {
    api::console_debug("write_byte");

    meta_const::RESULT_IGNORED
}

pub fn write_char(value: i32) -> i32 {
    api::console_debug("write_char");

    meta_const::RESULT_IGNORED
}

pub fn write_short(value: i32) -> i32 {
    api::console_debug("write_short");

    meta_const::RESULT_IGNORED
}

pub fn write_long(value: i32) -> i32 {
    api::console_debug("write_long");

    meta_const::RESULT_IGNORED
}

pub fn write_angle(value: f32) -> i32 {
    api::console_debug("write_angle");

    meta_const::RESULT_IGNORED
}

pub fn write_coord(value: f32) -> i32 {
    api::console_debug("write_coord");

    meta_const::RESULT_IGNORED
}

pub fn write_string(value: String) -> i32 {
    api::console_debug("write_string");

    meta_const::RESULT_IGNORED
}

pub fn write_entity(value: i32) -> i32 {
    api::console_debug("write_entity");

    meta_const::RESULT_IGNORED
}

pub fn message_end() -> i32 {
    api::console_debug("message_end");

    meta_const::RESULT_IGNORED
}

pub fn message_begin_post(
    msg_dest: i32,
    msg_type: i32,
    origin: *const f32,
    ed: *mut abi::edict_t,
) -> i32 {
    api::console_debug("message_begin_post");

    meta_const::RESULT_IGNORED
}

pub fn write_byte_post(value: i32) -> i32 {
    api::console_debug("write_byte_post");

    meta_const::RESULT_IGNORED
}

pub fn write_char_post(value: i32) -> i32 {
    api::console_debug("write_char_post");

    meta_const::RESULT_IGNORED
}

pub fn write_short_post(value: i32) -> i32 {
    api::console_debug("write_short_post");

    meta_const::RESULT_IGNORED
}

pub fn write_long_post(value: i32) -> i32 {
    api::console_debug("write_long_post");

    meta_const::RESULT_IGNORED
}

pub fn write_angle_post(value: f32) -> i32 {
    api::console_debug("write_angle_post");

    meta_const::RESULT_IGNORED
}

pub fn write_coord_post(value: f32) -> i32 {
    api::console_debug("write_coord_post");

    meta_const::RESULT_IGNORED
}

pub fn write_string_post(value: String) -> i32 {
    api::console_debug("write_string_post");

    meta_const::RESULT_IGNORED
}

pub fn write_entity_post(value: i32) -> i32 {
    api::console_debug("write_entity_post");

    meta_const::RESULT_IGNORED
}

pub fn message_end_post() -> i32 {
    api::console_debug("message_end_post");

    meta_const::RESULT_IGNORED
}
