use std::{
    alloc::{self, Layout},
    mem::transmute,
    os::raw::c_void,
    ptr::{null, null_mut},
    sync::Mutex,
};

use crate::{
    adapter::{
        action_handler::{TakeDamageCallback, TakeDamageHook, TakeDamageTrampoline},
        metamod::{meta, meta_const},
        trampoline,
    },
    util::log,
};

struct SpecialBotHandler {
    take_damage_register: Option<TakeDamageCallback>,
    vtable: *mut *mut c_void,
}

unsafe impl Sync for SpecialBotHandler {}
unsafe impl Send for SpecialBotHandler {}

static SPECIAL_BOT_HANDLER: Mutex<Option<SpecialBotHandler>> = Mutex::new(None);

static TAKE_DAMAGE_TRAMPOLINE: Mutex<Option<TakeDamageTrampoline>> = Mutex::new(None);

static mut TAKE_DAMAGE_HOOK: *const TakeDamageHook = null();

pub fn set_client_key_value(client_index: i32, _info_buffer: String, key: String, value: String) {
    let mut handler = SPECIAL_BOT_HANDLER.lock().unwrap();

    if let Some(handler) = handler.as_ref() {
        if handler.vtable != null_mut() {
            return;
        }
    }

    let ent = meta::get_ent_by_index(client_index); // to change to player table
    let Some(ent) = ent else {
        return;
    };

    if (unsafe { &*ent }.v.flags & meta_const::FL_FAKECLIENT) != meta_const::FL_FAKECLIENT {
        let auth = meta::get_player_auth_id(ent);
        if let Some(auth) = auth
            && auth != "BOT"
        {
            return;
        }
    }

    if key != "*bot" || value != "1" {
        return;
    }

    let vtable = super::action_handler::get_vtable(unsafe { &*ent }.pvPrivateData, 0x0); // 0x0 for now should be ok

    if let Some(handler) = handler.as_mut() {
        handler.vtable = vtable;
        if let Some(cb) = handler.take_damage_register {
            register_checked_take_damage(cb, vtable);
            handler.take_damage_register = None;
        }
    }

    *handler = Some(SpecialBotHandler {
        take_damage_register: None,
        vtable,
    });
}

pub fn register_special_bot_take_damage(callback: TakeDamageCallback) {
    let mut handler = SPECIAL_BOT_HANDLER.lock().unwrap();

    if let Some(handler) = handler.as_mut() {
        if handler.vtable == null_mut() {
            handler.take_damage_register = Some(callback);
            return;
        }
        register_checked_take_damage(callback, handler.vtable);
        return;
    }

    *handler = Some(SpecialBotHandler {
        take_damage_register: Some(callback),
        vtable: null_mut(),
    });
}

fn register_checked_take_damage(callback: TakeDamageCallback, vtable: *mut *mut c_void) {
    let damage_offset: usize = 12 * 4;

    let vfunction = unsafe { *((vtable.addr() + damage_offset) as *mut *mut c_void) }; // 12 - take damage offset, *4 - i32 size

    // check if is hooked

    let mut damage_trampoline = TAKE_DAMAGE_TRAMPOLINE.lock().unwrap();

    if let Some(tramp) = damage_trampoline.as_ref() {
        if tramp.tramp.as_ptr::<c_void>() == vfunction {
            log::info("BOT powtórzona rejestracja");
            return;
        }
    }

    let layout = Layout::new::<TakeDamageHook>();
    let ptr = unsafe { alloc::alloc(layout) };
    if ptr.is_null() {
        log::error(&format!("cannot allocate memory"));
        return;
    }
    unsafe {
        *(ptr as *mut TakeDamageHook) = TakeDamageHook {
            callback,
            func: vfunction,
            vtable
        };
    }
    unsafe {
        TAKE_DAMAGE_HOOK = ptr as *const TakeDamageHook;
    }

    let tramp = trampoline::create_generic_trampoline(
        true,
        false,
        false,
        4,
        unsafe { TAKE_DAMAGE_HOOK as *const c_void },
        super::action_handler::hook_take_damage as *const c_void,
    );

    *damage_trampoline = Some(TakeDamageTrampoline { tramp: tramp });
    use region::Protection;
    unsafe {
        if let Err(err) = region::protect(
            (vtable.addr() + damage_offset) as *mut *mut c_void,
            4,
            Protection::READ_WRITE,
        ) {
            log::error(&format!("error while registering take damage: {:?}", err));
            return;
        }
    };
    unsafe {
        *((vtable.addr() + damage_offset) as *mut *mut c_void) =
            transmute(damage_trampoline.as_ref().unwrap().tramp.as_ptr::<c_void>());
    }
}
