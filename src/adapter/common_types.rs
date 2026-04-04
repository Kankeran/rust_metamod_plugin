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

#[derive(Debug)]
pub enum BlockMode {
    BlockNone,
    BlockOne,
    BlockAll,
}

#[derive(Debug)]
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
    VoiceMask,
    ReqState,
    Geiger,
    Flashlight,
    FlashBat,
    Train,
    HudTextPro,
    HudText,
    ViewMode,
    GameTitle,
    ScoreAttrib,
    TeamScore,
    GameMode,
    ItemPickup,
    HideWeapon,
    SetFOV,
    ScreenShake,
    ScreenFade,
    Money,
    ArmorType,
    BlinkAcct,
    StatusValue,
    StatusText,
    StatusIcon,
    ReloadSound,
    Crosshair,
    NVGToggle,
    Radar,
    Spectator,
    TutorText,
    TutorLine,
    TutorState,
    TutorClose,
    AllowSpec,
    BombDrop,
    BombPickup,
    ClCorpse,
    HostagePos,
    HostageK,
    HLTV,
    SpecHealth,
    ForceCam,
    ADStop,
    ReceiveW,
    CZCareer,
    CZCareerHUD,
    ShadowIdx,
    TaskTime,
    Scenario,
    BotVoice,
    BuyClose,
    SpecHealth2,
    BarTime2,
    ItemStatus,
    Location,
    BotProgress,
    Brass,
    Fog,
    ShowTimer,
    HudTextArgs,
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
            UserMsgs::VoiceMask => meta_api::get_voice_mask_id(),
            UserMsgs::ReqState => meta_api::get_req_state_id(),
            UserMsgs::Geiger => meta_api::get_geiger_id(),
            UserMsgs::Flashlight => meta_api::get_flashlight_id(),
            UserMsgs::FlashBat => meta_api::get_flash_bat_id(),
            UserMsgs::Train => meta_api::get_train_id(),
            UserMsgs::HudTextPro => meta_api::get_hud_text_pro_id(),
            UserMsgs::HudText => meta_api::get_hud_text_id(),
            UserMsgs::ViewMode => meta_api::get_view_mode_id(),
            UserMsgs::GameTitle => meta_api::get_game_title_id(),
            UserMsgs::ScoreAttrib => meta_api::get_score_attrib_id(),
            UserMsgs::TeamScore => meta_api::get_team_score_id(),
            UserMsgs::GameMode => meta_api::get_game_mode_id(),
            UserMsgs::ItemPickup => meta_api::get_item_pickup_id(),
            UserMsgs::HideWeapon => meta_api::get_hide_weapon_id(),
            UserMsgs::SetFOV => meta_api::get_set_fov_id(),
            UserMsgs::ScreenShake => meta_api::get_screen_shake_id(),
            UserMsgs::ScreenFade => meta_api::get_screen_fade_id(),
            UserMsgs::Money => meta_api::get_money_id(),
            UserMsgs::ArmorType => meta_api::get_armor_type_id(),
            UserMsgs::BlinkAcct => meta_api::get_blink_acct_id(),
            UserMsgs::StatusValue => meta_api::get_status_value_id(),
            UserMsgs::StatusText => meta_api::get_status_text_id(),
            UserMsgs::StatusIcon => meta_api::get_status_icon_id(),
            UserMsgs::ReloadSound => meta_api::get_reload_sound_id(),
            UserMsgs::Crosshair => meta_api::get_crosshair_id(),
            UserMsgs::NVGToggle => meta_api::get_nvg_toggle_id(),
            UserMsgs::Radar => meta_api::get_radar_id(),
            UserMsgs::Spectator => meta_api::get_spectator_id(),
            UserMsgs::TutorText => meta_api::get_tutor_text_id(),
            UserMsgs::TutorLine => meta_api::get_tutor_line_id(),
            UserMsgs::TutorState => meta_api::get_tutor_state_id(),
            UserMsgs::TutorClose => meta_api::get_tutor_close_id(),
            UserMsgs::AllowSpec => meta_api::get_allow_spec_id(),
            UserMsgs::BombDrop => meta_api::get_bomb_drop_id(),
            UserMsgs::BombPickup => meta_api::get_bomb_pickup_id(),
            UserMsgs::ClCorpse => meta_api::get_cl_corpse_id(),
            UserMsgs::HostagePos => meta_api::get_hostage_pos_id(),
            UserMsgs::HostageK => meta_api::get_hostage_k_id(),
            UserMsgs::HLTV => meta_api::get_hltv_id(),
            UserMsgs::SpecHealth => meta_api::get_spec_health_id(),
            UserMsgs::ForceCam => meta_api::get_force_cam_id(),
            UserMsgs::ADStop => meta_api::get_ad_stop_id(),
            UserMsgs::ReceiveW => meta_api::get_receive_w_id(),
            UserMsgs::CZCareer => meta_api::get_cz_career_id(),
            UserMsgs::CZCareerHUD => meta_api::get_cz_career_hud_id(),
            UserMsgs::ShadowIdx => meta_api::get_shadow_idx_id(),
            UserMsgs::TaskTime => meta_api::get_task_time_id(),
            UserMsgs::Scenario => meta_api::get_scenario_id(),
            UserMsgs::BotVoice => meta_api::get_bot_voice_id(),
            UserMsgs::BuyClose => meta_api::get_buy_close_id(),
            UserMsgs::SpecHealth2 => meta_api::get_spec_health2_id(),
            UserMsgs::BarTime2 => meta_api::get_bar_time2_id(),
            UserMsgs::ItemStatus => meta_api::get_item_status_id(),
            UserMsgs::Location => meta_api::get_location_id(),
            UserMsgs::BotProgress => meta_api::get_bot_progress_id(),
            UserMsgs::Brass => meta_api::get_brass_id(),
            UserMsgs::Fog => meta_api::get_fog_id(),
            UserMsgs::ShowTimer => meta_api::get_show_timer_id(),
            UserMsgs::HudTextArgs => meta_api::get_hud_text_args_id(),
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
        } else if let Some(id) = meta_api::get_voice_mask_id()
         && msg_id == id
         {
            Some(UserMsgs::VoiceMask)
         }
         else if let Some(id) = meta_api::get_req_state_id()
         && msg_id == id
         {
            Some(UserMsgs::ReqState)
         }
         else if let Some(id) = meta_api::get_geiger_id()
         && msg_id == id
         {
            Some(UserMsgs::Geiger)
         }
         else if let Some(id) = meta_api::get_flashlight_id()
         && msg_id == id
         {
            Some(UserMsgs::Flashlight)
         }
         else if let Some(id) = meta_api::get_flash_bat_id()
         && msg_id == id
         {
            Some(UserMsgs::FlashBat)
         }
         else if let Some(id) = meta_api::get_train_id()
         && msg_id == id
         {
            Some(UserMsgs::Train)
         }
         else if let Some(id) = meta_api::get_hud_text_pro_id()
         && msg_id == id
         {
            Some(UserMsgs::HudTextPro)
         }
         else if let Some(id) = meta_api::get_hud_text_id()
         && msg_id == id
         {
            Some(UserMsgs::HudText)
         }
         else if let Some(id) = meta_api::get_view_mode_id()
         && msg_id == id
         {
            Some(UserMsgs::ViewMode)
         }
         else if let Some(id) = meta_api::get_game_title_id()
         && msg_id == id
         {
            Some(UserMsgs::GameTitle)
         }
         else if let Some(id) = meta_api::get_score_attrib_id()
         && msg_id == id
         {
            Some(UserMsgs::ScoreAttrib)
         }
         else if let Some(id) = meta_api::get_team_score_id()
         && msg_id == id
         {
            Some(UserMsgs::TeamScore)
         }
         else if let Some(id) = meta_api::get_game_mode_id()
         && msg_id == id
         {
            Some(UserMsgs::GameMode)
         }
         else if let Some(id) = meta_api::get_item_pickup_id()
         && msg_id == id
         {
            Some(UserMsgs::ItemPickup)
         }
         else if let Some(id) = meta_api::get_hide_weapon_id()
         && msg_id == id
         {
            Some(UserMsgs::HideWeapon)
         }
         else if let Some(id) = meta_api::get_set_fov_id()
         && msg_id == id
         {
            Some(UserMsgs::SetFOV)
         }
         else if let Some(id) = meta_api::get_screen_shake_id()
         && msg_id == id
         {
            Some(UserMsgs::ScreenShake)
         }
         else if let Some(id) = meta_api::get_screen_fade_id()
         && msg_id == id
         {
            Some(UserMsgs::ScreenFade)
         }
         else if let Some(id) = meta_api::get_money_id()
         && msg_id == id
         {
            Some(UserMsgs::Money)
         }
         else if let Some(id) = meta_api::get_armor_type_id()
         && msg_id == id
         {
            Some(UserMsgs::ArmorType)
         }
         else if let Some(id) = meta_api::get_blink_acct_id()
         && msg_id == id
         {
            Some(UserMsgs::BlinkAcct)
         }
         else if let Some(id) = meta_api::get_status_value_id()
         && msg_id == id
         {
            Some(UserMsgs::StatusValue)
         }
         else if let Some(id) = meta_api::get_status_text_id()
         && msg_id == id
         {
            Some(UserMsgs::StatusText)
         }
         else if let Some(id) = meta_api::get_status_icon_id()
         && msg_id == id
         {
            Some(UserMsgs::StatusIcon)
         }
         else if let Some(id) = meta_api::get_reload_sound_id()
         && msg_id == id
         {
            Some(UserMsgs::ReloadSound)
         }
         else if let Some(id) = meta_api::get_crosshair_id()
         && msg_id == id
         {
            Some(UserMsgs::Crosshair)
         }
         else if let Some(id) = meta_api::get_nvg_toggle_id()
         && msg_id == id
         {
            Some(UserMsgs::NVGToggle)
         }
         else if let Some(id) = meta_api::get_radar_id()
         && msg_id == id
         {
            Some(UserMsgs::Radar)
         }
         else if let Some(id) = meta_api::get_spectator_id()
         && msg_id == id
         {
            Some(UserMsgs::Spectator)
         }
         else if let Some(id) = meta_api::get_tutor_text_id()
         && msg_id == id
         {
            Some(UserMsgs::TutorText)
         }
         else if let Some(id) = meta_api::get_tutor_line_id()
         && msg_id == id
         {
            Some(UserMsgs::TutorLine)
         }
         else if let Some(id) = meta_api::get_tutor_state_id()
         && msg_id == id
         {
            Some(UserMsgs::TutorState)
         }
         else if let Some(id) = meta_api::get_tutor_close_id()
         && msg_id == id
         {
            Some(UserMsgs::TutorClose)
         }
         else if let Some(id) = meta_api::get_allow_spec_id()
         && msg_id == id
         {
            Some(UserMsgs::AllowSpec)
         }
         else if let Some(id) = meta_api::get_bomb_drop_id()
         && msg_id == id
         {
            Some(UserMsgs::BombDrop)
         }
         else if let Some(id) = meta_api::get_bomb_pickup_id()
         && msg_id == id
         {
            Some(UserMsgs::BombPickup)
         }
         else if let Some(id) = meta_api::get_cl_corpse_id()
         && msg_id == id
         {
            Some(UserMsgs::ClCorpse)
         }
         else if let Some(id) = meta_api::get_hostage_pos_id()
         && msg_id == id
         {
            Some(UserMsgs::HostagePos)
         }
         else if let Some(id) = meta_api::get_hostage_k_id()
         && msg_id == id
         {
            Some(UserMsgs::HostageK)
         }
         else if let Some(id) = meta_api::get_hltv_id()
         && msg_id == id
         {
            Some(UserMsgs::HLTV)
         }
         else if let Some(id) = meta_api::get_spec_health_id()
         && msg_id == id
         {
            Some(UserMsgs::SpecHealth)
         }
         else if let Some(id) = meta_api::get_force_cam_id()
         && msg_id == id
         {
            Some(UserMsgs::ForceCam)
         }
         else if let Some(id) = meta_api::get_ad_stop_id()
         && msg_id == id
         {
            Some(UserMsgs::ADStop)
         }
         else if let Some(id) = meta_api::get_receive_w_id()
         && msg_id == id
         {
            Some(UserMsgs::ReceiveW)
         }
         else if let Some(id) = meta_api::get_cz_career_id()
         && msg_id == id
         {
            Some(UserMsgs::CZCareer)
         }
         else if let Some(id) = meta_api::get_cz_career_hud_id()
         && msg_id == id
         {
            Some(UserMsgs::CZCareerHUD)
         }
         else if let Some(id) = meta_api::get_shadow_idx_id()
         && msg_id == id
         {
            Some(UserMsgs::ShadowIdx)
         }
         else if let Some(id) = meta_api::get_task_time_id()
         && msg_id == id
         {
            Some(UserMsgs::TaskTime)
         }
         else if let Some(id) = meta_api::get_scenario_id()
         && msg_id == id
         {
            Some(UserMsgs::Scenario)
         }
         else if let Some(id) = meta_api::get_bot_voice_id()
         && msg_id == id
         {
            Some(UserMsgs::BotVoice)
         }
         else if let Some(id) = meta_api::get_buy_close_id()
         && msg_id == id
         {
            Some(UserMsgs::BuyClose)
         }
         else if let Some(id) = meta_api::get_spec_health2_id()
         && msg_id == id
         {
            Some(UserMsgs::SpecHealth2)
         }
         else if let Some(id) = meta_api::get_bar_time2_id()
         && msg_id == id
         {
            Some(UserMsgs::BarTime2)
         }
         else if let Some(id) = meta_api::get_item_status_id()
         && msg_id == id
         {
            Some(UserMsgs::ItemStatus)
         }
         else if let Some(id) = meta_api::get_location_id()
         && msg_id == id
         {
            Some(UserMsgs::Location)
         }
         else if let Some(id) = meta_api::get_bot_progress_id()
         && msg_id == id
         {
            Some(UserMsgs::BotProgress)
         }
         else if let Some(id) = meta_api::get_brass_id()
         && msg_id == id
         {
            Some(UserMsgs::Brass)
         }
         else if let Some(id) = meta_api::get_fog_id()
         && msg_id == id
         {
            Some(UserMsgs::Fog)
         }
         else if let Some(id) = meta_api::get_show_timer_id()
         && msg_id == id
         {
            Some(UserMsgs::ShowTimer)
         }
         else if let Some(id) = meta_api::get_hud_text_args_id()
         && msg_id == id
         {
            Some(UserMsgs::HudTextArgs)
         } else if msg_id == meta_const::SVC_TEMPENTITY {
            Some(UserMsgs::SvcTempEntity)
        } else {
            None
        }

         
    }
}
