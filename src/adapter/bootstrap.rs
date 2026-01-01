use crate::{adapter::entry, metamod::meta, plugin};

pub fn load() {
    let _ = meta::setup_entry(entry::adapter_init, entry::adapter_setup, entry::client_command);

    plugin::load();
}
