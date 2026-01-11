use crate::adapter::api;

pub fn plugin_init() {
    api::console_debug("jest init");
    api::register_client_command(
        String::from("say"),
        Some(String::from("/classes")),
        on_classes_command,
    );
    api::register_client_command(
        String::from("say"),
        Some(String::from("/class")),
        on_class_command,
    );
    api::register_client_command(String::from("rust_test"), None, on_rust_test);
    // api::handle_msg(api::UserMsgs::Damage);
    api::register_client_command(String::from("say"), Some(String::from("/menu")), on_menu);
    api::register_client_command(String::from("say"), Some(String::from("/hud")), on_hud);
    api::register_client_command(String::from("say"), Some(String::from("/dhud")), on_dhud);
}

pub fn plugin_precache() {}

fn on_classes_command(id: i32, _arguments: &Vec<String>) -> api::Return {
    api::console_debug("jest client command 'classes'");
    api::client_print(Some(id), api::PrintMode::PrintChat, "YOLO classes chat");
    api::client_print(
        Some(id),
        api::PrintMode::PrintConsole,
        "YOLO classes console\n",
    );
    api::client_print(
        Some(id),
        api::PrintMode::PrintNotify,
        "YOLO classes notify\n",
    );
    api::client_print(Some(id), api::PrintMode::PrintCenter, "YOLO classes center");

    api::Return::Ignored
}

fn on_class_command(_id: i32, _arguments: &Vec<String>) -> api::Return {
    api::console_debug("jest client command 'class'");
    api::client_print(None, api::PrintMode::PrintChat, "ELO class");

    api::Return::Supercede
}

fn on_rust_test(id: i32, _arguments: &Vec<String>) -> api::Return {
    api::client_print(Some(id), api::PrintMode::PrintChat, "rust test");

    api::Return::Ignored
}

fn on_menu(id: i32, _arguments: &Vec<String>) -> api::Return {
    api::show_menu(
        id,
        1023,
        10,
        format!(
            "Elo\\r3\\y2\\w0\ngskhegskeghskjdghsd\nfrkshfekfsuehfksuehfukshefs\n\\r5.\\w hahahahah\nsdkjghserhgrshgsrhgrhgkldhrgkljhdsrgkhdrkglshdrkslghskldrhgkdsjhrkg\nkjehfjsekgfjesgfjksegfjhsgefjsgejfgsejfgskjefgjshdgfjshbfejhbsef\nkjehkslfhejhfgsejhgfsjegfjsegfjskdgfjds"
        ),
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
