pub const PRINT_NOTIFY: i32 = 1;
pub const PRINT_CONSOLE: i32 = 2;
pub const PRINT_CHAT: i32 = 3;
pub const PRINT_CENTER: i32 = 4;

pub const RESULT_IGNORED: i32 = 1;
pub const RESULT_HANDLED: i32 = 2;
pub const RESULT_OVERRIDE: i32 = 3;
pub const RESULT_SUPERCEDE: i32 = 4;

pub const MSG_BROADCAST: i32 = 0;
pub const MSG_ONE: i32 = 1;
pub const MSG_ALL: i32 = 2;
pub const MSG_INIT: i32 = 3;
pub const MSG_PVS: i32 = 4;
pub const MSG_PAS: i32 = 5;
pub const MSG_PVS_R: i32 = 6;
pub const MSG_PAS_R: i32 = 7;
pub const MSG_ONE_UNRELIABLE: i32 = 8;
pub const MSG_SPEC: i32 = 9;

pub const MAX_LEVEL_CONNECTIONS: i32 = 16;
pub const MAX_QPATH: i32 = 64;

pub const MAX_WEAPONS: i32 = 32;

// hardcoded messages

pub const SVC_BAD: i32 = 0;
pub const SVC_NOP: i32 = 1;
pub const SVC_DISCONNECT: i32 = 2;
pub const SVC_EVENT: i32 = 3;
pub const SVC_VERSION: i32 = 4;
pub const SVC_SETVIEW: i32 = 5;
pub const SVC_SOUND: i32 = 6;
pub const SVC_TIME: i32 = 7;
pub const SVC_PRINT: i32 = 8;
pub const SVC_STUFFTEXT: i32 = 9;
pub const SVC_SETANGLE: i32 = 10;
pub const SVC_SERVERINFO: i32 = 11;
pub const SVC_LIGHTSTYLE: i32 = 12;
pub const SVC_UPDATEUSERINFO: i32 = 13;
pub const SVC_DELTADESCRIPTION: i32 = 14;
pub const SVC_CLIENTDATA: i32 = 15;
pub const SVC_STOPSOUND: i32 = 16;
pub const SVC_PINGS: i32 = 17;
pub const SVC_PARTICLE: i32 = 18;
pub const SVC_DAMAGE: i32 = 19;
pub const SVC_SPAWNSTATIC: i32 = 20;
pub const SVC_EVENT_RELIABLE: i32 = 21;
pub const SVC_SPAWNBASELINE: i32 = 22;
pub const SVC_TEMPENTITY: i32 = 23;
pub const SVC_SETPAUSE: i32 = 24;
pub const SVC_SIGNONNUM: i32 = 25;
pub const SVC_CENTERPRINT: i32 = 26;
pub const SVC_KILLEDMONSTER: i32 = 27;
pub const SVC_FOUNDSECRET: i32 = 28;
pub const SVC_SPAWNSTATICSOUND: i32 = 29;
pub const SVC_INTERMISSION: i32 = 30;
pub const SVC_FINALE: i32 = 31;
pub const SVC_CDTRACK: i32 = 32;
pub const SVC_RESTORE: i32 = 33;
pub const SVC_CUTSCENE: i32 = 34;
pub const SVC_WEAPONANIM: i32 = 35;
pub const SVC_DECALNAME: i32 = 36;
pub const SVC_ROOMTYPE: i32 = 37;
pub const SVC_ADDANGLE: i32 = 38;
pub const SVC_NEWUSERMSG: i32 = 39;
pub const SVC_PACKETENTITIES: i32 = 40;
pub const SVC_DELTAPACKETENTITIES: i32 = 41;
pub const SVC_CHOKE: i32 = 42;
pub const SVC_RESOURCELIST: i32 = 43;
pub const SVC_NEWMOVEVARS: i32 = 44;
pub const SVC_RESOURCEREQUEST: i32 = 45;
pub const SVC_CUSTOMIZATION: i32 = 46;
pub const SVC_CROSSHAIRANGLE: i32 = 47;
pub const SVC_SOUNDFADE: i32 = 48;
pub const SVC_FILETXFERFAILED: i32 = 49;
pub const SVC_HLTV: i32 = 50;
pub const SVC_DIRECTOR: i32 = 51;
pub const SVC_VOICEINIT: i32 = 52;
pub const SVC_VOICEDATA: i32 = 53;
pub const SVC_SENDEXTRAINFO: i32 = 54;
pub const SVC_TIMESCALE: i32 = 55;
pub const SVC_RESOURCELOCATION: i32 = 56;
pub const SVC_SENDCVARVALUE: i32 = 57;
pub const SVC_SENDCVARVALUE2: i32 = 58;

pub const TE_TEXTMESSAGE: i32 = 29;

/**
 * From hltv.h from the HLSDK, these are used in conjunction with SVC_DIRECTOR
 * sub commands of svc_director
 */
pub const DRC_CMD_NONE: i32 = 0; // NULL director command
pub const DRC_CMD_START: i32 = 1; // start director mode
pub const DRC_CMD_EVENT: i32 = 2; // informs about director command
pub const DRC_CMD_MODE: i32 = 3; // switches camera modes
pub const DRC_CMD_CAMERA: i32 = 4; // sets camera registers
pub const DRC_CMD_TIMESCALE: i32 = 5; // sets time scale
pub const DRC_CMD_MESSAGE: i32 = 6; // send HUD centerprint
pub const DRC_CMD_SOUND: i32 = 7; // plays a particular sound
pub const DRC_CMD_STATUS: i32 = 8; // status info about broadcast
pub const DRC_CMD_BANNER: i32 = 9; // banner file name for HLTV gui
pub const DRC_CMD_FADE: i32 = 10; // send screen fade command
pub const DRC_CMD_SHAKE: i32 = 11; // send screen shake command
pub const DRC_CMD_STUFFTEXT: i32 = 12; // like the normal svc_stufftext but as director command
