use std::sync::Once;

use dotenv::dotenv;

static INIT: Once = Once::new();

/// Initializes the environment for testing.
pub fn test_setup() {
    INIT.call_once(|| {
        dotenv().ok();
    });
}
