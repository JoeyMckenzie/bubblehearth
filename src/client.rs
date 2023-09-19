//! A top-level client client for interacting with Blizzard Game Data APIs,
//! including authentication and all publicly available APIs for Blizzard games.

use std::sync::Mutex;
use std::time::Duration;

use crate::auth::{AccessTokenResponse, AuthenticationContext};
use crate::errors::BubbleHearthResult;
use crate::regionality::AccountRegion;

const DEFAULT_TIMEOUT_SECONDS: u8 = 5;

/// The primary BubbleHearth client, acting as the gateway for connecting.
///
/// ```rust
/// use std::time::Duration;
/// use bubblehearth::client::BubbleHearthClient;
/// use bubblehearth::regionality::AccountRegion;
///
/// #[tokio::main]
/// async fn main() {
///     dotenvy::dotenv().expect("test client credentials unable to load");
///     let client_id = std::env::var("CLIENT_ID").expect("test client ID not found");
///     let client_secret = std::env::var("CLIENT_SECRET").expect("test client secret not found");
///     let client = BubbleHearthClient::new_with_timeout(
///         client_id,
///         client_secret,
///         AccountRegion::US,
///         Duration::from_secs(30),
///     );
///
///     // Retrieve an access token, with successive retrievals returning the cached token
///     let token = client.get_access_token().await.unwrap();
///     let cached_token = client.get_access_token().await.unwrap();
///     assert_eq!(token, cached_token);
/// }
#[derive(Debug)]
pub struct BubbleHearthClient {
    /// Client ID provided by Blizzard's developer portal.
    client_id: String,
    /// Client secret provided by Blizzard's developer portal.
    client_secret: String,
    /// Internal HTTP client for sending requests to various Blizzard APIs.
    http: reqwest::Client,
    /// Required account region.
    region: AccountRegion,
    /// Internally cached authentication context, allowing for token reuse and smart refreshing.
    authentication: Mutex<AuthenticationContext>,
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
            authentication: Mutex::new(AuthenticationContext::new(None)),
        }
    }

    /// Requests a raw access token for authenticating against all client requests.
    /// Upon retrieval, access tokens are cached within client unless explicitly flushed.
    pub async fn get_access_token(&self) -> BubbleHearthResult<String> {
        if let Ok(lock) = self.authentication.try_lock() {
            // If we have an existing access token, return it and skip the call to retrieve a new one
            if lock.try_refresh_required().unwrap_or(false) {
                if let Ok(token) = lock.try_access_token() {
                    return Ok(token);
                }
            }
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

        // TODO: Probably don't want to clone here, figure it out later
        let access_token = token_response.access_token.clone();

        if let Ok(mut lock) = self.authentication.try_lock() {
            *lock = AuthenticationContext::new(Some(token_response))
        }

        Ok(access_token)
    }
}
