use std::{
    alloc::{self, Layout},
    mem::transmute,
    os::raw::c_void,
    ptr::null_mut,
    sync::Mutex,
};

use region::Protection;

use crate::{
    adapter::{
        action_handler::{self, HamCallback, TakeDamageHook, TakeDamageTrampoline},
        metamod::{meta, meta_const},
        trampoline,
    },
    util::log,
};

struct SpecialBotHandler {
    take_damage_register: Vec<HamCallback>,
    vtable: *mut *mut c_void,
}

unsafe impl Sync for SpecialBotHandler {}
unsafe impl Send for SpecialBotHandler {}

static SPECIAL_BOT_HANDLER: Mutex<Option<SpecialBotHandler>> = Mutex::new(None);

static TAKE_DAMAGE_TRAMPOLINE: Mutex<Option<TakeDamageTrampoline>> = Mutex::new(None);

static TAKE_DAMAGE_HOOK: Mutex<Option<Box<TakeDamageHook>>> = Mutex::new(None);

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

    if let Some(handler) = handler.take() {
        for cb in handler.take_damage_register.into_iter() {
            register_checked_take_damage(cb, vtable);
        }
    }

    *handler = Some(SpecialBotHandler {
        take_damage_register: Vec::new(),
        vtable,
    });
}

pub fn register_special_bot_take_damage(callback: HamCallback) {
    let mut handler = SPECIAL_BOT_HANDLER.lock().unwrap();

    if let Some(handler) = handler.as_mut() {
        if handler.vtable == null_mut() {
            handler.take_damage_register.push(callback);
            return;
        }
        register_checked_take_damage(callback, handler.vtable);
        return;
    }

    *handler = Some(SpecialBotHandler {
        take_damage_register: vec![callback],
        vtable: null_mut(),
    });
}

fn register_checked_take_damage(callback: HamCallback, vtable: *mut *mut c_void) {
    let damage_offset: usize = 12 * 4;

    let vfunction = unsafe { *((vtable.addr() + damage_offset) as *mut *mut c_void) }; // 12 - take damage offset, *4 - i32 size

    // check if is hooked

    let mut damage_trampoline = TAKE_DAMAGE_TRAMPOLINE.lock().unwrap();

    if let Some(tramp) = damage_trampoline.as_ref() {
        if tramp.tramp.as_ptr::<c_void>() == vfunction {
            log::info("BOT powtórzona rejestracja");
            match callback {
                HamCallback::TakeDamage(callback) => {
                    if let Some(hook) = TAKE_DAMAGE_HOOK.lock().unwrap().as_mut() {
                        hook.callback_pre.push(callback);
                    }
                }
                HamCallback::TakeDamagePost(callback) => {
                    if let Some(hook) = TAKE_DAMAGE_HOOK.lock().unwrap().as_mut() {
                        hook.callback_post.push(callback);
                    }
                }
            };
            return;
        }
    }

    let layout = Layout::new::<TakeDamageHook>();
    let ptr = unsafe { alloc::alloc(layout) };
    if ptr.is_null() {
        log::error(&format!("cannot allocate memory"));
        return;
    }
    let hook = match callback {
        HamCallback::TakeDamage(callback) => Box::new(TakeDamageHook {
            callback_pre: vec![callback],
            callback_post: Vec::new(),
            func: vfunction,
            vtable,
        }),
        HamCallback::TakeDamagePost(callback) => Box::new(TakeDamageHook {
            callback_pre: Vec::new(),
            callback_post: vec![callback],
            func: vfunction,
            vtable,
        }),
    };

    let tramp = trampoline::create_generic_trampoline(
        true,
        false,
        false,
        4,
        (&raw const (*hook)) as *const c_void,
        super::action_handler::hook_take_damage as *const c_void,
    );
    let tramp_address = tramp.as_ptr::<c_void>();
    *(TAKE_DAMAGE_HOOK.lock().unwrap()) = Some(hook);

    *damage_trampoline = Some(TakeDamageTrampoline { tramp: tramp });
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
        *(action_handler::move_address(vtable, action_handler::DAMAGE_OFFSET)) = transmute(tramp_address);
    }
}

pub fn free_hooks() {
    if let Some(hook) = TAKE_DAMAGE_HOOK.lock().unwrap().take() {
        let vtable = hook.vtable;
        unsafe {
            if let Err(err) = region::protect(
                action_handler::move_address(vtable, action_handler::DAMAGE_OFFSET),
                4,
                Protection::READ_WRITE,
            ) {
                log::error(&format!("error while registering take damage: {:?}", err));
                return;
            }
            *(action_handler::move_address(vtable, action_handler::DAMAGE_OFFSET)) = transmute(hook.func);
        }
    }
    *(TAKE_DAMAGE_TRAMPOLINE.lock().unwrap()) = None;
    *(SPECIAL_BOT_HANDLER.lock().unwrap()) = None;
}
