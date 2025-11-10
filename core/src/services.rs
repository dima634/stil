use std::sync::Once;

use crate::workspaces::workspace_registry;

pub fn global_init() {
    INIT.call_once(|| {
        let _ = workspace_registry();
    });
}

const INIT: Once = Once::new();
