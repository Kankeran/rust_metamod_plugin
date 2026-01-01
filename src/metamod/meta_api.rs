use crate::metamod::msgs;

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
