mod plugin;
mod process;
mod registry;
mod trigger;

pub use fh_ipc::Usage;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

// Re-export
pub use plugin::Plugin;
pub use process::ProcessPlugin;
pub use registry::Registry;
pub use trigger::Trigger;

pub type UsageCache = Arc<Mutex<HashMap<String, Vec<Usage>>>>;
