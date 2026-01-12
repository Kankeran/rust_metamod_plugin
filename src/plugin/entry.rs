use crate::adapter::api::{self, NumKeys};

pub fn plugin_init() {
    api::console_debug("jest init");
    api::register_client_command(
        String::from("say"),
        Some(String::from("/classes")),
        Box::new(on_classes_command),
    );
    api::register_client_command(
        String::from("say"),
        Some(String::from("/class")),
        Box::new(on_class_command),
    );
    api::register_client_command(String::from("rust_test"), None, Box::new(on_rust_test));
    // api::handle_msg(api::UserMsgs::Damage);
    api::register_client_command(
        String::from("say"),
        Some(String::from("/menu")),
        Box::new(on_menu),
    );
    api::register_client_command(
        String::from("say"),
        Some(String::from("/menu2")),
        Box::new(on_menu2),
    );
    api::register_client_command(
        String::from("say"),
        Some(String::from("/hud")),
        Box::new(on_hud),
    );
    api::register_client_command(
        String::from("say"),
        Some(String::from("/dhud")),
        Box::new(on_dhud),
    );
}

pub fn plugin_precache() {}

fn on_classes_command(id: i32, _arguments: &Vec<String>) -> api::Return {
    api::console_debug("jest client command 'classes'");
    api::client_print(
        Some(id),
        api::PrintMode::PrintChat,
        "YOLO classes chat".to_owned(),
    );
    api::client_print(
        Some(id),
        api::PrintMode::PrintConsole,
        "YOLO classes console\n".to_owned(),
    );
    api::client_print(
        Some(id),
        api::PrintMode::PrintNotify,
        "YOLO classes notify\n".to_owned(),
    );
    api::client_print(
        Some(id),
        api::PrintMode::PrintCenter,
        "YOLO classes center".to_owned(),
    );

    api::Return::Ignored
}

fn on_class_command(_id: i32, _arguments: &Vec<String>) -> api::Return {
    api::console_debug("jest client command 'class'");
    api::client_print(None, api::PrintMode::PrintChat, "ELO class".to_owned());

    api::Return::Supercede
}

fn on_rust_test(id: i32, _arguments: &Vec<String>) -> api::Return {
    api::client_print(Some(id), api::PrintMode::PrintChat, "rust test".to_owned());

    api::Return::Ignored
}

fn on_menu(id: i32, _arguments: &Vec<String>) -> api::Return {
    api::show_menu(
        id,
        Box::new(|_id: i32, menu: &mut api::Menu| {
            menu.add_line("\\yawesome title", NumKeys::KeyNone); // title
            menu.add_line("", NumKeys::KeyNone); // blank
            menu.add_line("\\r1. \\welo", NumKeys::Key1); // item 1
            menu.add_line("", NumKeys::KeyNone); // brank
            menu.add_line("\\d3. elo2", NumKeys::KeyNone); // disabled item 3

            menu.add_line("\\r4. \\yYO", NumKeys::Key4); // item 4

            menu.add_line("\\welo321", NumKeys::KeyNone); // text
            menu.add_line("\\r0. \\yexit", NumKeys::Key0); // exit 0.
        }),
        Box::new(|id, item| {
            api::client_print(
                Some(id),
                api::PrintMode::PrintChat,
                format!("wybrano {}", item),
            );
        }),
    );

    api::Return::Ignored
}

fn on_menu2(id: i32, _arguments: &Vec<String>) -> api::Return {
    api::show_menu(
        id,
        Box::new(|_id: i32, menu: &mut api::Menu| {
            menu.add_line("\\ysuper ciekawe menu\nbez jakichkolwiek opcji\nto menu niesie tylko informacje", NumKeys::KeyNone); // title
            menu.add_line("cokolwiek klikniesz", NumKeys::KeyNone);
            menu.add_line("to menu zniknie", NumKeys::KeyNone);
            menu.add_keys(&[NumKeys::KeyAll]);
        }),
        Box::new(|id, item| {
            api::client_print(
                Some(id),
                api::PrintMode::PrintChat,
                format!("wybrano {}", item),
            );
        }),
    );

    api::Return::Ignored
}

fn on_hud(id: i32, _arguments: &Vec<String>) -> api::Return {
    let style = api::HudStyle {
        point: api::Point { x: 0.02, y: 0.23 },
        effect: 0,
        color1: api::Color {
            r: 0,
            g: 255,
            b: 0,
            a: 0,
        },
        color2: api::Color::default_color2(),
        fade_in_time: 0.0,
        fade_out_time: 0.0,
        hold_time: 1.0,
        fx_time: 0.0,
    };
    api::show_hud_message(
        Some(id),
        style,
        api::HudChannel::One,
        format!("piekny hud :)"),
    );

    api::Return::Ignored
}

fn on_dhud(id: i32, _arguments: &Vec<String>) -> api::Return {
    let style = api::HudStyle {
        point: api::Point { x: 0.2, y: 0.23 },
        effect: 0,
        color1: api::Color {
            r: 100,
            g: 150,
            b: 0,
            a: 0,
        },
        color2: api::Color::default_color2(),
        fade_in_time: 0.0,
        fade_out_time: 0.0,
        hold_time: 4.0,
        fx_time: 0.0,
    };
    api::show_dhud_message(Some(id), style, format!("piekny dhud :)"));

    api::Return::Ignored
}
