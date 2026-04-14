use std::{ffi::c_void, mem::transmute, rc::Rc};

use region::{Allocation, Protection};

use crate::{adapter::metamod::abi, util::log};

use super::Return;

pub struct Trampoline(pub Allocation);

unsafe impl Sync for Trampoline {}
unsafe impl Send for Trampoline {}

pub type SpawnCallback = Rc<dyn Fn(i32) -> Return<()>>; // Void_Void
pub type SpawnCallbackPost = Rc<dyn Fn(i32)>;
pub type PrecacheCallback = Rc<dyn Fn(i32) -> Return<()>>; // Void_Void
pub type PrecacheCallbackPost = Rc<dyn Fn(i32)>;
pub type KeyValueCallback = Rc<dyn Fn(i32, i32) -> Return<()>>; // Void_Int
pub type KeyValueCallbackPost = Rc<dyn Fn(i32, i32)>;
pub type ObjectCapsCallback = Rc<dyn Fn(i32) -> Return<OverrideObjectCaps>>; // Int_Void
pub type ObjectCapsCallbackPost = Rc<dyn Fn(i32)>;
pub type ActiveCallback = Rc<dyn Fn(i32) -> Return<()>>; // Void_Void
pub type ActiveCallbackPost = Rc<dyn Fn(i32)>;
pub type SetObjectCollisionboxCallback = Rc<dyn Fn(i32) -> Return<()>>; // Void_Void
pub type SetObjectCollisionboxCallbackPost = Rc<dyn Fn(i32)>;
pub type ClassifyCallback = Rc<dyn Fn(i32) -> Return<OverrideClassify>>; // Int_Void
pub type ClassifyCallbackPost = Rc<dyn Fn(i32)>;
pub type DeathNoticeCallback = Rc<dyn Fn(i32, i32) -> Return<()>>; // Void_Entvar
pub type DeathNoticeCallbackPost = Rc<dyn Fn(i32, i32)>;
pub type TraceAttackCallback = Rc<dyn Fn(i32, i32, f32, &[f32], i32 /*trace */, i32) -> Return<()>>; // Void_Entvar_Float_Vector_Trace_Int
pub type TraceAttackCallbackPost = Rc<dyn Fn(i32, i32, f32, &[f32], i32 /*trace */, i32)>;
/// Rc::new(|this: i32, inflictor: i32, attacker: i32, damage: f32, damagebits: i32| -> api::Return<Vec<OverrideTakeDamage>> {/* your code */})
pub type TakeDamageCallback =
    Rc<dyn Fn(i32, i32, i32, f32, i32) -> Return<Vec<OverrideTakeDamage>>>;
/// Rc::new(|this: i32, inflictor: i32, attacker: i32, damage: f32, damagebits: i32| {/* your code */})
pub type TakeDamageCallbackPost = Rc<dyn Fn(i32, i32, i32, f32, i32)>;

pub enum OverrideObjectCaps {
    Return(i32),
}

pub enum OverrideClassify {
    Return(i32),
}

pub enum OverrideTakeDamage {
    Damage(f32),
    Damagebits(i32),
    Return(i32),
}

#[derive(Clone)]
pub enum HamCallback {
    Spawn(SpawnCallback),
    SpawnPost(SpawnCallbackPost),
    Precache(PrecacheCallback),
    PrecachePost(PrecacheCallbackPost),
    KeyValue(KeyValueCallback),
    KeyValuePost(KeyValueCallbackPost),
    ObjectCaps(ObjectCapsCallback),
    ObjectCapsPost(ObjectCapsCallbackPost),
    Active(ActiveCallback),
    ActivePost(ActiveCallbackPost),
    SetObjectCollisionbox(SetObjectCollisionboxCallback),
    SetObjectCollisionboxPost(SetObjectCollisionboxCallbackPost),
    Classify(ClassifyCallback),
    ClassifyPost(ClassifyCallbackPost),
    DeathNotice(DeathNoticeCallback),
    DeathNoticePost(DeathNoticeCallbackPost),
    TraceAttack(TraceAttackCallback),
    TraceAttackPost(TraceAttackCallbackPost),
    TakeDamage(TakeDamageCallback),
    TakeDamagePost(TakeDamageCallbackPost),
}

pub trait HamFunction {
    fn func(&self) -> *mut c_void;
    fn append_callback(&mut self, callback: &HamCallback);
}

pub struct TakeDamageFunctions {
    pub callback_pre: Vec<TakeDamageCallback>,
    pub callback_post: Vec<TakeDamageCallbackPost>,
    pub func: *mut c_void,
}

impl TakeDamageFunctions {
    pub fn new(callback: &HamCallback, func: *mut c_void) -> Self {
        let mut instance = Self {
            callback_pre: Vec::new(),
            callback_post: Vec::new(),
            func,
        };
        instance.append_callback(callback);
        instance
    }

    pub fn call_func(
        &self,
        this: *mut c_void,
        inflictor: *mut abi::entvars_t,
        attacker: *mut abi::entvars_t,
        damage: f32,
        damagebits: i32,
    ) -> i32 {
        let run: extern "fastcall" fn(
            this: *mut c_void,
            i: i32,
            inflictor: *mut abi::entvars_t,
            attacker: *mut abi::entvars_t,
            damage: f32,
            damagebits: i32,
        ) -> i32 = unsafe { transmute(self.func) };

        run(this, 0, inflictor, attacker, damage, damagebits)
    }
}

impl HamFunction for TakeDamageFunctions {
    fn func(&self) -> *mut c_void {
        self.func
    }

    fn append_callback(&mut self, callback: &HamCallback) {
        if let HamCallback::TakeDamage(cb) = callback {
            self.callback_pre.push(cb.clone());
        }
        if let HamCallback::TakeDamagePost(cb) = callback {
            self.callback_post.push(cb.clone());
        }
    }
}

pub struct Hook {
    pub functions: Box<dyn HamFunction>,
    pub vtable: *mut *mut c_void,
    pub tramp: Allocation,
}

// unsafe impl Sync for Hook {}
unsafe impl Send for Hook {}

impl Drop for Hook {
    fn drop(&mut self) {
        let vtable_with_offset = unsafe { self.vtable.add(super::offset::TAKE_DAMAGE) };
        unsafe {
            if let Err(err) = region::protect(vtable_with_offset, 4, Protection::READ_WRITE) {
                log::error(&format!("error while droping take damage: {:?}", err));
                return;
            }
            *vtable_with_offset = transmute(self.functions.func());
        }
    }
}

pub struct SpecialBotHandler {
    pub callbacks: Vec<HamCallback>,
    pub vtable: *mut *mut c_void,
}

// unsafe impl Sync for SpecialBotHandler {}
unsafe impl Send for SpecialBotHandler {}
