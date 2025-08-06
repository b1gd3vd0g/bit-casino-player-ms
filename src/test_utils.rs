use std::{env, sync::Once};

#[allow(dead_code)]
static INIT: Once = Once::new(); // Not dead

/// Initializes the environment for testing.
#[allow(dead_code)] // This is not dead code, just used in tests.
pub fn test_setup() {
    INIT.call_once(|| {
        env::set_var("JWT_SECRET", "supersecretvalue");
    });
}
