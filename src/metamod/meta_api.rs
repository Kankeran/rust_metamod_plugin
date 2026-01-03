use crate::metamod::{abi, entry, msgs};

pub fn setup_entry(
    init: fn(),
    setup: fn(),
    client_command: fn(i32, Vec<String>) -> i32,
    message_begin: fn(i32, i32, *const f32, *mut abi::edict_t) -> i32,
    write_byte: fn(i32) -> i32,
    write_char: fn(i32) -> i32,
    write_short: fn(i32) -> i32,
    write_long: fn(i32) -> i32,
    write_angle: fn(f32) -> i32,
    write_coord: fn(f32) -> i32,
    write_string: fn(String) -> i32,
    write_entity: fn(i32) -> i32,
    message_end: fn() -> i32,
    message_begin_post: fn(i32, i32, *const f32, *mut abi::edict_t) -> i32,
    write_byte_post: fn(i32) -> i32,
    write_char_post: fn(i32) -> i32,
    write_short_post: fn(i32) -> i32,
    write_long_post: fn(i32) -> i32,
    write_angle_post: fn(f32) -> i32,
    write_coord_post: fn(f32) -> i32,
    write_string_post: fn(String) -> i32,
    write_entity_post: fn(i32) -> i32,
    message_end_post: fn() -> i32,
) {
    let _ = entry::ENTRY_FUNCS.set(entry::EntryFuncs::new(
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
    ));
}

pub fn get_text_msg_id() -> Option<i32> {
    unsafe { msgs::TEXT_MSG }
}

pub fn get_bar_time_id() -> Option<i32> {
    unsafe { msgs::BAR_TIME }
}

pub fn get_cur_weapon_id() -> Option<i32> {
    unsafe { msgs::CUR_WEAPON }
}

pub fn get_damage_id() -> Option<i32> {
    unsafe { msgs::DAMAGE }
}

pub fn get_death_msg_id() -> Option<i32> {
    unsafe { msgs::DEATH_MSG }
}

pub fn get_team_info_id() -> Option<i32> {
    unsafe { msgs::TEAM_INFO }
}

pub fn get_weapon_list_id() -> Option<i32> {
    unsafe { msgs::WEAPON_LIST }
}

pub fn get_motd_id() -> Option<i32> {
    unsafe { msgs::MOTD }
}

pub fn get_server_name_id() -> Option<i32> {
    unsafe { msgs::SERVER_NAME }
}

pub fn get_health_id() -> Option<i32> {
    unsafe { msgs::HEALTH }
}

pub fn get_battery_id() -> Option<i32> {
    unsafe { msgs::BATTERY }
}

pub fn get_show_menu_id() -> Option<i32> {
    unsafe { msgs::SHOW_MENU }
}

pub fn get_send_audio_id() -> Option<i32> {
    unsafe { msgs::SEND_AUDIO }
}

pub fn get_ammo_x_id() -> Option<i32> {
    unsafe { msgs::AMMO_X }
}

pub fn get_score_info_id() -> Option<i32> {
    unsafe { msgs::SCORE_INFO }
}

pub fn get_vgui_menu_id() -> Option<i32> {
    unsafe { msgs::VGUI_MENU }
}

pub fn get_ammo_pickup_id() -> Option<i32> {
    unsafe { msgs::AMMO_PICKUP }
}

pub fn get_weap_pickup_id() -> Option<i32> {
    unsafe { msgs::WEAP_PICKUP }
}

pub fn get_reset_hud_id() -> Option<i32> {
    unsafe { msgs::RESET_HUD }
}

pub fn get_round_time_id() -> Option<i32> {
    unsafe { msgs::ROUND_TIME }
}

pub fn get_say_text_id() -> Option<i32> {
    unsafe { msgs::SAY_TEXT }
}

pub fn get_init_hud_id() -> Option<i32> {
    unsafe { msgs::INIT_HUD }
}
