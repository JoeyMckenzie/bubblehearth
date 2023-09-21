//! Contextual caching for authentication, allowing resuability of access tokens and smart token refreshing.

use std::ops::Add;
use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::errors::BubbleHearthError::{AccessTokenNotFound, ExpirationNotFound};
use crate::errors::BubbleHearthResult;
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
#[derive(Debug, Clone)]
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
    access_token: Option<String>,
    /// Expiration of the access token, typically on the order of 24 hours.
    expires_at: Option<OffsetDateTime>,
}

impl AuthenticationContext {
    /// Constructs a new authentication context given a recently requested access token with expiration.
    pub fn new(authentication_response: Option<AccessTokenResponse>) -> Self {
        match authentication_response {
            None => Self {
                expires_at: None,
                access_token: None,
            },
            Some(response) => {
                let expires_in_duration = Duration::from_secs(response.expires_in);
                Self {
                    access_token: Some(response.access_token),
                    expires_at: Some(OffsetDateTime::now_utc().add(expires_in_duration)),
                }
            }
        }
    }

    /// Returns a mutable copy of the current access token.
    pub fn try_access_token(&self) -> BubbleHearthResult<String> {
        match &self.access_token {
            None => Err(AccessTokenNotFound),
            Some(token) => Ok(token.clone()),
        }
    }

    /// Determines if the current access has expired and requires refreshing.
    pub fn try_refresh_required(&self) -> BubbleHearthResult<bool> {
        match &self.expires_at {
            None => Err(ExpirationNotFound),
            Some(current_expiration) => {
                let now = OffsetDateTime::now_utc();
                Ok(current_expiration.le(&now))
            }
        }
    }

    /// Requests a raw access token for authenticating against all client requests.
    /// Upon retrieval, access tokens are cached within client unless explicitly flushed.
    pub async fn get_access_token(&self) -> BubbleHearthResult<String> {
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

        Ok(token_response.access_token)
    }
}
