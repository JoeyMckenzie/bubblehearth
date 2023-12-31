//! A top-level client client for interacting with Blizzard Game Data APIs,
//! including authentication and all publicly available APIs for Blizzard games.

use std::ops::Add;
use std::sync::Mutex;
use std::time::Duration;

use http::{HeaderMap, StatusCode};
use serde::Deserialize;
use time::OffsetDateTime;

use crate::auth::AccessTokenResponse;
use crate::builder::BubbleHearthClientOptions;
use crate::classic::WorldOfWarcraftClassicConnector;
use crate::connectors::ClientConnector;
use crate::errors::{BubbleHearthError, BubbleHearthResult};
use crate::hearthstone::HearthstoneConnector;
use crate::localization::Locale;
use crate::regionality::AccountRegion;

/// Default the reqwest HTTP timeout to 5 seconds, overridable if provided.
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
///     use bubblehearth::localization::Locale;
///     dotenvy::dotenv().expect("test client credentials unable to load");
///     let client_id = std::env::var("CLIENT_ID").expect("test client ID not found");
///     let client_secret = std::env::var("CLIENT_SECRET").expect("test client secret not found");
///     let client = BubbleHearthClient::new_with_timeout(
///         client_id,
///         client_secret,
///         AccountRegion::US,
///         Locale::EnglishUS,
///         Duration::from_secs(30)
///     );
///
///     // Retrieve an access token, with successive retrievals returning the cached token
///     let token = client.get_access_token().await.unwrap();
///     let cached_token = client.get_access_token().await.unwrap();
///     assert_eq!(token, cached_token);
/// }
#[derive(Debug)]
pub struct BubbleHearthClient {
    /// Reference to the internal client, configured for timeout and other defaults.
    http: reqwest::Client,
    /// Configured account region.
    pub(crate) region: AccountRegion,
    /// Configured locale for all API calls.
    pub(crate) locale: Locale,
    /// Client ID provided by Blizzard's developer portal.
    client_id: String,
    /// Client secret provided by Blizzard's developer portal.
    client_secret: String,
    /// Current access token used to authenticate against Blizzard APIs.
    access_token: Mutex<Option<String>>,
    /// Expiration of the access token, typically on the order of 24 hours.
    expires_at: Mutex<OffsetDateTime>,
}

impl BubbleHearthClient {
    /// Constructs a new client with default configuration options, though requiring a region.
    pub fn new(
        client_id: String,
        client_secret: String,
        region: AccountRegion,
        locale: Locale,
    ) -> Self {
        let default_timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECONDS.into());
        Self::new_with_timeout(client_id, client_secret, region, locale, default_timeout)
    }

    /// Constructs a new client instance with a configurable timeout.
    pub fn new_with_timeout(
        client_id: String,
        client_secret: String,
        region: AccountRegion,
        locale: Locale,
        timeout: Duration,
    ) -> Self {
        let client = reqwest::ClientBuilder::new()
            .timeout(timeout)
            .build()
            .unwrap();

        Self {
            http: client,
            client_id,
            client_secret,
            region,
            locale,
            access_token: Mutex::new(None),
            expires_at: Mutex::new(OffsetDateTime::UNIX_EPOCH),
        }
    }

    /// Constructs a new client instance with configurable options.
    pub fn new_with_options(options: BubbleHearthClientOptions) -> BubbleHearthResult<Self> {
        if !options.has_required_options() {
            return Err(BubbleHearthError::InvalidClientOptions);
        }

        let client = match options.http {
            // If we're not given any reqwest configurations,
            // check to see which type of client should be built
            None => match options.timeout {
                // If no timeout was given, construct one with the default timeout
                None => reqwest::ClientBuilder::new()
                    .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS.into()))
                    .build()
                    .unwrap(),
                // If we have a configured timeout, use that instead
                Some(timeout) => reqwest::ClientBuilder::new()
                    .timeout(timeout)
                    .build()
                    .unwrap(),
            },
            // If we're given a preconfigured HTTP client, use that over all other configurations
            Some(http) => http,
        };

        Ok(Self {
            http: client,
            client_id: options.client_id.unwrap(),
            client_secret: options.client_secret.unwrap(),
            region: options.region.unwrap(),
            locale: options.locale.unwrap(),
            access_token: Mutex::new(None),
            expires_at: Mutex::new(OffsetDateTime::UNIX_EPOCH),
        })
    }

    /// Returns a mutable copy of the current access token. In the case a token refresh is required,
    /// explicitly return a none to force retrieving of a fresh accessing token.
    fn try_access_token(&self) -> BubbleHearthResult<Option<String>> {
        match self.access_token.try_lock() {
            Ok(token_lock) => match token_lock.as_ref() {
                None => Err(BubbleHearthError::AccessTokenNotFound),
                Some(token) => match self.try_refresh_required() {
                    Ok(refresh_required) => {
                        if refresh_required {
                            Ok(None)
                        } else {
                            Ok(Some(token.to_owned()))
                        }
                    }
                    Err(e) => Err(BubbleHearthError::AuthenticationLockFailed(e.to_string())),
                },
            },
            Err(e) => Err(BubbleHearthError::AuthenticationLockFailed(e.to_string())),
        }
    }

    /// Determines if the current access has expired and requires refreshing.
    fn try_refresh_required(&self) -> BubbleHearthResult<bool> {
        match self.expires_at.try_lock() {
            Ok(expiration) => {
                let now = OffsetDateTime::now_utc();
                Ok(expiration.le(&now))
            }
            Err(e) => Err(BubbleHearthError::AuthenticationLockFailed(e.to_string())),
        }
    }

    /// Requests a raw access token for authenticating against all client requests.
    /// Upon retrieval, access tokens are cached within client unless explicitly flushed.
    pub async fn get_access_token(&self) -> BubbleHearthResult<String> {
        // If we have a cached access token, go ahead and grab it as it hasn't hit the expired time yet
        if let Ok(Some(cached_token)) = self.try_access_token() {
            return Ok(cached_token);
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
        let access_token = token_response.access_token;

        if let Ok(mut token_lock) = self.access_token.try_lock() {
            *token_lock = Some(access_token.clone());
        }

        if let Ok(mut expiration_lock) = self.expires_at.try_lock() {
            let expires_in_duration = Duration::from_secs(token_response.expires_in);
            *expiration_lock = OffsetDateTime::now_utc().add(expires_in_duration);
        }

        Ok(access_token)
    }

    /// Gets the region-specific namespace based on the region localilty.
    fn get_namespace_locality(&self) -> String {
        format!("dynamic-classic-{}", self.region.get_region_abbreviation())
    }

    /// Sends a request with the required namespace and authentication token.
    async fn send_request(&self, url: String) -> BubbleHearthResult<reqwest::Response> {
        let token = self.get_access_token().await?;
        let mut headers = HeaderMap::new();
        headers.append(
            "Battlenet-Namespace",
            self.get_namespace_locality().parse().unwrap(),
        );
        let response = self
            .http
            .get(url)
            .headers(headers)
            .bearer_auth(token)
            .send()
            .await?;

        Ok(response)
    }

    /// Sends a request with the required namespace and authentication token and deserializes the response.
    pub(crate) async fn send_request_and_deserialize<T: for<'de> Deserialize<'de>>(
        &self,
        url: String,
    ) -> BubbleHearthResult<T> {
        let response = self.send_request(url).await?.json::<T>().await?;
        Ok(response)
    }

    /// Sends a request with the required namespace and authentication token and deserializes the response.
    pub(crate) async fn send_request_and_optionally_deserialize<T: for<'de> Deserialize<'de>>(
        &self,
        url: String,
    ) -> BubbleHearthResult<Option<T>> {
        let response = self.send_request(url).await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let response = response.json::<T>().await?;

        Ok(Some(response))
    }

    /// A client connector for interacting with World of Warcraft Classic Game Data APIs.
    pub fn classic(&self) -> WorldOfWarcraftClassicConnector {
        WorldOfWarcraftClassicConnector::new_connector(self)
    }

    /// A client connector for interacting with Hearthstone Game Data APIs.
    pub fn hearthstone(&self) -> HearthstoneConnector {
        HearthstoneConnector::new_connector(self)
    }
}
