use crate::adapter::messages::ShowMenuMessage;

/// Box::new(|id: i32, mut menu: Menu| {/* your code */})
pub type MenuBuilder = Box<dyn Fn(i32, &mut Menu) + Send + Sync + 'static>;
/// Box::new(|id: i32, item: i32| {/* your code */})
pub type MenuCallback = Box<dyn Fn(i32, i32) + Send + Sync + 'static>;

static mut PLAYERS_KEYS: [i32; 33] = [0; 33];
static mut PLAYERS_MENU_CALLBACK: [Option<MenuCallback>; 33] = [const { None }; 33];

pub fn show_menu(id: i32, menu_builder: MenuBuilder, callback: MenuCallback) {
    let mut menu = Menu::new();

    menu_builder(id, &mut menu);
    unsafe {
        PLAYERS_MENU_CALLBACK[id as usize] = Some(callback);
        PLAYERS_KEYS[id as usize] = menu.keys;
    }

    menu.show(id);
}

pub struct Menu {
    keys: i32,
    time: i32,
    buf: String,
}

impl Menu {
    fn new() -> Self {
        Menu {
            keys: 0,
            time: -1,
            buf: String::with_capacity(511),
        }
    }

    pub fn add_line(&mut self, text: &str, key: NumKeys) {
        let mut len = self.buf.len();
        if len >= 511 {
            return;
        }
        len += text.len();
        if len >= 511 {
            let point = text.floor_char_boundary(len - 511);
            let (text, _) = text.split_at(point);
            self.buf.push_str(text);
        } else {
            self.buf.push_str(text);
            self.buf.push('\n');
        }

        self.keys |= key.to_bits();
    }

    pub fn add_keys(&mut self, keys: &[NumKeys]) {
        for key in keys.iter() {
            self.keys |= key.to_bits();
        }
    }

    pub fn set_time(&mut self, time: i32) {
        if time < 0 {
            self.time = -1;
        } else {
            self.time = time
        }
    }

    fn show(self, id: i32) {
        ShowMenuMessage::new(id, self.keys, self.time, self.buf).send();
    }
}

pub enum NumKeys {
    KeyNone,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    KeyAll,
}

impl NumKeys {
    fn to_bits(&self) -> i32 {
        match self {
            NumKeys::KeyNone => 0,
            NumKeys::Key1 => 1 << 0,
            NumKeys::Key2 => 1 << 1,
            NumKeys::Key3 => 1 << 2,
            NumKeys::Key4 => 1 << 3,
            NumKeys::Key5 => 1 << 4,
            NumKeys::Key6 => 1 << 5,
            NumKeys::Key7 => 1 << 6,
            NumKeys::Key8 => 1 << 7,
            NumKeys::Key9 => 1 << 8,
            NumKeys::Key0 => 1 << 9,
            NumKeys::KeyAll => 1023,
        }
    }
}

pub fn handle_menu_select(id: i32, command: &str, argument: &str) {
    if !command.eq("menuselect") {
        return;
    }

    if let Ok(item) = argument.parse::<i32>() {
        let key = 1 << (item - 1);
        if (key & unsafe { PLAYERS_KEYS[id as usize] }) > 0 {
            if let Some(func) = unsafe { PLAYERS_MENU_CALLBACK[id as usize].take() } {
                unsafe {
                    PLAYERS_KEYS[id as usize] = 0;
                }
                func(id, item);
            }
        }
    }
}
