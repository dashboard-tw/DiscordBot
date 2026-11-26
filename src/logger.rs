
use std::sync::OnceLock;
use tracing_subscriber::EnvFilter;

static LOGGER: OnceLock<()> = OnceLock::new();

pub fn init() {
    LOGGER.get_or_init(|| {
        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| EnvFilter::new("info"))
            )
            .with_target(true)
            .with_thread_ids(true)
            .init();
    });
}

pub use tracing::{debug, error, info, trace, warn};
