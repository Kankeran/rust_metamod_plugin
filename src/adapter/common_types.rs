pub enum Return {
    Ignored,
    Handled,
    Override,
    Supercede,
    DeferSupercede,
}

#[derive(Clone, Copy)]
pub enum BlockMode {
    BlockNone,
    BlockOne,
    BlockAll,
}

pub enum PrintMode {
    PrintNotify,
    PrintConsole,
    PrintChat,
    PrintCenter,
}

#[derive(Debug)]
pub enum UserMsgs {
    TextMsg,
    BarTime,
    CurWeapon,
    Damage,
    DeathMsg,
    TeamInfo,
    WeaponList,
    MOTD,
    ServerName,
    Health,
    Battery,
    ShowMenu,
    SendAudio,
    AmmoX,
    ScoreInfo,
    VguiMenu,
    AmmoPickup,
    WeapPickup,
    ResetHud,
    RoundTime,
    SayText,
    InitHud,
    SvcTempEntity,
}
