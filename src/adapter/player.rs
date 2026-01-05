use std::sync::Mutex;

use crate::metamod::meta_api;

pub const MAX_PLAYERS: usize = 32;

static PLAYERS: Mutex<Option<[PlayerRaw; 33]>> = Mutex::new(None);

pub fn new() -> Option<[PlayerRaw; 33]> {
    Some([PlayerRaw::EMPTY; 33])
}

pub fn get_mut_player(id: i32) {
    PLAYERS.lock().unwrap().as_mut().map(|p| {
        let _p = &mut p[id as usize];
    });
}

pub fn get_player(id: i32) {
    PLAYERS.lock().unwrap().as_mut().map(|p| {
        let _p = &p[id as usize];
    });
}

pub struct PlayerRaw {
    pub edict: Option<meta_api::EdictPtr>,
    pub connected: bool,
    pub userid: i32,
}

impl PlayerRaw {
    const EMPTY: PlayerRaw = PlayerRaw {
        edict: None,
        connected: false,
        userid: 0,
    };
}
