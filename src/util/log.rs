use std::fs::OpenOptions;
use std::io::Write;

pub fn info(msg: &str) {
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("rust_plugin.log")
    {
        let _ = writeln!(f, "{msg}");
    }
}

pub fn error(msg: &str) {
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("rust_plugin_error.log")
    {
        let _ = writeln!(f, "{msg}");
    }
}
