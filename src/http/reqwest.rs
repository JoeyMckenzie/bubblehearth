//! An internal HTTP client for all interacting with all Blizzard APIs used across all crates.

use std::time::Duration;

use crate::regionality::AccountRegion;

const DEFAULT_TIMEOUT_SECONDS: u8 = 5;

/// A configurable HTTP client used across crates for connecting to all Blizzard APIs.
#[derive(Debug, Clone)]
pub struct InternalHttpClient {
    /// Internal reqwest client for making HTTP requests.
    pub http: reqwest::Client,
    /// Region all API calls should target.
    regionality: AccountRegion,
}

impl InternalHttpClient {
    /// Constructs a new default reqwest instance with a pre-configured timeout.
    pub fn new(regionality: AccountRegion) -> Self {
        let timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECONDS.into());
        let client = reqwest::ClientBuilder::new()
            .timeout(timeout)
            .build()
            .unwrap();

        Self {
            http: client,
            regionality,
        }
    }
}
