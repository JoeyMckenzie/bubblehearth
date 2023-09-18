//! Request models and APIs for using Battle.net and OAuth interactions with Blizzard.

use serde::{Deserialize, Serialize};

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
