use crate::{
    adapter::api,
    metamod::{meta_api, meta_const},
};

pub fn result(result: &api::Return) -> i32 {
    match result {
        api::Return::Ignored => meta_const::RESULT_IGNORED,
        api::Return::Handled => meta_const::RESULT_HANDLED,
        api::Return::HandledMain => meta_const::RESULT_SUPERCEDE,
        api::Return::Override => meta_const::RESULT_OVERRIDE,
        api::Return::Supercede => meta_const::RESULT_SUPERCEDE,
    }
}

pub fn print_mode(mode: &api::PrintMode) -> i32 {
    match mode {
        api::PrintMode::PrintNotify => meta_const::PRINT_NOTIFY,
        api::PrintMode::PrintConsole => meta_const::PRINT_CONSOLE,
        api::PrintMode::PrintChat => meta_const::PRINT_CHAT,
        api::PrintMode::PrintCenter => meta_const::PRINT_CENTER,
    }
}

pub fn user_msg_id(msg: &api::UserMsgs) -> Option<i32> {
    match msg {
        api::UserMsgs::TextMsg => meta_api::get_text_msg_id(),
        api::UserMsgs::BarTime => meta_api::get_bar_time_id(),
        api::UserMsgs::CurWeapon => meta_api::get_cur_weapon_id(),
        api::UserMsgs::Damage => meta_api::get_damage_id(),
        api::UserMsgs::DeathMsg => meta_api::get_death_msg_id(),
        api::UserMsgs::TeamInfo => meta_api::get_team_info_id(),
        api::UserMsgs::WeaponList => meta_api::get_weapon_list_id(),
        api::UserMsgs::MOTD => meta_api::get_motd_id(),
        api::UserMsgs::ServerName => meta_api::get_server_name_id(),
        api::UserMsgs::Health => meta_api::get_health_id(),
        api::UserMsgs::Battery => meta_api::get_battery_id(),
        api::UserMsgs::ShowMenu => meta_api::get_show_menu_id(),
        api::UserMsgs::SendAudio => meta_api::get_send_audio_id(),
        api::UserMsgs::AmmoX => meta_api::get_ammo_x_id(),
        api::UserMsgs::ScoreInfo => meta_api::get_score_info_id(),
        api::UserMsgs::VguiMenu => meta_api::get_vgui_menu_id(),
        api::UserMsgs::AmmoPickup => meta_api::get_ammo_pickup_id(),
        api::UserMsgs::WeapPickup => meta_api::get_weap_pickup_id(),
        api::UserMsgs::ResetHud => meta_api::get_reset_hud_id(),
        api::UserMsgs::RoundTime => meta_api::get_round_time_id(),
        api::UserMsgs::SayText => meta_api::get_say_text_id(),
        api::UserMsgs::InitHud => meta_api::get_init_hud_id(),
    }
}
