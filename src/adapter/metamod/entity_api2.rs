//! [abi::META_FUNCTIONS::pfnGetEntityAPI2] and [abi::META_FUNCTIONS::pfnGetEntityAPI2_Post] implementations

use super::{abi, adapter, entry, meta, meta_const, msgs};
use crate::util::log;
use cstr::cstr;
use std::{cmp::max, ptr::null_mut, sync::LazyLock};

static mut INITIALIZED: bool = false;

static FUNCTION_TABLE: LazyLock<abi::DLL_FUNCTIONS> = LazyLock::new(|| abi::DLL_FUNCTIONS {
    pfnSpawn: Some(spawn),
    pfnClientConnect: Some(client_connect),
    pfnClientCommand: Some(client_command),
    pfnServerActivate: Some(server_activate),
    ..Default::default()
});

pub extern "C" fn get_api(
    function_table: *mut abi::DLL_FUNCTIONS,
    interface_version: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    log::debug("get_entity_api2");
    if function_table.is_null() {
        log::error("metamod function table is null");
        return 0;
    }
    if unsafe { *interface_version } != abi::INTERFACE_VERSION as i32 {
        log::error("half life interface version mismatch");
        return 0;
    }

    unsafe {
        *function_table = *FUNCTION_TABLE;
    }

    1
}

extern "C" fn spawn(_entity: *mut abi::edict_t) -> i32 {
    if unsafe { INITIALIZED } {
        meta::set_result(meta_const::RESULT_IGNORED);
        return 0;
    }
    entry::meta_setup();
    crate::adapter::setup_first_edict();

    unsafe {
        INITIALIZED = true;
    }
    meta::set_result(meta_const::RESULT_IGNORED);
    0
}

extern "C" fn client_connect(
    entity: *mut abi::edict_t,
    name: *const ::std::os::raw::c_char,
    _address: *const ::std::os::raw::c_char,
    _reject_reason: *mut ::std::os::raw::c_char,
) -> abi::qboolean {
    if let (Some(player_id), player_name) =
        (meta::get_ent_index(entity), meta::c_char_to_string(name))
    {
        adapter::console_debug(&format!(
            "player with id {} and name {} is connecting",
            player_id, player_name
        ));
    }
    meta::set_result(meta_const::RESULT_IGNORED);

    1
}

extern "C" fn client_command(entity: *mut abi::edict_t) {
    if let (Some(player_id), Some(api)) = (meta::get_ent_index(entity), meta::ENG_FUNCS.get()) {
        if let (Some(argv_fn), Some(argc_fn)) = (api.pfnCmd_Argv, api.pfnCmd_Argc) {
            let args_num = max(unsafe { argc_fn() }, 2); // need at least cmd and first argument, if first argument not exist then empty string is provided by engine
            let mut arguments: Vec<String> = Vec::with_capacity(args_num as usize);
            for n in 0..args_num {
                let arg = unsafe { argv_fn(n) };
                if !arg.is_null() {
                    let str_arg = meta::c_char_to_string(arg);
                    arguments.push(str_arg);
                }
            }

            meta::set_result(entry::client_command(player_id, arguments));
            return;
        }
    }
    meta::set_result(meta_const::RESULT_IGNORED);
}

extern "C" fn server_activate(
    _entity_list: *mut abi::edict_t,
    _entity_count: ::std::os::raw::c_int,
    _client_max: ::std::os::raw::c_int,
) {
    if let None = unsafe { msgs::TEXT_MSG } {
        unsafe { msgs::TEXT_MSG = meta::get_user_msg_id(cstr!("TextMsg"), null_mut()) };
    } else if let None = unsafe { msgs::BAR_TIME } {
        unsafe { msgs::BAR_TIME = meta::get_user_msg_id(cstr!("BarTime"), null_mut()) };
    } else if let None = unsafe { msgs::CUR_WEAPON } {
        unsafe { msgs::CUR_WEAPON = meta::get_user_msg_id(cstr!("CurWeapon"), null_mut()) };
    } else if let None = unsafe { msgs::DAMAGE } {
        unsafe { msgs::DAMAGE = meta::get_user_msg_id(cstr!("Damage"), null_mut()) };
    } else if let None = unsafe { msgs::DEATH_MSG } {
        unsafe { msgs::DEATH_MSG = meta::get_user_msg_id(cstr!("DeathMsg"), null_mut()) };
    } else if let None = unsafe { msgs::TEAM_INFO } {
        unsafe { msgs::TEAM_INFO = meta::get_user_msg_id(cstr!("TeamInfo"), null_mut()) };
    } else if let None = unsafe { msgs::WEAPON_LIST } {
        unsafe { msgs::WEAPON_LIST = meta::get_user_msg_id(cstr!("WeaponList"), null_mut()) };
    } else if let None = unsafe { msgs::MOTD } {
        unsafe { msgs::MOTD = meta::get_user_msg_id(cstr!("MOTD"), null_mut()) };
    } else if let None = unsafe { msgs::SERVER_NAME } {
        unsafe { msgs::SERVER_NAME = meta::get_user_msg_id(cstr!("ServerName"), null_mut()) };
    } else if let None = unsafe { msgs::HEALTH } {
        unsafe { msgs::HEALTH = meta::get_user_msg_id(cstr!("Health"), null_mut()) };
    } else if let None = unsafe { msgs::BATTERY } {
        unsafe { msgs::BATTERY = meta::get_user_msg_id(cstr!("Battery"), null_mut()) };
    } else if let None = unsafe { msgs::SHOW_MENU } {
        unsafe { msgs::SHOW_MENU = meta::get_user_msg_id(cstr!("ShowMenu"), null_mut()) };
    } else if let None = unsafe { msgs::SEND_AUDIO } {
        unsafe { msgs::SEND_AUDIO = meta::get_user_msg_id(cstr!("SendAudio"), null_mut()) };
    } else if let None = unsafe { msgs::AMMO_X } {
        unsafe { msgs::AMMO_X = meta::get_user_msg_id(cstr!("AmmoX"), null_mut()) };
    } else if let None = unsafe { msgs::SCORE_INFO } {
        unsafe { msgs::SCORE_INFO = meta::get_user_msg_id(cstr!("ScoreInfo"), null_mut()) };
    } else if let None = unsafe { msgs::VGUI_MENU } {
        unsafe { msgs::VGUI_MENU = meta::get_user_msg_id(cstr!("VGUIMenu"), null_mut()) };
    } else if let None = unsafe { msgs::AMMO_PICKUP } {
        unsafe { msgs::AMMO_PICKUP = meta::get_user_msg_id(cstr!("AmmoPickup"), null_mut()) };
    } else if let None = unsafe { msgs::WEAP_PICKUP } {
        unsafe { msgs::WEAP_PICKUP = meta::get_user_msg_id(cstr!("WeapPickup"), null_mut()) };
    } else if let None = unsafe { msgs::RESET_HUD } {
        unsafe { msgs::RESET_HUD = meta::get_user_msg_id(cstr!("ResetHUD"), null_mut()) };
    } else if let None = unsafe { msgs::ROUND_TIME } {
        unsafe { msgs::ROUND_TIME = meta::get_user_msg_id(cstr!("RoundTime"), null_mut()) };
    } else if let None = unsafe { msgs::SAY_TEXT } {
        unsafe { msgs::SAY_TEXT = meta::get_user_msg_id(cstr!("SayText"), null_mut()) };
    } else if let None = unsafe { msgs::INIT_HUD } {
        unsafe { msgs::INIT_HUD = meta::get_user_msg_id(cstr!("InitHUD"), null_mut()) };
    } else if let None = unsafe { msgs::VOICE_MASK } {
        unsafe { msgs::VOICE_MASK = meta::get_user_msg_id(cstr!("VoiceMask"), null_mut()) };
    } else if let None = unsafe { msgs::REQ_STATE } {
        unsafe { msgs::REQ_STATE = meta::get_user_msg_id(cstr!("ReqState"), null_mut()) };
    } else if let None = unsafe { msgs::GEIGER } {
        unsafe { msgs::GEIGER = meta::get_user_msg_id(cstr!("Geiger"), null_mut()) };
    } else if let None = unsafe { msgs::FLASHLIGHT } {
        unsafe { msgs::FLASHLIGHT = meta::get_user_msg_id(cstr!("Flashlight"), null_mut()) };
    } else if let None = unsafe { msgs::FLASH_BAT } {
        unsafe { msgs::FLASH_BAT = meta::get_user_msg_id(cstr!("FlashBat"), null_mut()) };
    } else if let None = unsafe { msgs::TRAIN } {
        unsafe { msgs::TRAIN = meta::get_user_msg_id(cstr!("Train"), null_mut()) };
    } else if let None = unsafe { msgs::HUD_TEXT_PRO } {
        unsafe { msgs::HUD_TEXT_PRO = meta::get_user_msg_id(cstr!("HudTextPro"), null_mut()) };
    } else if let None = unsafe { msgs::HUD_TEXT } {
        unsafe { msgs::HUD_TEXT = meta::get_user_msg_id(cstr!("HudText"), null_mut()) };
    } else if let None = unsafe { msgs::VIEW_MODE } {
        unsafe { msgs::VIEW_MODE = meta::get_user_msg_id(cstr!("ViewMode"), null_mut()) };
    } else if let None = unsafe { msgs::GAME_TITLE } {
        unsafe { msgs::GAME_TITLE = meta::get_user_msg_id(cstr!("GameTitle"), null_mut()) };
    } else if let None = unsafe { msgs::SCORE_ATTRIB } {
        unsafe { msgs::SCORE_ATTRIB = meta::get_user_msg_id(cstr!("ScoreAttrib"), null_mut()) };
    } else if let None = unsafe { msgs::TEAM_SCORE } {
        unsafe { msgs::TEAM_SCORE = meta::get_user_msg_id(cstr!("TeamScore"), null_mut()) };
    } else if let None = unsafe { msgs::GAME_MODE } {
        unsafe { msgs::GAME_MODE = meta::get_user_msg_id(cstr!("GameMode"), null_mut()) };
    } else if let None = unsafe { msgs::ITEM_PICKUP } {
        unsafe { msgs::ITEM_PICKUP = meta::get_user_msg_id(cstr!("ItemPickup"), null_mut()) };
    } else if let None = unsafe { msgs::HIDE_WEAPON } {
        unsafe { msgs::HIDE_WEAPON = meta::get_user_msg_id(cstr!("HideWeapon"), null_mut()) };
    } else if let None = unsafe { msgs::SET_FOV } {
        unsafe { msgs::SET_FOV = meta::get_user_msg_id(cstr!("SetFOV"), null_mut()) };
    } else if let None = unsafe { msgs::SCREEN_SHAKE } {
        unsafe { msgs::SCREEN_SHAKE = meta::get_user_msg_id(cstr!("ScreenShake"), null_mut()) };
    } else if let None = unsafe { msgs::SCREEN_FADE } {
        unsafe { msgs::SCREEN_FADE = meta::get_user_msg_id(cstr!("ScreenFade"), null_mut()) };
    } else if let None = unsafe { msgs::MONEY } {
        unsafe { msgs::MONEY = meta::get_user_msg_id(cstr!("Money"), null_mut()) };
    } else if let None = unsafe { msgs::ARMOR_TYPE } {
        unsafe { msgs::ARMOR_TYPE = meta::get_user_msg_id(cstr!("ArmorType"), null_mut()) };
    } else if let None = unsafe { msgs::BLINK_ACCT } {
        unsafe { msgs::BLINK_ACCT = meta::get_user_msg_id(cstr!("BlinkAcct"), null_mut()) };
    } else if let None = unsafe { msgs::STATUS_VALUE } {
        unsafe { msgs::STATUS_VALUE = meta::get_user_msg_id(cstr!("StatusValue"), null_mut()) };
    } else if let None = unsafe { msgs::STATUS_TEXT } {
        unsafe { msgs::STATUS_TEXT = meta::get_user_msg_id(cstr!("StatusText"), null_mut()) };
    } else if let None = unsafe { msgs::STATUS_ICON } {
        unsafe { msgs::STATUS_ICON = meta::get_user_msg_id(cstr!("StatusIcon"), null_mut()) };
    } else if let None = unsafe { msgs::RELOAD_SOUND } {
        unsafe { msgs::RELOAD_SOUND = meta::get_user_msg_id(cstr!("ReloadSound"), null_mut()) };
    } else if let None = unsafe { msgs::CROSSHAIR } {
        unsafe { msgs::CROSSHAIR = meta::get_user_msg_id(cstr!("Crosshair"), null_mut()) };
    } else if let None = unsafe { msgs::NVG_TOGGLE } {
        unsafe { msgs::NVG_TOGGLE = meta::get_user_msg_id(cstr!("NVGToggle"), null_mut()) };
    } else if let None = unsafe { msgs::RADAR } {
        unsafe { msgs::RADAR = meta::get_user_msg_id(cstr!("Radar"), null_mut()) };
    } else if let None = unsafe { msgs::SPECTATOR } {
        unsafe { msgs::SPECTATOR = meta::get_user_msg_id(cstr!("Spectator"), null_mut()) };
    } else if let None = unsafe { msgs::TUTOR_TEXT } {
        unsafe { msgs::TUTOR_TEXT = meta::get_user_msg_id(cstr!("TutorText"), null_mut()) };
    } else if let None = unsafe { msgs::TUTOR_LINE } {
        unsafe { msgs::TUTOR_LINE = meta::get_user_msg_id(cstr!("TutorLine"), null_mut()) };
    } else if let None = unsafe { msgs::TUTOR_STATE } {
        unsafe { msgs::TUTOR_STATE = meta::get_user_msg_id(cstr!("TutorState"), null_mut()) };
    } else if let None = unsafe { msgs::TUTOR_CLOSE } {
        unsafe { msgs::TUTOR_CLOSE = meta::get_user_msg_id(cstr!("TutorClose"), null_mut()) };
    } else if let None = unsafe { msgs::ALLOW_SPEC } {
        unsafe { msgs::ALLOW_SPEC = meta::get_user_msg_id(cstr!("AllowSpec"), null_mut()) };
    } else if let None = unsafe { msgs::BOMB_DROP } {
        unsafe { msgs::BOMB_DROP = meta::get_user_msg_id(cstr!("BombDrop"), null_mut()) };
    } else if let None = unsafe { msgs::BOMB_PICKUP } {
        unsafe { msgs::BOMB_PICKUP = meta::get_user_msg_id(cstr!("BombPickup"), null_mut()) };
    } else if let None = unsafe { msgs::CL_CORPSE } {
        unsafe { msgs::CL_CORPSE = meta::get_user_msg_id(cstr!("ClCorpse"), null_mut()) };
    } else if let None = unsafe { msgs::HOSTAGE_POS } {
        unsafe { msgs::HOSTAGE_POS = meta::get_user_msg_id(cstr!("HostagePos"), null_mut()) };
    } else if let None = unsafe { msgs::HOSTAGE_K } {
        unsafe { msgs::HOSTAGE_K = meta::get_user_msg_id(cstr!("HostageK"), null_mut()) };
    } else if let None = unsafe { msgs::HLTV } {
        unsafe { msgs::HLTV = meta::get_user_msg_id(cstr!("HLTV"), null_mut()) };
    } else if let None = unsafe { msgs::SPEC_HEALTH } {
        unsafe { msgs::SPEC_HEALTH = meta::get_user_msg_id(cstr!("SpecHealth"), null_mut()) };
    } else if let None = unsafe { msgs::FORCE_CAM } {
        unsafe { msgs::FORCE_CAM = meta::get_user_msg_id(cstr!("ForceCam"), null_mut()) };
    } else if let None = unsafe { msgs::AD_STOP } {
        unsafe { msgs::AD_STOP = meta::get_user_msg_id(cstr!("ADStop"), null_mut()) };
    } else if let None = unsafe { msgs::RECEIVE_W } {
        unsafe { msgs::RECEIVE_W = meta::get_user_msg_id(cstr!("ReceiveW"), null_mut()) };
    } else if let None = unsafe { msgs::CZ_CAREER } {
        unsafe { msgs::CZ_CAREER = meta::get_user_msg_id(cstr!("CZCareer"), null_mut()) };
    } else if let None = unsafe { msgs::CZ_CAREER_HUD } {
        unsafe { msgs::CZ_CAREER_HUD = meta::get_user_msg_id(cstr!("CZCareerHUD"), null_mut()) };
    } else if let None = unsafe { msgs::SHADOW_IDX } {
        unsafe { msgs::SHADOW_IDX = meta::get_user_msg_id(cstr!("ShadowIdx"), null_mut()) };
    } else if let None = unsafe { msgs::TASK_TIME } {
        unsafe { msgs::TASK_TIME = meta::get_user_msg_id(cstr!("TaskTime"), null_mut()) };
    } else if let None = unsafe { msgs::SCENARIO } {
        unsafe { msgs::SCENARIO = meta::get_user_msg_id(cstr!("Scenario"), null_mut()) };
    } else if let None = unsafe { msgs::BOT_VOICE } {
        unsafe { msgs::BOT_VOICE = meta::get_user_msg_id(cstr!("BotVoice"), null_mut()) };
    } else if let None = unsafe { msgs::BUY_CLOSE } {
        unsafe { msgs::BUY_CLOSE = meta::get_user_msg_id(cstr!("BuyClose"), null_mut()) };
    } else if let None = unsafe { msgs::SPEC_HEALTH2 } {
        unsafe { msgs::SPEC_HEALTH2 = meta::get_user_msg_id(cstr!("SpecHealth2"), null_mut()) };
    } else if let None = unsafe { msgs::BAR_TIME2 } {
        unsafe { msgs::BAR_TIME2 = meta::get_user_msg_id(cstr!("BarTime2"), null_mut()) };
    } else if let None = unsafe { msgs::ITEM_STATUS } {
        unsafe { msgs::ITEM_STATUS = meta::get_user_msg_id(cstr!("ItemStatus"), null_mut()) };
    } else if let None = unsafe { msgs::LOCATION } {
        unsafe { msgs::LOCATION = meta::get_user_msg_id(cstr!("Location"), null_mut()) };
    } else if let None = unsafe { msgs::BOT_PROGRESS } {
        unsafe { msgs::BOT_PROGRESS = meta::get_user_msg_id(cstr!("BotProgress"), null_mut()) };
    } else if let None = unsafe { msgs::BRASS } {
        unsafe { msgs::BRASS = meta::get_user_msg_id(cstr!("Brass"), null_mut()) };
    } else if let None = unsafe { msgs::FOG } {
        unsafe { msgs::FOG = meta::get_user_msg_id(cstr!("Fog"), null_mut()) };
    } else if let None = unsafe { msgs::SHOW_TIMER } {
        unsafe { msgs::SHOW_TIMER = meta::get_user_msg_id(cstr!("ShowTimer"), null_mut()) };
    } else if let None = unsafe { msgs::HUD_TEXT_ARGS } {
        unsafe { msgs::HUD_TEXT_ARGS = meta::get_user_msg_id(cstr!("HudTextArgs"), null_mut()) };
    }
}

static FUNCTION_TABLE_POST: LazyLock<abi::DLL_FUNCTIONS> = LazyLock::new(|| abi::DLL_FUNCTIONS {
    pfnClientPutInServer: Some(client_put_in_server_post),
    pfnServerActivate: Some(server_activate_post),
    pfnServerDeactivate: Some(server_deactivate_post),
    ..Default::default()
});

pub extern "C" fn get_api_post(
    function_table: *mut abi::DLL_FUNCTIONS,
    interface_version: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    log::debug("get_entity_api2_post");
    if function_table.is_null() {
        log::error("[POST] metamod function table is null");
        return 0;
    }
    if unsafe { *interface_version } != abi::INTERFACE_VERSION as i32 {
        log::error("[POST] half life interface version mismatch");
        return 0;
    }

    unsafe {
        *function_table = *FUNCTION_TABLE_POST;
    }

    1
}

extern "C" fn server_activate_post(
    _entity_list: *mut abi::edict_t,
    _entity_count: ::std::os::raw::c_int,
    _client_max: ::std::os::raw::c_int,
) {
    entry::meta_init();
}

extern "C" fn client_put_in_server_post(entity: *mut abi::edict_t) {
    if let Some(player_id) = meta::get_ent_index(entity) {
        adapter::console_debug(&format!("player with id {} joined", player_id));
    }
}

extern "C" fn server_deactivate_post() {
    crate::adapter::entry::free_data();
}
