//! A top-level client client for interacting with Blizzard Game Data APIs,
//! including authentication and all publicly available APIs for Blizzard games.

use std::ops::Add;
use std::time::Duration;

use time::OffsetDateTime;

use crate::errors::BubbleHearthResult;
use crate::oauth::AccessTokenResponse;
use crate::regionality::AccountRegion;

const DEFAULT_TIMEOUT_SECONDS: u8 = 5;

/// The primary BubbleHearth client, acting as the gateway for connecting
#[derive(Debug, Clone)]
pub struct BubbleHearthClient {
    /// Client ID provided by Blizzard's developer portal.
    client_id: String,
    /// Client secret provided by Blizzard's developer portal.
    client_secret: String,
    /// Internal HTTP client for sending requests to various Blizzard APIs.
    http: reqwest::Client,
    /// Required account region.
    region: AccountRegion,
    /// Current access token used to authenticate against Blizzard APIs.
    access_token: Option<String>,
    /// Expiration of the access token, typically on the order of 24 hours.
    expires_at: Option<OffsetDateTime>,
}

impl BubbleHearthClient {
    /// Constructs a new client with default configuration options, though requiring a region.
    pub fn new(client_id: String, client_secret: String, region: AccountRegion) -> Self {
        let default_timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECONDS.into());
        Self::new_with_timeout(client_id, client_secret, region, default_timeout)
    }

    /// Constructs a new client instance with a configurable timeout.
    pub fn new_with_timeout(
        client_id: String,
        client_secret: String,
        region: AccountRegion,
        timeout: Duration,
    ) -> Self {
        let client = reqwest::ClientBuilder::new()
            .timeout(timeout)
            .build()
            .unwrap();

        Self {
            client_id,
            client_secret,
            http: client,
            region,
            access_token: None,
            expires_at: None,
        }
    }

    /// Requests a raw access token for authenticating against all client requests.
    /// Upon retrieval, access tokens are cached within client unless explicitly flushed.
    pub async fn get_access_token(&mut self) -> BubbleHearthResult<String> {
        if let Some(auth_context) = &self.access_token {
            return Ok(auth_context.clone());
        }

        let form = reqwest::multipart::Form::new().text("grant_type", "client_credentials");
        let token_response = self
            .http
            .post(self.region.get_token_endpoint())
            .multipart(form)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await?
            .json::<AccessTokenResponse>()
            .await?;

        let expires_in = token_response.expires_in;
        let access_token = token_response.access_token;
        let expires_in_duration = Duration::from_secs(expires_in);
        let expires_at = OffsetDateTime::now_utc().add(expires_in_duration);
        self.access_token = Some(access_token.clone());
        self.expires_at = Some(expires_at);

        Ok(access_token)
    }
}
