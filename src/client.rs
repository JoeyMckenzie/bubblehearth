//! A top-level client client for interacting with Blizzard Game Data APIs,
//! including authentication and all publicly available APIs for Blizzard games.

use std::time::Duration;

use crate::errors::{BubbleHearthError, BubbleHearthResult};
use crate::http::reqwest::InternalHttpClient;
use crate::regionality::AccountRegion;

const DEFAULT_TIMEOUT_DURATION_SECONDS: u8 = 5;

/// Fluent builder for configuring the top-level Blizzard client.
#[derive(Debug, Clone)]
pub struct BubbleHearthClientBuilder {
    /// Client to be constructed, requiring at least a region to be configured.
    client: Option<BubbleHearthClient>,
    /// Optional configurable timeout for the internal client, defaulting if not provided.
    timeout: Option<Duration>,
    /// Optionally configuration account region, though required on construction.
    region: Option<AccountRegion>,
}

/// Configuration options for the internal HTTP client.
#[derive(Debug, Copy, Clone)]
pub struct BubbleHearthClientOptions {
    /// Optional configurable timeout for the internal client, defaulting if not provided.
    timeout: Option<Duration>,
}

/// The primary BubbleHearth client, acting as the gateway for connecting
#[derive(Debug, Clone)]
pub struct BubbleHearthClient {
    /// Internal HTTP client for sending requests to various Blizzard APIs.
    internal_client: InternalHttpClient,
}

impl BubbleHearthClient {
    /// Constructs a new client with default configuration options, though requiring a region.
    pub fn new(region: AccountRegion) -> Self {
        let internal_client = InternalHttpClient::new(region);
        Self { internal_client }
    }
}

impl Default for BubbleHearthClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BubbleHearthClientBuilder {
    /// Constructs a new client builder instance with default options.
    pub fn new() -> Self {
        Self {
            client: None,
            timeout: None,
            region: None,
        }
    }

    /// Attempts to construct a client instance, checking for a required account region.
    pub fn build(self) -> BubbleHearthResult<BubbleHearthClient> {
        match self.region {
            None => Err(BubbleHearthError::RegionRequired),
            Some(_) => match self.client {
                None => Err(BubbleHearthError::ClientInstanceRequired),
                Some(client) => Ok(client),
            },
        }
    }
}
