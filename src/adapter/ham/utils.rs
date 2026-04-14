use std::{ffi::c_void, ptr::null_mut, sync::OnceLock};

use crate::adapter::metamod::{abi, meta, meta_api};

static FIRST_EDICT: OnceLock<meta_api::EdictPtr> = OnceLock::new();
// static mut PEV: usize = 0;

pub fn setup_first_edict() {
    let first_edict = meta::get_ent_by_index(0).unwrap();
    let _ = FIRST_EDICT.set(meta_api::EdictPtr::new(first_edict));

    // let pev = meta_api::vars(first_edict);
    // let private_data = unsafe { *first_edict }.pvPrivateData as *const u8;

    // for i in 0..0xff {
    //     let address = unsafe { private_data.add(i) } as *const *const entvars_t;
    //     if unsafe { std::ptr::read_unaligned(address) } == pev {
    //         unsafe { PEV = i };
    //     }
    // }
}

pub fn get_vtable(this: *mut c_void, size: usize) -> *mut *mut c_void {
    unsafe { *((this.add(size)) as *mut *mut *mut c_void) }
}

fn entvar_to_edict(pev: *mut abi::entvars_t) -> *mut abi::edict_t {
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

pub fn entvars_to_id(pev: *mut abi::entvars_t) -> i32 {
    edict_to_id(entvar_to_edict(pev))
}

fn ref_pdata<T: Copy>(private_data: *mut c_void, offset: usize, element: usize) -> T {
    return unsafe { *((private_data.addr() + offset + (element * size_of::<T>())) as *mut T) };
}

fn get_pdata<T: Copy>(private_data: *mut c_void, offset: usize, element: usize) -> T {
    ref_pdata::<T>(private_data, offset, element)
}

fn cbase_to_entvar(cbase: *mut c_void) -> *mut abi::entvars_t {
    if cbase.is_null() {
        return null_mut();
    }

    get_pdata(cbase, super::offset::PEV, 0)
}

pub fn cbase_to_id(cbase: *mut c_void) -> i32 {
    entvars_to_id(cbase_to_entvar(cbase))
}
