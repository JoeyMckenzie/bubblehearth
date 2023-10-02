//! Contextual caching for authentication, allowing resuability of access tokens and smart token refreshing.

use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::errors::{BubbleHearthError, BubbleHearthResult};
use crate::regionality::AccountRegion;

/// Represents the access token response at the token endpoint based on the client region.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessTokenResponse {
    /// Represents the access token used to authenticate against Blizzard APIs.
    pub access_token: String,
    /// OAuth-based token type, usually a bearer.
    pub token_type: String,
    /// Number of seconds until the token expires, usually defaulting to 1 day.
    pub expires_in: u64,
    /// Subscriber of the authentication request, defaults to the client ID of the request.
    pub sub: String,
    /// Optional scope associated to the token, mainly used for user profile data.
    pub scope: Option<String>,
}

/// Represents the current authentication context, including the most recently requested access token.
#[derive(Debug)]
pub struct AuthenticationContext {
    /// Reference to the internal client, configured for timeout and other defaults.
    http: Arc<reqwest::Client>,
    /// Configured account region.
    region: AccountRegion,
    /// Client ID provided by Blizzard's developer portal.
    client_id: String,
    /// Client secret provided by Blizzard's developer portal.
    client_secret: String,
    /// Current access token used to authenticate against Blizzard APIs.
    access_token: Mutex<Option<String>>,
    /// Expiration of the access token, typically on the order of 24 hours.
    expires_at: Mutex<OffsetDateTime>,
}

impl AuthenticationContext {
    /// Constructs a new authentication context given a recently requested access token with expiration.
    pub fn new(
        http: Arc<reqwest::Client>,
        region: AccountRegion,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Self {
            http,
            region,
            client_id,
            client_secret,
            access_token: Mutex::new(None),
            expires_at: Mutex::new(OffsetDateTime::UNIX_EPOCH),
        }
    }

    /// Returns a mutable copy of the current access token. In the case a token refresh is required,
    /// explicitly return a none to force retrieving of a fresh accessing token.
    pub fn try_access_token(&self) -> BubbleHearthResult<Option<String>> {
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
    pub fn try_refresh_required(&self) -> BubbleHearthResult<bool> {
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
}
