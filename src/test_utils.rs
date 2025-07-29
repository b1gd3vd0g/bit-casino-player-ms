use std::sync::Once;

use dotenv::dotenv;

#[allow(dead_code)]
static INIT: Once = Once::new(); // Not dead

/// Initializes the environment for testing.
#[allow(dead_code)] // This is not dead code, just used in tests.
pub fn test_setup() {
    INIT.call_once(|| {
        dotenv().ok();
    });
}
