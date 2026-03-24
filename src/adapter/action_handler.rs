use std::{
    alloc::{self, Layout},
    ffi::CString,
    mem::transmute,
    os::raw::c_void,
    ptr::{null, null_mut},
    sync::{Mutex, OnceLock},
};

use region::Allocation;

use crate::{
    adapter::{
        api::Return, metamod::{abi, meta, meta_api}, specialbot, trampoline
    },
    util::log,
};

pub static FIRST_EDICT: OnceLock<meta_api::EdictPtr> = OnceLock::new();

// win and linux
// pev 4
// base 0x0

// takedamage 12

/// Box::new(|id: i32, item: i32| {/* your code */})
pub type TakeDamageCallback = fn(i32, i32, i32, f32, i32) -> Return;

pub struct TakeDamageTrampoline {
    pub tramp: Allocation,
}

unsafe impl Sync for TakeDamageTrampoline {}
unsafe impl Send for TakeDamageTrampoline {}

pub struct TakeDamageHook {
    pub callback: TakeDamageCallback,
    pub func: *mut c_void,
}

unsafe impl Sync for TakeDamageHook {}
unsafe impl Send for TakeDamageHook {}

static TAKE_DAMAGE_TRAMPOLINE: Mutex<Option<TakeDamageTrampoline>> = Mutex::new(None);

static mut TAKE_DAMAGE_HOOK: *const TakeDamageHook = null();

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
        log::info(&format!("edict {} | first edict {} | sizeof edict {}",edict.addr() as i32, first.as_ptr().addr() as i32, size_of::<abi::edict_t>() as i32));
        return ((edict.addr() - first.as_ptr().addr())/size_of::<abi::edict_t>()) as i32;
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
    log::info(&format!("take damage wywolany2 {ithis} {iinflictor} {iattacker} {damage} {damagebits}"));

    (unsafe { &*hook }.callback)(ithis, iinflictor, iattacker, damage, damagebits);

    let run: extern "fastcall" fn(
        this: *mut c_void,
        i: i32,
        inflictor: *mut super::metamod::abi::entvars_t,
        attacker: *mut super::metamod::abi::entvars_t,
        damage: f32,
        damagebits: i32,
    ) -> i32 = unsafe { transmute((&*hook).func) };

    let ret = run(this, 0, inflictor, attacker, damage, damagebits);

    return ret;
}

pub fn register_take_damage(ent_name: &str, callback: TakeDamageCallback) {
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

    specialbot::register_special_bot_take_damage(callback);

    let damage_offset:usize = 12*4 ;

    let vfunction = unsafe { *((vtable.addr() + damage_offset) as *mut *mut c_void) }; // 12 - take damage offset, *4 - i32 size

    // check if is hooked

    let mut damage_trampoline = TAKE_DAMAGE_TRAMPOLINE.lock().unwrap();

    if let Some(tramp) = damage_trampoline.as_ref() {
        if tramp.tramp.as_ptr::<c_void>() == vfunction {
            log::info("powtórzona rejestracja");
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
        hook_take_damage as *const c_void,
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
