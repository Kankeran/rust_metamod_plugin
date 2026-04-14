use std::ffi::c_void;

use crate::adapter::{
    api::Return,
    ham::{
        types::{OverrideTakeDamage, TakeDamageFunctions},
        utils,
    },
    metamod::abi,
};

pub extern "system" fn hook_take_damage(
    hook: *const TakeDamageFunctions,
    this: *mut c_void,
    inflictor: *mut abi::entvars_t,
    attacker: *mut abi::entvars_t,
    mut damage: f32,
    mut damagebits: i32,
) -> i32 {
    let hook_ref = unsafe { &*hook };
    let mut over_ret = None;
    let ithis = utils::cbase_to_id(this);
    let iinflictor = utils::entvars_to_id(inflictor);
    let iattacker = utils::entvars_to_id(attacker);
    // log::info(&format!("take damage wywolany2 {ithis} {iinflictor} {iattacker} {damage} {damagebits}"));

    for callback in hook_ref.callback_pre.iter() {
        // TODO: handling returns
        let res = callback(ithis, iinflictor, iattacker, damage, damagebits);
        if let Return::Supercede = res {
            return 0;
        }
        if let Return::Override(values) = res {
            for value in values.into_iter() {
                match value {
                    OverrideTakeDamage::Damage(dmg) => {
                        damage = dmg;
                    }
                    OverrideTakeDamage::Damagebits(bits) => {
                        damagebits = bits;
                    }
                    OverrideTakeDamage::Return(ret) => {
                        over_ret = Some(ret);
                    }
                }
            }
        }
    }

    let ret = hook_ref.call_func(this, inflictor, attacker, damage, damagebits);

    for callback in hook_ref.callback_post.iter() {
        callback(ithis, iinflictor, iattacker, damage, damagebits);
    }

    if let Some(ret) = over_ret {
        return ret;
    }
    return ret;
}
