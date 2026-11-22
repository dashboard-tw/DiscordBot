use reqwest::Client;
pub mod datasets;
use std::sync::OnceLock;

pub struct HttpClient {
    pub client: Client,
}

static HTTP: OnceLock<HttpClient> = OnceLock::new();

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("TWDashboard/1.0")
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    pub fn init() {
        HTTP.get_or_init(Self::new);
    }

    pub fn get() -> &'static Self {
        HTTP.get_or_init(Self::new)
    }
}
