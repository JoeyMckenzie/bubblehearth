//! A top-level client client for interacting with Blizzard Game Data APIs,
//! including authentication and all publicly available APIs for Blizzard games.

use std::time::Duration;

use crate::http::reqwest::InternalHttpClient;
use crate::regionality::AccountRegion;

const DEFAULT_TIMEOUT_DURATION_SECONDS: u8 = 5;

/// Fluent builder for configuring the top-level Blizzard client.
#[derive(Debug, Copy, Clone)]
pub struct BubbleHearthClientBuilder {}

/// Configuration options for the internal HTTP client.
#[derive(Debug, Copy, Clone)]
pub struct BubbleHearthClientOptions {
    /// Optional configurable timeout for the internal client, defaulting if not provided.
    pub timeout: Option<Duration>,
}

/// The primary BubbleHearth client, acting as the gateway for connecting
#[derive(Debug)]
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
