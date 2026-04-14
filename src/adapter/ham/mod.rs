mod api;
mod exports;
mod offset;
mod trampoline;
mod types;
mod utils;

use super::api::Return;

pub use types::HamCallback;
pub use types::OverrideTakeDamage;

pub use api::free_hooks;
pub use api::register_ham;
pub use api::set_client_key_value;
pub use utils::setup_first_edict;
