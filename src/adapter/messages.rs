use crate::{
    adapter::{api, convert},
    metamod::{meta_api, meta_const},
};

pub struct TextMessage {
    id: Option<i32>,
    mode: api::PrintMode,
    msg: String,
}

impl TextMessage {
    pub fn new(id: Option<i32>, mode: api::PrintMode, msg: String) -> Self {
        TextMessage { id, mode, msg }
    }

    pub fn send(&self) {
        if let Some(msg_id) = meta_api::get_text_msg_id() {
            let point = self.msg.floor_char_boundary(188);
            let (msg, _) = self.msg.split_at(point);
            let entity = meta_api::get_ent_by_index_option(self.id);
            let msg_dest = if let None = entity {
                meta_const::MSG_BROADCAST
            } else {
                meta_const::MSG_ONE
            };
            meta_api::message_begin(msg_dest, msg_id, None, entity.as_ref());
            meta_api::write_byte(convert::print_mode(&self.mode));
            meta_api::write_string(msg);
            meta_api::message_end();
        }
    }
}

pub struct ShowMenuMessage {
    id: i32,
    keys: i32,
    time: i32, // seconds, -1 = infinity
    buf: String,
}

impl ShowMenuMessage {
    pub fn new(id: i32, keys: i32, time: i32, buf: String) -> Self {
        ShowMenuMessage {
            id,
            keys,
            time,
            buf,
        }
    }

    pub fn send(&self) {
        if let Some(msg_id) = meta_api::get_show_menu_id() {
            let entity = meta_api::get_ent_by_index(self.id);
            if let None = entity {
                return;
            }
            let mut msg;
            let mut next_msg = self.buf.as_str();
            loop {
                let point = next_msg.floor_char_boundary(175);
                (msg, next_msg) = next_msg.split_at(point);
                let has_more = if next_msg.len() > 0 { 1 } else { 0 };
                meta_api::message_begin(meta_const::MSG_ONE, msg_id, None, entity.as_ref());
                meta_api::write_short(self.keys);
                meta_api::write_char(self.time);
                meta_api::write_byte(has_more);
                meta_api::write_string(msg);
                meta_api::message_end();
                if next_msg.len() == 0 {
                    break;
                }
            }
        }
    }
}

pub enum HudChannel {
    // Auto,
    One,
    Two,
    Three,
    Four,
}

impl HudChannel {
    fn to_id(&self) -> i32 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
        }
    }
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn default_color2() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 250,
            a: 0,
        }
    }
}

pub struct HudStyle {
    pub point: Point,
    pub effect: i32, // I don't how this works, should be as enum in my opinion
    pub color1: Color,
    pub color2: Color,
    pub fade_in_time: f32,
    pub fade_out_time: f32,
    pub hold_time: f32,
    pub fx_time: f32,
}

pub struct HudMessage {
    id: Option<i32>,
    style: HudStyle,
    channel: HudChannel,
    message: String,
}

impl HudMessage {
    pub fn new(id: Option<i32>, style: HudStyle, channel: HudChannel, message: String) -> Self {
        HudMessage {
            id,
            style,
            channel,
            message,
        }
    }
    pub fn send(&self) {
        let msg = split_message_for_hud(&self.message);
        let entity = meta_api::get_ent_by_index_option(self.id);
        let msg_dest = if let None = entity {
            meta_const::MSG_BROADCAST
        } else {
            meta_const::MSG_ONE_UNRELIABLE
        };
        meta_api::message_begin(msg_dest, meta_const::SVC_TEMPENTITY, None, entity.as_ref());
        meta_api::write_byte(meta_const::TE_TEXTMESSAGE);
        meta_api::write_byte(self.channel.to_id());
        meta_api::write_short(fixed_signed_16(self.style.point.x, (1 << 13) as f32) as i32);
        meta_api::write_short(fixed_signed_16(self.style.point.y, (1 << 13) as f32) as i32);
        meta_api::write_byte(self.style.effect);
        meta_api::write_byte(self.style.color1.r as i32);
        meta_api::write_byte(self.style.color1.g as i32);
        meta_api::write_byte(self.style.color1.b as i32);
        meta_api::write_byte(self.style.color1.a as i32);
        meta_api::write_byte(self.style.color2.r as i32);
        meta_api::write_byte(self.style.color2.g as i32);
        meta_api::write_byte(self.style.color2.b as i32);
        meta_api::write_byte(self.style.color2.a as i32);
        meta_api::write_short(fixed_unsigned_16(self.style.fade_in_time, (1 << 8) as f32) as i32);
        meta_api::write_short(fixed_unsigned_16(self.style.fade_out_time, (1 << 8) as f32) as i32);
        meta_api::write_short(fixed_unsigned_16(self.style.hold_time, (1 << 8) as f32) as i32);

        if self.style.effect == 2 {
            meta_api::write_short(fixed_unsigned_16(self.style.fx_time, (1 << 8) as f32) as i32);
        }

        meta_api::write_string(&msg);
        meta_api::message_end();
    }
}

pub struct DHudMessage {
    id: Option<i32>,
    style: HudStyle,
    message: String,
}

impl DHudMessage {
    pub fn new(id: Option<i32>, style: HudStyle, message: String) -> Self {
        DHudMessage { id, style, message }
    }

    pub fn send(&self) {
        let point = self.message.floor_char_boundary(127);
        let (msg, _) = self.message.split_at(point);
        let entity = meta_api::get_ent_by_index_option(self.id);
        let msg_dest = if let None = entity {
            meta_const::MSG_BROADCAST
        } else {
            meta_const::MSG_ONE_UNRELIABLE
        };
        meta_api::message_begin(msg_dest, meta_const::SVC_DIRECTOR, None, entity.as_ref());
        meta_api::write_byte((msg.len()+31) as i32);
        meta_api::write_byte(meta_const::DRC_CMD_MESSAGE);
        meta_api::write_byte(self.style.effect);
        meta_api::write_long((self.style.color1.b as i32) + ((self.style.color1.g as i32)<<8) + ((self.style.color1.r as i32)<<16));
        meta_api::write_long(f32_to_i32_raw(self.style.point.x));
        meta_api::write_long(f32_to_i32_raw(self.style.point.y));
        meta_api::write_long(f32_to_i32_raw(self.style.fade_in_time));
        meta_api::write_long(f32_to_i32_raw(self.style.fade_out_time));
        meta_api::write_long(f32_to_i32_raw(self.style.hold_time));
        meta_api::write_long(f32_to_i32_raw(self.style.fx_time));
        meta_api::write_string(msg);
        meta_api::message_end();

    }
}

pub fn fixed_signed_16(value: f32, scale: f32) -> i16 {
    let output = (value * scale) as i32;

    if output > 32767 {
        32767
    } else if output < -32768 {
        -32768
    } else {
        output as i16
    }
}

pub fn fixed_unsigned_16(value: f32, scale: f32) -> u16 {
    let output = (value * scale) as i32;

    if output > 65535 {
        65535
    } else if output < 0 {
        0
    } else {
        output as u16
    }
}

pub fn split_message_for_hud(src: &str) -> String {
    let mut message = String::with_capacity(511);
    let src = src.trim();
    let bytes = src.as_bytes();

    let (mut spacebar_position, mut line_length, mut final_length, mut src_last_char) =
        (0, 0, 0, 0);
    for (i, &item) in bytes.iter().enumerate() {
        final_length += 1;
        line_length += 1;
        if item == b'\n' {
            let pos = i + 1;
            message.push_str(&src[pos - line_length..pos]);
            line_length = 0;
            spacebar_position = 0;
        } else if item == b' ' {
            spacebar_position = i;
        }

        if line_length == 69 {
            if spacebar_position == 0 {
                let pos = i + 1;
                message.push_str(&src[pos - line_length..pos]);
                message.push_str("\n");
                final_length += 1;
                line_length = 0;
            } else {
                message.push_str(&src[i + 1 - line_length..spacebar_position]);
                message.push_str("\n");
                line_length = i - spacebar_position;
                spacebar_position = 0;
            }
        }
        if final_length == 511 {
            src_last_char = i + 1;
            break;
        }
    }

    if src_last_char == 0 {
        src_last_char = src.len();
    } else {
        src_last_char = src.floor_char_boundary(src_last_char);
    }
    message.push_str(&src[src_last_char - line_length..src_last_char]);

    message
}

fn f32_to_i32_raw(value :f32) -> i32 {
    unsafe{*((&raw const value) as *const i32)}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_message_long_word() {
        let src = "asdflkjhasdfkshdfkshfdaklhsdfkjahdkflhsakdfgreighsrughsrkghdkjhgfkhsglkjf";
        let expected =
            "asdflkjhasdfkshdfkshfdaklhsdfkjahdkflhsakdfgreighsrughsrkghdkjhgfkhsg\nlkjf"
                .to_owned();

        assert_eq!(split_message_for_hud(src), expected);
    }

    #[test]
    fn test_split_message_with_spaces() {
        let src = "asdflkjhasdfkshdfkshfdaklhsdfkjahdkflhsakd fgreighsrughsrkghdkjhgfkhsglkjf";
        let expected =
            "asdflkjhasdfkshdfkshfdaklhsdfkjahdkflhsakd\nfgreighsrughsrkghdkjhgfkhsglkjf"
                .to_owned();

        assert_eq!(split_message_for_hud(src), expected);
    }

    #[test]
    fn test_split_message_with_linebreaks() {
        let src = "asdflkjhasdfkshdfkshfdaklhsdfkjahdkflhsakd\nfgreighsrughsrkghdkjhgfkhsglkjf";
        let expected =
            "asdflkjhasdfkshdfkshfdaklhsdfkjahdkflhsakd\nfgreighsrughsrkghdkjhgfkhsglkjf"
                .to_owned();

        assert_eq!(split_message_for_hud(src), expected);
    }

    #[test]
    fn test_split_message_mixed() {
        let src = "asdflkjhasdfkshdfkshfdaklhsdfkjahdkflhsakd\nfgreighsrughsrkghdkjhgfkhsglkjfksjghsklahgkla jhrgkrhagklhrdklghskdhgksdhrgkjhdsgkge5hyye5kjhb43jhb54j35hb4fgsdgsjkgnhseg";
        let expected = "asdflkjhasdfkshdfkshfdaklhsdfkjahdkflhsakd\nfgreighsrughsrkghdkjhgfkhsglkjfksjghsklahgkla\njhrgkrhagklhrdklghskdhgksdhrgkjhdsgkge5hyye5kjhb43jhb54j35hb4fgsdgsjk\ngnhseg".to_owned();

        assert_eq!(split_message_for_hud(src), expected);
    }

    #[test]
    fn test_split_message_too_long() {
        let src = "asdflkjhasdfkshdfkshfdaklhsdfkjahdkflhsakd\nfgreighsrughsrkghdkjhgfkhsglkjfksjghsklahgkla jhrgkrhagklhrdklghskdhgksdhrgkjhdsgkge5hyye5kjhb43jhb54j35hb4fgsdgsjkgnhsegasdflkjhasdfkshdfkshfdaklhsdfkjahdkflhsakd\nfgreighsrughsrkghdkjhgfkhsglkjfksjghsklahgkla jhrgkrhagklhrdklghskdhgksdhrgkjhdsgkge5hyye5kjhb43jhb54j35hb4fgsdgsjkgnhsegasdflkjhasdfkshdfkshfdaklhsdfkjahdkflhsakd\nfgreighsrughsrkghdkjhgfkhsglkjfksjghsklahgkla jhrgkrhagklhrdklghskdhgksdhrgkjhdsgkge5hyye5kjhb43jhb54j35hb4fgsdgsjkgnhsegjhrgkrhagklhrdklghskdhgksdhrgkjhdsgkge5hyye5kjhb43jhb54j35hb4fgsdgsjkgnhseg";
        let expected = 511;

        assert_eq!(split_message_for_hud(src).len(), expected);
    }

    #[test]
    fn test_fixed_signed_16() {
        assert_eq!(fixed_signed_16(0.31, (1 << 13) as f32), 2539);
        assert_eq!(fixed_signed_16(0.32, (1 << 13) as f32), 2621);
        assert_eq!(fixed_signed_16(0.02, (1 << 13) as f32), 163);
        assert_eq!(fixed_signed_16(0.23, (1 << 13) as f32), 1884);
    }

    #[test]
    fn test_fixed_unsigned_16() {
        assert_eq!(fixed_unsigned_16(0.31, (1 << 8) as f32), 79);
        assert_eq!(fixed_unsigned_16(0.32, (1 << 8) as f32), 81);
        assert_eq!(fixed_unsigned_16(0.02, (1 << 8) as f32), 5);
        assert_eq!(fixed_unsigned_16(0.23, (1 << 8) as f32), 58);
    }

    #[test]
    fn test_float_to_i32() {
        let  a:f32 = 0.12;
        assert_eq!(f32_to_i32_raw(a), 1039516303);
    }
}
