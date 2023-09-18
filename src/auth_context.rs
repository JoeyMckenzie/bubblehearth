//! Contextual caching for authentication, allowing resuability of access tokens and smart token refreshing.

use std::ops::Add;
use std::time::Duration;

use time::OffsetDateTime;

use crate::oauth::AccessTokenResponse;

/// Represents the current authentication context, including the most recently requested access token.
#[derive(Debug, Clone)]
pub struct AuthenticationContext {
    /// Current access token used to authenticate against Blizzard APIs.
    access_token: String,
    /// Expiration of the access token, typically on the order of 24 hours.
    expires_at: OffsetDateTime,
}

impl AuthenticationContext {
    /// Constructs a new authentication context given a recently requested access token with expiration.
    pub fn new(authentication_response: AccessTokenResponse) -> Self {
        let expires_in_duration = Duration::from_secs(authentication_response.expires_in);
        Self {
            access_token: authentication_response.access_token,
            expires_at: OffsetDateTime::now_utc().add(expires_in_duration),
        }
    }

    /// Returns a mutable copy of the current access token.
    pub fn get_access_token(&self) -> String {
        self.access_token.clone()
    }

    /// Determines if the current access has expired and requires refreshing.
    pub fn refresh_required(&self) -> bool {
        let current = OffsetDateTime::now_utc();
        self.expires_at.le(&current)
    }
}
