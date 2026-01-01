use crate::adapter::api;

mod entry;

pub fn load() {
    let _ = api::setup_entry(entry::plugin_init, entry::plugin_precache);
}
