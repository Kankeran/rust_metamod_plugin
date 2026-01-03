use std::{ptr, sync::LazyLock};

use crate::metamod::abi;

static mut PLAYERS: LazyLock<Players> = LazyLock::new(Players::new); // super unsafe btw

pub struct Players {
    slots: Vec<Option<PlayerRaw>>,
}

pub fn init_players() -> &'static mut Players {
    unsafe { &mut *PLAYERS }
}

pub fn get_player(id: i32) -> Option<&'static mut PlayerRaw> {
    unsafe { (*PLAYERS).get_mut(id) }
}

// pub fn players() -> &'static Players {
//     PLAYERS.get().unwrap()
// }

type PlayerId = i32;
pub const MAX_PLAYERS: usize = 32;

impl Players {
    pub fn new() -> Self {
        Players {
            slots: Vec::with_capacity(33), // indeks 0 ignorowany
        }
    }

    pub fn get(&self, id: PlayerId) -> Option<&PlayerRaw> {
        self.slots.get(id as usize)?.as_ref()
    }

    pub fn get_mut(&mut self, id: PlayerId) -> Option<&mut PlayerRaw> {
        self.slots.get_mut(id as usize)?.as_mut()
    }

    pub fn connect(&mut self, userid: PlayerId, edict: *mut abi::edict_t) {
        self.slots[userid as usize] = Some(PlayerRaw {
            userid,
            edict: EdictPtr(edict),
            connected: true,
        });
    }

    pub fn disconnect(&mut self, id: PlayerId) {
        self.slots[id as usize] = None;
    }
}

pub struct EdictPtr(*mut abi::edict_t);

unsafe impl Sync for EdictPtr {}
unsafe impl Send for EdictPtr {}

pub struct PlayerRaw {
    pub edict: EdictPtr,
    pub connected: bool,
    pub userid: i32,
}

impl PlayerRaw {
    const EMPTY: PlayerRaw = PlayerRaw {
        edict: EdictPtr(ptr::null_mut()),
        connected: false,
        userid: 0,
    };
}
