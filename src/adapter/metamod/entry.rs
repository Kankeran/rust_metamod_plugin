use std::sync::OnceLock;

use super::{abi, meta, meta_api, meta_const};

pub struct EntryFuncs {
    init: fn(),
    setup: fn(),
    client_command: fn(i32, Vec<String>) -> i32,
    message_begin: fn(i32, i32, Option<[f32; 3]>, Option<meta_api::EdictPtr>) -> i32,
    write_byte: fn(i32) -> i32,
    write_char: fn(i32) -> i32,
    write_short: fn(i32) -> i32,
    write_long: fn(i32) -> i32,
    write_angle: fn(f32) -> i32,
    write_coord: fn(f32) -> i32,
    write_string: fn(String) -> i32,
    write_entity: fn(i32) -> i32,
    message_end: fn() -> i32,
    message_begin_post: fn(i32, i32, Option<[f32; 3]>, Option<meta_api::EdictPtr>) -> i32,
    write_byte_post: fn(i32) -> i32,
    write_char_post: fn(i32) -> i32,
    write_short_post: fn(i32) -> i32,
    write_long_post: fn(i32) -> i32,
    write_angle_post: fn(f32) -> i32,
    write_coord_post: fn(f32) -> i32,
    write_string_post: fn(String) -> i32,
    write_entity_post: fn(i32) -> i32,
    message_end_post: fn() -> i32,
}

impl EntryFuncs {
    pub fn new(
        init: fn(),
        setup: fn(),
        client_command: fn(i32, Vec<String>) -> i32,
        message_begin: fn(i32, i32, Option<[f32; 3]>, Option<meta_api::EdictPtr>) -> i32,
        write_byte: fn(i32) -> i32,
        write_char: fn(i32) -> i32,
        write_short: fn(i32) -> i32,
        write_long: fn(i32) -> i32,
        write_angle: fn(f32) -> i32,
        write_coord: fn(f32) -> i32,
        write_string: fn(String) -> i32,
        write_entity: fn(i32) -> i32,
        message_end: fn() -> i32,
        message_begin_post: fn(i32, i32, Option<[f32; 3]>, Option<meta_api::EdictPtr>) -> i32,
        write_byte_post: fn(i32) -> i32,
        write_char_post: fn(i32) -> i32,
        write_short_post: fn(i32) -> i32,
        write_long_post: fn(i32) -> i32,
        write_angle_post: fn(f32) -> i32,
        write_coord_post: fn(f32) -> i32,
        write_string_post: fn(String) -> i32,
        write_entity_post: fn(i32) -> i32,
        message_end_post: fn() -> i32,
    ) -> EntryFuncs {
        EntryFuncs {
            init,
            setup,
            client_command,
            message_begin,
            write_byte,
            write_char,
            write_short,
            write_long,
            write_angle,
            write_coord,
            write_string,
            write_entity,
            message_end,
            message_begin_post,
            write_byte_post,
            write_char_post,
            write_short_post,
            write_long_post,
            write_angle_post,
            write_coord_post,
            write_string_post,
            write_entity_post,
            message_end_post,
        }
    }
}

pub static ENTRY_FUNCS: OnceLock<EntryFuncs> = OnceLock::new();

pub fn meta_init() {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        (entry_funcs.init)()
    }
}

pub fn meta_setup() {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        (entry_funcs.setup)()
    }
}

pub fn client_command(id: i32, args: Vec<String>) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.client_command)(id, args);
    }

    meta_const::RESULT_IGNORED
}

pub fn message_begin(
    msg_dest: i32,
    msg_type: i32,
    origin: *const f32,
    ent: *mut abi::edict_t,
) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        let origin = meta::origin_from_ptr(origin);
        let ent = if !ent.is_null() {
            Some(meta_api::EdictPtr::new(ent))
        } else {
            None
        };
        return (entry_funcs.message_begin)(msg_dest, msg_type, origin, ent);
    }

    meta_const::RESULT_IGNORED
}

pub fn write_byte(value: i32) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.write_byte)(value);
    }

    meta_const::RESULT_IGNORED
}

pub fn write_char(value: i32) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.write_char)(value);
    }

    meta_const::RESULT_IGNORED
}

pub fn write_short(value: i32) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.write_short)(value);
    }

    meta_const::RESULT_IGNORED
}

pub fn write_long(value: i32) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.write_long)(value);
    }

    meta_const::RESULT_IGNORED
}

pub fn write_angle(value: f32) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.write_angle)(value);
    }

    meta_const::RESULT_IGNORED
}

pub fn write_coord(value: f32) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.write_coord)(value);
    }

    meta_const::RESULT_IGNORED
}

pub fn write_string(value: *const ::std::os::raw::c_char) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        let value = meta::c_char_to_string(value);
        return (entry_funcs.write_string)(value);
    }

    meta_const::RESULT_IGNORED
}

pub fn write_entity(value: i32) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.write_entity)(value);
    }

    meta_const::RESULT_IGNORED
}

pub fn message_end() -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.message_end)();
    }

    meta_const::RESULT_IGNORED
}

pub fn message_begin_post(
    msg_dest: i32,
    msg_type: i32,
    origin: *const f32,
    ent: *mut abi::edict_t,
) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        let origin = meta::origin_from_ptr(origin);
        let ent = if !ent.is_null() {
            Some(meta_api::EdictPtr::new(ent))
        } else {
            None
        };
        return (entry_funcs.message_begin_post)(msg_dest, msg_type, origin, ent);
    }

    meta_const::RESULT_IGNORED
}

pub fn write_byte_post(value: i32) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.write_byte_post)(value);
    }

    meta_const::RESULT_IGNORED
}

pub fn write_char_post(value: i32) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.write_char_post)(value);
    }

    meta_const::RESULT_IGNORED
}

pub fn write_short_post(value: i32) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.write_short_post)(value);
    }

    meta_const::RESULT_IGNORED
}

pub fn write_long_post(value: i32) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.write_long_post)(value);
    }

    meta_const::RESULT_IGNORED
}

pub fn write_angle_post(value: f32) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.write_angle_post)(value);
    }

    meta_const::RESULT_IGNORED
}

pub fn write_coord_post(value: f32) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.write_coord_post)(value);
    }

    meta_const::RESULT_IGNORED
}

pub fn write_string_post(value: *const ::std::os::raw::c_char) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        let value = meta::c_char_to_string(value);
        return (entry_funcs.write_string_post)(value);
    }

    meta_const::RESULT_IGNORED
}

pub fn write_entity_post(value: i32) -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.write_entity_post)(value);
    }

    meta_const::RESULT_IGNORED
}

pub fn message_end_post() -> i32 {
    if let Some(entry_funcs) = ENTRY_FUNCS.get() {
        return (entry_funcs.message_end_post)();
    }

    meta_const::RESULT_IGNORED
}
