//! An internal HTTP client for all interacting with all Blizzard APIs used across all crates.

use std::time::Duration;

use crate::regionality::AccountRegion;

const DEFAULT_TIMEOUT_SECONDS: u8 = 5;

/// A configurable HTTP client used across crates for connecting to all Blizzard APIs.
#[derive(Debug, Clone)]
pub struct InternalHttpClient {
    /// Internal reqwest client for making HTTP requests.
    http: reqwest::Client,
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

/// A client builder with configurable HTTP options.
#[derive(Debug, Clone)]
pub struct InternalHttpClientBuilder {
    /// Internal client instance to be configured.
    client: InternalHttpClient,
    /// Optional configurable timeout.
    timeout: Option<Duration>,
    /// Required account region for targeting the correct set of Blizzard APIs.
    regionality: AccountRegion,
}

impl InternalHttpClientBuilder {
    /// Constructs a new client builder instances with default options mirroring the default client.
    pub fn new() -> Self {
        let default_region = AccountRegion::US;
        Self {
            client: InternalHttpClient::new(default_region),
            timeout: Some(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS.into())),
            regionality: default_region,
        }
    }

    /// Sets the timeout on the internal client.
    pub fn timeout(self, duration: Duration) -> Self {
        Self {
            timeout: Some(duration),
            ..self
        }
    }

    /// Sets the region on the internal client.
    pub fn region(self, region: AccountRegion) -> Self {
        Self {
            regionality: region,
            ..self
        }
    }
}
