//! An internal client builder for configuring reqwest options for all Blizzard API requests.

use std::time::Duration;

use super::reqwest::InternalHttpClient;

/// A fluent builder for constructing HTTP client instances with configurable options.
#[derive(Debug)]
pub struct InternalHttpClientBuilder<'a> {
    /// Internal HTTP instance to be constructed with the configurable options.
    pub client: Option<InternalHttpClient>,
    /// Base URL of the Blizzard API targeted by the client, depends on locale.
    pub base_url: Option<&'a str>,
    /// Timeout duration configurable by the consuming client.
    pub timeout: Option<Duration>,
}

impl<'a> Default for InternalHttpClientBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> InternalHttpClientBuilder<'a> {
    /// Constructs a new client builder instance with no default options configured.
    pub fn new() -> Self {
        Self {
            client: None,
            base_url: None,
            timeout: None,
        }
    }
}
