use crate::{
    adapter::{entry, messages_handler, player},
    metamod::meta_api,
    plugin,
};

pub fn load() {
    meta_api::setup_entry(
        entry::adapter_init,
        entry::adapter_setup,
        entry::client_command,
        messages_handler::message_begin,
        messages_handler::write_byte,
        messages_handler::write_char,
        messages_handler::write_short,
        messages_handler::write_long,
        messages_handler::write_angle,
        messages_handler::write_coord,
        messages_handler::write_string,
        messages_handler::write_entity,
        messages_handler::message_end,
        messages_handler::message_begin_post,
        messages_handler::write_byte_post,
        messages_handler::write_char_post,
        messages_handler::write_short_post,
        messages_handler::write_long_post,
        messages_handler::write_angle_post,
        messages_handler::write_coord_post,
        messages_handler::write_string_post,
        messages_handler::write_entity_post,
        messages_handler::message_end_post,
    );

    plugin::load();

    let p = player::get_player(13);
    if let Some(p) = p {
        p.connected = true;
    }
}
