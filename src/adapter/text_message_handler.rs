use std::sync::Mutex;

use super::api::PrintMode::PrintChat;
use super::api::{PrintMode, Return};
use super::common_types::UserMsgs;
use super::messages::TextMessage;
use super::messages_handler::RawMessage;
use super::messages_handler::{self, MessageValue};
use super::metamod::meta_api;

type TextMessageCallback = Box<dyn Fn(&TextMessage) -> Return<Vec<OverrideTextMessage>> + Send + Sync + 'static>;
static CALLBACKS: Mutex<Vec<TextMessageCallback>> = Mutex::new(Vec::new());

pub enum OverrideTextMessage {
    Id(Option<i32>),
    Mode(PrintMode),
    Msg(String),
}

impl RawMessage {
    pub fn to_text_message(&self) -> TextMessage {
        let mut mode = PrintChat;
        let mut msg = String::new();
        for (i, data) in self.data.iter().enumerate() {
            match data {
                MessageValue::Byte(val) if i == 0 => mode = PrintMode::from_i32(*val),
                MessageValue::String(val) if i == 1 => msg = val.to_owned(),
                _ => {}
            }
        }
        TextMessage::new(meta_api::get_ent_index(self.ent.as_ref()), mode, msg)
    }
}

pub fn register_text_message(callback: TextMessageCallback) {
    CALLBACKS.lock().unwrap().push(callback);
    messages_handler::register_msg_handler(UserMsgs::TextMsg);
}

pub fn handle_text_message(msg: &mut TextMessage) -> Return<()> {
    let mut result = Return::Ignored;
    if let Ok(callbacks) = CALLBACKS.lock() {
        for callback in callbacks.iter() {
            let res = callback(&msg);
            if let Return::Supercede = res {
                return Return::Supercede;
            }
            if result.lt(&res) {
                result = res.into_empty()
            }
            if let Return::Override(values) = res {
                for value in values.into_iter() {
                    match value {
                        OverrideTextMessage::Id(id) => {msg.id = id;}
                        OverrideTextMessage::Mode(mode) => {msg.mode = mode;}
                        OverrideTextMessage::Msg(message) => {msg.msg = message;}
                    }
                }
            }
        }
    }

    result
}
