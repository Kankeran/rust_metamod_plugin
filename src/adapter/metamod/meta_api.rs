use crate::adapter::metamod::abi::{edict_t, entvars_t};

use super::{abi, entry, meta, msgs};
use std::{
    ffi::CString,
    ptr::{self, null},
};

#[derive(Debug)]
pub struct EdictPtr(*mut abi::edict_t);

unsafe impl Sync for EdictPtr {}
unsafe impl Send for EdictPtr {}

impl EdictPtr {
    pub fn new(ent: *mut abi::edict_t) -> EdictPtr {
        EdictPtr(ent)
    }

    pub fn as_ptr(&self) -> *mut abi::edict_t {
        self.0
    }
}

pub fn vars(pent: *mut edict_t) -> *const entvars_t {
    if pent.is_null() {
        return null();
    }

    return unsafe { ptr::addr_of!((*pent).v) };
}

pub fn setup_entry(
    init: fn(),
    setup: fn(),
    client_command: fn(i32, Vec<String>) -> i32,
    message_begin: fn(i32, i32, Option<[f32; 3]>, Option<EdictPtr>) -> i32,
    write_byte: fn(i32) -> i32,
    write_char: fn(i32) -> i32,
    write_short: fn(i32) -> i32,
    write_long: fn(i32) -> i32,
    write_angle: fn(f32) -> i32,
    write_coord: fn(f32) -> i32,
    write_string: fn(String) -> i32,
    write_entity: fn(i32) -> i32,
    message_end: fn() -> i32,
    message_begin_post: fn(i32, i32, Option<[f32; 3]>, Option<EdictPtr>) -> i32,
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

pub fn get_ent_index(entity: Option<&EdictPtr>) -> Option<i32> {
    if let Some(ent) = entity {
        meta::get_ent_index(ent.as_ptr())
    } else {
        None
    }
}

pub fn get_ent_by_index_option(index: Option<i32>) -> Option<EdictPtr> {
    if let Some(index) = index {
        if let Some(ent) = meta::get_ent_by_index(index)
            && !ent.is_null()
        {
            Some(EdictPtr::new(ent))
        } else {
            None
        }
    } else {
        None
    }
}

pub fn get_ent_by_index(index: i32) -> Option<EdictPtr> {
    if let Some(ent) = meta::get_ent_by_index(index)
        && !ent.is_null()
    {
        Some(EdictPtr::new(ent))
    } else {
        None
    }
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

pub fn get_voice_mask_id() -> Option<i32> {
    unsafe { msgs::VOICE_MASK }
}

pub fn get_req_state_id() -> Option<i32> {
    unsafe { msgs::REQ_STATE }
}

pub fn get_geiger_id() -> Option<i32> {
    unsafe { msgs::GEIGER }
}

pub fn get_flashlight_id() -> Option<i32> {
    unsafe { msgs::FLASHLIGHT }
}

pub fn get_flash_bat_id() -> Option<i32> {
    unsafe { msgs::FLASH_BAT }
}

pub fn get_train_id() -> Option<i32> {
    unsafe { msgs::TRAIN }
}

pub fn get_hud_text_pro_id() -> Option<i32> {
    unsafe { msgs::HUD_TEXT_PRO }
}

pub fn get_hud_text_id() -> Option<i32> {
    unsafe { msgs::HUD_TEXT }
}

pub fn get_view_mode_id() -> Option<i32> {
    unsafe { msgs::VIEW_MODE }
}

pub fn get_game_title_id() -> Option<i32> {
    unsafe { msgs::GAME_TITLE }
}

pub fn get_score_attrib_id() -> Option<i32> {
    unsafe { msgs::SCORE_ATTRIB }
}

pub fn get_team_score_id() -> Option<i32> {
    unsafe { msgs::TEAM_SCORE }
}

pub fn get_game_mode_id() -> Option<i32> {
    unsafe { msgs::GAME_MODE }
}

pub fn get_item_pickup_id() -> Option<i32> {
    unsafe { msgs::ITEM_PICKUP }
}

pub fn get_hide_weapon_id() -> Option<i32> {
    unsafe { msgs::HIDE_WEAPON }
}

pub fn get_set_fov_id() -> Option<i32> {
    unsafe { msgs::SET_FOV }
}

pub fn get_screen_shake_id() -> Option<i32> {
    unsafe { msgs::SCREEN_SHAKE }
}

pub fn get_screen_fade_id() -> Option<i32> {
    unsafe { msgs::SCREEN_FADE }
}

pub fn get_money_id() -> Option<i32> {
    unsafe { msgs::MONEY }
}

pub fn get_armor_type_id() -> Option<i32> {
    unsafe { msgs::ARMOR_TYPE }
}

pub fn get_blink_acct_id() -> Option<i32> {
    unsafe { msgs::BLINK_ACCT }
}

pub fn get_status_value_id() -> Option<i32> {
    unsafe { msgs::STATUS_VALUE }
}

pub fn get_status_text_id() -> Option<i32> {
    unsafe { msgs::STATUS_TEXT }
}

pub fn get_status_icon_id() -> Option<i32> {
    unsafe { msgs::STATUS_ICON }
}

pub fn get_reload_sound_id() -> Option<i32> {
    unsafe { msgs::RELOAD_SOUND }
}

pub fn get_crosshair_id() -> Option<i32> {
    unsafe { msgs::CROSSHAIR }
}

pub fn get_nvg_toggle_id() -> Option<i32> {
    unsafe { msgs::NVG_TOGGLE }
}

pub fn get_radar_id() -> Option<i32> {
    unsafe { msgs::RADAR }
}

pub fn get_spectator_id() -> Option<i32> {
    unsafe { msgs::SPECTATOR }
}

pub fn get_tutor_text_id() -> Option<i32> {
    unsafe { msgs::TUTOR_TEXT }
}

pub fn get_tutor_line_id() -> Option<i32> {
    unsafe { msgs::TUTOR_LINE }
}

pub fn get_tutor_state_id() -> Option<i32> {
    unsafe { msgs::TUTOR_STATE }
}

pub fn get_tutor_close_id() -> Option<i32> {
    unsafe { msgs::TUTOR_CLOSE }
}

pub fn get_allow_spec_id() -> Option<i32> {
    unsafe { msgs::ALLOW_SPEC }
}

pub fn get_bomb_drop_id() -> Option<i32> {
    unsafe { msgs::BOMB_DROP }
}

pub fn get_bomb_pickup_id() -> Option<i32> {
    unsafe { msgs::BOMB_PICKUP }
}

pub fn get_cl_corpse_id() -> Option<i32> {
    unsafe { msgs::CL_CORPSE }
}

pub fn get_hostage_pos_id() -> Option<i32> {
    unsafe { msgs::HOSTAGE_POS }
}

pub fn get_hostage_k_id() -> Option<i32> {
    unsafe { msgs::HOSTAGE_K }
}

pub fn get_hltv_id() -> Option<i32> {
    unsafe { msgs::HLTV }
}

pub fn get_spec_health_id() -> Option<i32> {
    unsafe { msgs::SPEC_HEALTH }
}

pub fn get_force_cam_id() -> Option<i32> {
    unsafe { msgs::FORCE_CAM }
}

pub fn get_ad_stop_id() -> Option<i32> {
    unsafe { msgs::AD_STOP }
}

pub fn get_receive_w_id() -> Option<i32> {
    unsafe { msgs::RECEIVE_W }
}

pub fn get_cz_career_id() -> Option<i32> {
    unsafe { msgs::CZ_CAREER }
}

pub fn get_cz_career_hud_id() -> Option<i32> {
    unsafe { msgs::CZ_CAREER_HUD }
}

pub fn get_shadow_idx_id() -> Option<i32> {
    unsafe { msgs::SHADOW_IDX }
}

pub fn get_task_time_id() -> Option<i32> {
    unsafe { msgs::TASK_TIME }
}

pub fn get_scenario_id() -> Option<i32> {
    unsafe { msgs::SCENARIO }
}

pub fn get_bot_voice_id() -> Option<i32> {
    unsafe { msgs::BOT_VOICE }
}

pub fn get_buy_close_id() -> Option<i32> {
    unsafe { msgs::BUY_CLOSE }
}

pub fn get_spec_health2_id() -> Option<i32> {
    unsafe { msgs::SPEC_HEALTH2 }
}

pub fn get_bar_time2_id() -> Option<i32> {
    unsafe { msgs::BAR_TIME2 }
}

pub fn get_item_status_id() -> Option<i32> {
    unsafe { msgs::ITEM_STATUS }
}

pub fn get_location_id() -> Option<i32> {
    unsafe { msgs::LOCATION }
}

pub fn get_bot_progress_id() -> Option<i32> {
    unsafe { msgs::BOT_PROGRESS }
}

pub fn get_brass_id() -> Option<i32> {
    unsafe { msgs::BRASS }
}

pub fn get_fog_id() -> Option<i32> {
    unsafe { msgs::FOG }
}

pub fn get_show_timer_id() -> Option<i32> {
    unsafe { msgs::SHOW_TIMER }
}

pub fn get_hud_text_args_id() -> Option<i32> {
    unsafe { msgs::HUD_TEXT_ARGS }
}

pub fn message_begin(
    msg_dest: i32,
    msg_type: i32,
    origin: Option<[f32; 3]>,
    entity: Option<&EdictPtr>,
) {
    if let Some(api) = meta::ENG_FUNCS.get() {
        if let Some(function) = api.pfnMessageBegin {
            let origin = if let Some(origin) = origin {
                origin.as_ptr()
            } else {
                ptr::null()
            };
            let entity = if let Some(entity) = entity {
                entity.as_ptr()
            } else {
                ptr::null_mut()
            };
            unsafe { function(msg_dest, msg_type, origin, entity) }
        }
    }
}

pub fn message_end() {
    if let Some(api) = meta::ENG_FUNCS.get() {
        if let Some(function) = api.pfnMessageEnd {
            unsafe { function() }
        }
    }
}

pub fn write_byte(value: i32) {
    if let Some(api) = meta::ENG_FUNCS.get() {
        if let Some(function) = api.pfnWriteByte {
            unsafe { function(value) }
        }
    }
}

pub fn write_char(value: i32) {
    if let Some(api) = meta::ENG_FUNCS.get() {
        if let Some(function) = api.pfnWriteChar {
            unsafe { function(value) }
        }
    }
}

pub fn write_short(value: i32) {
    if let Some(api) = meta::ENG_FUNCS.get() {
        if let Some(function) = api.pfnWriteShort {
            unsafe { function(value) }
        }
    }
}

pub fn write_long(value: i32) {
    if let Some(api) = meta::ENG_FUNCS.get() {
        if let Some(function) = api.pfnWriteLong {
            unsafe { function(value) }
        }
    }
}

pub fn write_angle(value: f32) {
    if let Some(api) = meta::ENG_FUNCS.get() {
        if let Some(function) = api.pfnWriteAngle {
            unsafe { function(value) }
        }
    }
}

pub fn write_coord(value: f32) {
    if let Some(api) = meta::ENG_FUNCS.get() {
        if let Some(function) = api.pfnWriteCoord {
            unsafe { function(value) }
        }
    }
}

pub fn write_string(value: &str) {
    if let Some(api) = meta::ENG_FUNCS.get() {
        if let Some(function) = api.pfnWriteString {
            unsafe { function(CString::new(value).unwrap().as_ptr()) }
        }
    }
}

pub fn write_entity(value: i32) {
    if let Some(api) = meta::ENG_FUNCS.get() {
        if let Some(function) = api.pfnWriteEntity {
            unsafe { function(value) }
        }
    }
}
