use std::{
    ffi::{CString, c_void},
    mem::transmute,
    ptr::null_mut,
    sync::Mutex,
};

use region::Protection;

use crate::{
    adapter::{
        ham::{
            exports, offset, trampoline,
            types::{HamCallback, HamFunction, Hook, SpecialBotHandler, TakeDamageFunctions},
            utils,
        },
        metamod::{meta, meta_const},
    },
    util::log,
};

static HOOKS: Mutex<Vec<Hook>> = Mutex::new(Vec::new());
static SPECIAL_BOT_HANDLER: Mutex<Option<SpecialBotHandler>> = Mutex::new(None);

pub fn free_hooks() {
    *(HOOKS.lock().unwrap()) = Vec::new();
    *(SPECIAL_BOT_HANDLER.lock().unwrap()) = None;
}

pub fn register_ham(ent_name: &str, callback: HamCallback) {
    let Some(ent) = meta::create_entity() else {
        return;
    };
    _ = meta::call_game_entity(
        CString::new(ent_name).unwrap().as_c_str(),
        &mut (unsafe { *ent }).v,
    );

    if unsafe { *ent }.pvPrivateData.is_null() {
        meta::remove_entity(ent);
        log::error(&format!("Failed to retrieve classtype for \"{ent_name}\""));
        return;
    }

    let vtable = utils::get_vtable(unsafe { *ent }.pvPrivateData, 0x0);
    meta::remove_entity(ent);

    if vtable.is_null() {
        log::error(&format!("Failed to retrieve vtable for \"{ent_name}\""));
        return;
    }

    if ent_name.eq("player") {
        register_special_bot_ham(callback.clone());
    }

    let ham_offset = get_offset_by_ham(&callback);
    let mut hooks = HOOKS.lock().unwrap();
    let vfunction = unsafe { *vtable.add(ham_offset) };

    for hook in hooks.iter_mut() {
        if hook.tramp.as_ptr::<c_void>() == vfunction {
            hook.functions.append_callback(&callback);
            return;
        }
    }

    let ham_functions = create_hook_by_ham(&callback, vfunction);

    let tramp = trampoline::create_generic_trampoline(
        true,
        false,
        false,
        get_param_count(&callback),
        (&raw const (*ham_functions)) as *const c_void,
        get_export_by_ham(&callback),
    );
    let tramp_address = tramp.as_ptr::<c_void>();
    hooks.push(Hook {
        functions: ham_functions,
        vtable,
        tramp,
    });
    unsafe {
        if let Err(err) = region::protect(vtable.add(ham_offset), 4, Protection::READ_WRITE) {
            log::error(&format!("error while registering take damage: {:?}", err));
            return;
        }
        *vtable.add(ham_offset) = transmute(tramp_address);
    }
}

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

    let vtable = utils::get_vtable(unsafe { &*ent }.pvPrivateData, 0x0); // 0x0 for now should be ok

    if let Some(handler) = handler.take() {
        for cb in handler.callbacks.into_iter() {
            register_checked_ham(cb, vtable);
        }
    }

    *handler = Some(SpecialBotHandler {
        callbacks: Vec::new(),
        vtable,
    });
}

fn register_special_bot_ham(callback: HamCallback) {
    let mut handler = SPECIAL_BOT_HANDLER.lock().unwrap();

    if let Some(handler) = handler.as_mut() {
        if handler.vtable == null_mut() {
            handler.callbacks.push(callback);
            return;
        }
        register_checked_ham(callback, handler.vtable);
        return;
    }

    *handler = Some(SpecialBotHandler {
        callbacks: vec![callback],
        vtable: null_mut(),
    });
}

fn register_checked_ham(callback: HamCallback, vtable: *mut *mut c_void) {
    let ham_offset = get_offset_by_ham(&callback);
    let mut hooks = HOOKS.lock().unwrap();
    let vfunction = unsafe { *vtable.add(ham_offset) };

    for hook in hooks.iter_mut() {
        if hook.tramp.as_ptr::<c_void>() == vfunction {
            hook.functions.append_callback(&callback);
            return;
        }
    }

    let ham_functions = create_hook_by_ham(&callback, vfunction);

    let tramp = trampoline::create_generic_trampoline(
        true,
        false,
        false,
        get_param_count(&callback),
        (&raw const (*ham_functions)) as *const c_void,
        get_export_by_ham(&callback),
    );
    let tramp_address = tramp.as_ptr::<c_void>();
    hooks.push(Hook {
        functions: ham_functions,
        vtable,
        tramp,
    });
    unsafe {
        if let Err(err) = region::protect(vtable.add(ham_offset), 4, Protection::READ_WRITE) {
            log::error(&format!("error while registering take damage: {:?}", err));
            return;
        }
        *vtable.add(ham_offset) = transmute(tramp_address);
    }
}

fn get_offset_by_ham(callback: &HamCallback) -> usize {
    if let HamCallback::TakeDamage(_) = callback {
        return offset::TAKE_DAMAGE;
    }

    if let HamCallback::TakeDamagePost(_) = callback {
        return offset::TAKE_DAMAGE;
    }

    return 0;
}

fn create_hook_by_ham(callback: &HamCallback, func: *mut c_void) -> Box<dyn HamFunction> {
    if let HamCallback::TakeDamage(_) | HamCallback::TakeDamagePost(_) = callback {
        return Box::new(TakeDamageFunctions::new(callback, func));
    }

    panic!("create_hook_by_ham");
}

fn get_export_by_ham(callback: &HamCallback) -> *const c_void {
    if let HamCallback::TakeDamage(_) | HamCallback::TakeDamagePost(_) = callback {
        return exports::hook_take_damage as *const c_void;
    }

    panic!("get_export_by_ham");
}

fn get_param_count(callback: &HamCallback) -> usize {
    if let HamCallback::TakeDamage(_) | HamCallback::TakeDamagePost(_) = callback {
        return 4;
    }

    panic!("get_param_count");
}
