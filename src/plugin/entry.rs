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
}

pub fn plugin_precache() {}

fn on_classes_command(id: i32, _arguments: &Vec<String>) -> api::Return {
    api::console_debug("jest client command 'classes'");
    api::client_print(id, api::PrintMode::PrintChat, "YOLO classes chat");
    api::client_print(id, api::PrintMode::PrintConsole, "YOLO classes console\n");
    api::client_print(id, api::PrintMode::PrintNotify, "YOLO classes notify\n");
    api::client_print(id, api::PrintMode::PrintCenter, "YOLO classes center");

    api::Return::Ignored
}

fn on_class_command(id: i32, _arguments: &Vec<String>) -> api::Return {
    api::console_debug("jest client command 'class'");
    api::client_print(id, api::PrintMode::PrintChat, "ELO class");

    api::Return::Supercede
}
