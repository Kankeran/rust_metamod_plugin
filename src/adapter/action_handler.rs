use std::{
    ffi::CString,
    mem::transmute,
    os::raw::c_void,
    ptr::null_mut,
    sync::{Mutex, OnceLock},
};

use region::{Allocation, Protection};

use crate::{
    adapter::{
        api::Return,
        metamod::{abi, meta, meta_api},
        specialbot, trampoline,
    },
    util::log,
};

pub static FIRST_EDICT: OnceLock<meta_api::EdictPtr> = OnceLock::new();

// win and linux
// pev 4
// base 0x0

// takedamage 12

/// Box::new(|this: i32, inflictor: i32, attacker: i32, damage: f32, damagebits: i32| {/* your code */})
pub type TakeDamageCallback = fn(i32, i32, i32, f32, i32) -> Return;

/// Box::new(|this: i32, inflictor: i32, attacker: i32, damage: f32, damagebits: i32| {/* your code */})
pub type TakeDamageCallbackPost = fn(i32, i32, i32, f32, i32);

#[derive(Clone)]
pub enum HamCallback {
    TakeDamage(TakeDamageCallback),
    TakeDamagePost(TakeDamageCallbackPost),
}

pub struct TakeDamageTrampoline {
    pub tramp: Allocation,
}

unsafe impl Sync for TakeDamageTrampoline {}
unsafe impl Send for TakeDamageTrampoline {}

pub struct TakeDamageHook {
    pub callback_pre: Vec<TakeDamageCallback>,
    pub callback_post: Vec<TakeDamageCallbackPost>,
    pub func: *mut c_void,
    pub vtable: *mut *mut c_void,
}

unsafe impl Sync for TakeDamageHook {}
unsafe impl Send for TakeDamageHook {}

static TAKE_DAMAGE_TRAMPOLINE: Mutex<Option<TakeDamageTrampoline>> = Mutex::new(None);

static TAKE_DAMAGE_HOOK: Mutex<Option<Box<TakeDamageHook>>> = Mutex::new(None);

pub const DAMAGE_OFFSET: usize = 12 * 4;

pub fn get_vtable(this: *mut c_void, size: usize) -> *mut *mut c_void {
    unsafe { *((this.addr() + size) as *mut *mut *mut c_void) }
}

fn entvar_to_edict(pev: *mut super::metamod::abi::entvars_t) -> *mut super::metamod::abi::edict_t {
    if pev.is_null() || unsafe { *pev }.pContainingEntity.is_null() {
        return null_mut();
    }

    unsafe { *pev }.pContainingEntity
}

fn edict_to_id(edict: *mut abi::edict_t) -> i32 {
    if edict.is_null() {
        return -1;
    }

    if let Some(first) = FIRST_EDICT.get() {
        return ((edict.addr() - first.as_ptr().addr()) / size_of::<abi::edict_t>()) as i32;
    }

    -1
}

fn entvars_to_id(pev: *mut super::metamod::abi::entvars_t) -> i32 {
    edict_to_id(entvar_to_edict(pev))
}

fn ref_pdata<T: Copy>(private_data: *mut c_void, offset: usize, element: usize) -> T {
    return unsafe {
        *((private_data.addr() + offset + (element * size_of::<T>() as usize)) as *mut T)
    };
}

fn get_pdata<T: Copy>(private_data: *mut c_void, offset: usize, element: usize) -> T {
    ref_pdata::<T>(private_data, offset, element)
}

fn cbase_to_entvar(cbase: *mut c_void) -> *mut super::metamod::abi::entvars_t {
    if cbase.is_null() {
        return null_mut();
    }

    get_pdata(cbase, 4, 0) // pev offset
}

fn cbase_to_id(cbase: *mut c_void) -> i32 {
    entvars_to_id(cbase_to_entvar(cbase))
}

pub extern "system" fn hook_take_damage(
    hook: *const TakeDamageHook,
    this: *mut c_void,
    inflictor: *mut super::metamod::abi::entvars_t,
    attacker: *mut super::metamod::abi::entvars_t,
    damage: f32,
    damagebits: i32,
) -> i32 {
    let ithis = cbase_to_id(this);
    let iinflictor = entvars_to_id(inflictor);
    let iattacker = entvars_to_id(attacker);
    // log::info(&format!("take damage wywolany2 {ithis} {iinflictor} {iattacker} {damage} {damagebits}"));

    for callback in unsafe { &*hook }.callback_pre.iter() {
        // TODO: handling returns
        callback(ithis, iinflictor, iattacker, damage, damagebits);
    }

    let run: extern "fastcall" fn(
        this: *mut c_void,
        i: i32,
        inflictor: *mut super::metamod::abi::entvars_t,
        attacker: *mut super::metamod::abi::entvars_t,
        damage: f32,
        damagebits: i32,
    ) -> i32 = unsafe { transmute((&*hook).func) };

    let ret = run(this, 0, inflictor, attacker, damage, damagebits);

    for callback in unsafe { &*hook }.callback_post.iter() {
        callback(ithis, iinflictor, iattacker, damage, damagebits);
    }

    return ret;
}

pub fn move_address(pointer: *mut *mut c_void, offset: usize) -> *mut *mut c_void {
    (pointer.addr() + offset) as *mut *mut c_void
}

pub fn register_take_damage(ent_name: &str, callback: HamCallback) {
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

    let vtable = get_vtable(unsafe { *ent }.pvPrivateData, 0x0);
    meta::remove_entity(ent);

    if vtable.is_null() {
        log::error(&format!("Failed to retrieve vtable for \"{ent_name}\""));
        return;
    }

    if ent_name.eq("player") {
        specialbot::register_special_bot_take_damage(callback.clone());
    }

    let vfunction = unsafe { *(move_address(vtable, DAMAGE_OFFSET)) };

    // check if is hooked

    let mut damage_trampoline = TAKE_DAMAGE_TRAMPOLINE.lock().unwrap();

    if let Some(tramp) = damage_trampoline.as_ref() {
        if tramp.tramp.as_ptr::<c_void>() == vfunction {
            log::info("powtórzona rejestracja");
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

    log::info("normalna rejestracja");

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
        hook_take_damage as *const c_void,
    );
    let tramp_address = tramp.as_ptr::<c_void>();
    *(TAKE_DAMAGE_HOOK.lock().unwrap()) = Some(hook);

    *damage_trampoline = Some(TakeDamageTrampoline { tramp: tramp });
    unsafe {
        if let Err(err) = region::protect(
            move_address(vtable, DAMAGE_OFFSET),
            4,
            Protection::READ_WRITE,
        ) {
            log::error(&format!("error while registering take damage: {:?}", err));
            return;
        }
    };
    unsafe {
        *(move_address(vtable, DAMAGE_OFFSET)) = transmute(tramp_address);
    }
}

pub fn free_hooks() {
    if let Some(hook) = TAKE_DAMAGE_HOOK.lock().unwrap().take() {
        let vtable = hook.vtable;
        unsafe {
            if let Err(err) = region::protect(
                move_address(vtable, DAMAGE_OFFSET),
                4,
                Protection::READ_WRITE,
            ) {
                log::error(&format!("error while registering take damage: {:?}", err));
                return;
            }
            *(move_address(vtable, DAMAGE_OFFSET)) = transmute(hook.func);
        }
    }
    *(TAKE_DAMAGE_TRAMPOLINE.lock().unwrap()) = None;
}
