use std::fs::OpenOptions;
use std::io::Write;

pub fn debug(msg: &str) {
    #[cfg(debug_assertions)]
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("rust_plugin_debug.log")
    {
        let _ = writeln!(f, "{msg}");
    }
}

pub fn info(msg: &str) {
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("rust_plugin_info.log")
    {
        let _ = writeln!(f, "{msg}");
    }
}

pub fn error(msg: &str) {
    #[cfg(debug_assertions)]
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("rust_plugin_error.log")
    {
        let _ = writeln!(f, "{msg}");
    }
}
