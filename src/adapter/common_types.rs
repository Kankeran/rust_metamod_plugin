use crate::adapter::metamod::{meta_api, meta_const};

pub enum Return {
    Ignored,
    Handled,
    Override,
    Supercede,
    DeferSupercede,
}

impl Return {
    pub fn lt(&self, ret: &Return) -> bool {
        match (self, ret) {
            (
                Return::Ignored,
                Return::Handled | Return::Override | Return::Supercede | Return::DeferSupercede,
            ) => true,
            (Return::Handled, Return::Override | Return::Supercede | Return::DeferSupercede) => {
                true
            }
            (Return::Override, Return::Supercede | Return::DeferSupercede) => true,
            (_, _) => false,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Return::Ignored => meta_const::RESULT_IGNORED,
            Return::Handled => meta_const::RESULT_HANDLED,
            Return::Override => meta_const::RESULT_OVERRIDE,
            Return::Supercede => meta_const::RESULT_SUPERCEDE,
            Return::DeferSupercede => meta_const::RESULT_SUPERCEDE,
        }
    }
}

#[derive(Clone, Copy)]
pub enum BlockMode {
    BlockNone,
    BlockOne,
    BlockAll,
}

pub enum PrintMode {
    PrintNotify,
    PrintConsole,
    PrintChat,
    PrintCenter,
}

impl PrintMode {
    pub fn to_i32(&self) -> i32 {
        match self {
            PrintMode::PrintNotify => meta_const::PRINT_NOTIFY,
            PrintMode::PrintConsole => meta_const::PRINT_CONSOLE,
            PrintMode::PrintChat => meta_const::PRINT_CHAT,
            PrintMode::PrintCenter => meta_const::PRINT_CENTER,
        }
    }

    pub fn from_i32(mode: i32) -> Self {
        match mode {
            meta_const::PRINT_NOTIFY => PrintMode::PrintNotify,
            meta_const::PRINT_CONSOLE => PrintMode::PrintConsole,
            meta_const::PRINT_CHAT => PrintMode::PrintChat,
            meta_const::PRINT_CENTER => PrintMode::PrintCenter,
            _ => PrintMode::PrintChat,
        }
    }
}

#[derive(Debug)]
pub enum UserMsgs {
    TextMsg,
    BarTime,
    CurWeapon,
    Damage,
    DeathMsg,
    TeamInfo,
    WeaponList,
    MOTD,
    ServerName,
    Health,
    Battery,
    ShowMenu,
    SendAudio,
    AmmoX,
    ScoreInfo,
    VguiMenu,
    AmmoPickup,
    WeapPickup,
    ResetHud,
    RoundTime,
    SayText,
    InitHud,
    SvcTempEntity,
}

impl UserMsgs {
    pub fn to_option_i32(&self) -> Option<i32> {
        match self {
            UserMsgs::TextMsg => meta_api::get_text_msg_id(),
            UserMsgs::BarTime => meta_api::get_bar_time_id(),
            UserMsgs::CurWeapon => meta_api::get_cur_weapon_id(),
            UserMsgs::Damage => meta_api::get_damage_id(),
            UserMsgs::DeathMsg => meta_api::get_death_msg_id(),
            UserMsgs::TeamInfo => meta_api::get_team_info_id(),
            UserMsgs::WeaponList => meta_api::get_weapon_list_id(),
            UserMsgs::MOTD => meta_api::get_motd_id(),
            UserMsgs::ServerName => meta_api::get_server_name_id(),
            UserMsgs::Health => meta_api::get_health_id(),
            UserMsgs::Battery => meta_api::get_battery_id(),
            UserMsgs::ShowMenu => meta_api::get_show_menu_id(),
            UserMsgs::SendAudio => meta_api::get_send_audio_id(),
            UserMsgs::AmmoX => meta_api::get_ammo_x_id(),
            UserMsgs::ScoreInfo => meta_api::get_score_info_id(),
            UserMsgs::VguiMenu => meta_api::get_vgui_menu_id(),
            UserMsgs::AmmoPickup => meta_api::get_ammo_pickup_id(),
            UserMsgs::WeapPickup => meta_api::get_weap_pickup_id(),
            UserMsgs::ResetHud => meta_api::get_reset_hud_id(),
            UserMsgs::RoundTime => meta_api::get_round_time_id(),
            UserMsgs::SayText => meta_api::get_say_text_id(),
            UserMsgs::InitHud => meta_api::get_init_hud_id(),
            UserMsgs::SvcTempEntity => Some(meta_const::SVC_TEMPENTITY),
        }
    }

    pub fn try_from_i32(msg_id: i32) -> Option<Self> {
        if let Some(id) = meta_api::get_text_msg_id()
            && msg_id == id
        {
            Some(UserMsgs::TextMsg)
        } else if let Some(id) = meta_api::get_bar_time_id()
            && msg_id == id
        {
            Some(UserMsgs::BarTime)
        } else if let Some(id) = meta_api::get_cur_weapon_id()
            && msg_id == id
        {
            Some(UserMsgs::CurWeapon)
        } else if let Some(id) = meta_api::get_damage_id()
            && msg_id == id
        {
            Some(UserMsgs::Damage)
        } else if let Some(id) = meta_api::get_death_msg_id()
            && msg_id == id
        {
            Some(UserMsgs::DeathMsg)
        } else if let Some(id) = meta_api::get_team_info_id()
            && msg_id == id
        {
            Some(UserMsgs::TeamInfo)
        } else if let Some(id) = meta_api::get_weapon_list_id()
            && msg_id == id
        {
            Some(UserMsgs::WeaponList)
        } else if let Some(id) = meta_api::get_motd_id()
            && msg_id == id
        {
            Some(UserMsgs::MOTD)
        } else if let Some(id) = meta_api::get_server_name_id()
            && msg_id == id
        {
            Some(UserMsgs::ServerName)
        } else if let Some(id) = meta_api::get_health_id()
            && msg_id == id
        {
            Some(UserMsgs::Health)
        } else if let Some(id) = meta_api::get_battery_id()
            && msg_id == id
        {
            Some(UserMsgs::Battery)
        } else if let Some(id) = meta_api::get_show_menu_id()
            && msg_id == id
        {
            Some(UserMsgs::ShowMenu)
        } else if let Some(id) = meta_api::get_send_audio_id()
            && msg_id == id
        {
            Some(UserMsgs::SendAudio)
        } else if let Some(id) = meta_api::get_ammo_x_id()
            && msg_id == id
        {
            Some(UserMsgs::AmmoX)
        } else if let Some(id) = meta_api::get_score_info_id()
            && msg_id == id
        {
            Some(UserMsgs::ScoreInfo)
        } else if let Some(id) = meta_api::get_vgui_menu_id()
            && msg_id == id
        {
            Some(UserMsgs::VguiMenu)
        } else if let Some(id) = meta_api::get_ammo_pickup_id()
            && msg_id == id
        {
            Some(UserMsgs::AmmoPickup)
        } else if let Some(id) = meta_api::get_weap_pickup_id()
            && msg_id == id
        {
            Some(UserMsgs::WeapPickup)
        } else if let Some(id) = meta_api::get_reset_hud_id()
            && msg_id == id
        {
            Some(UserMsgs::ResetHud)
        } else if let Some(id) = meta_api::get_round_time_id()
            && msg_id == id
        {
            Some(UserMsgs::RoundTime)
        } else if let Some(id) = meta_api::get_say_text_id()
            && msg_id == id
        {
            Some(UserMsgs::SayText)
        } else if let Some(id) = meta_api::get_init_hud_id()
            && msg_id == id
        {
            Some(UserMsgs::InitHud)
        } else if msg_id == meta_const::SVC_TEMPENTITY {
            Some(UserMsgs::SvcTempEntity)
        } else {
            None
        }
    }
}
