//! Connectors for Classic WoW Game Data APIs. To date, Blizzard does not offer Community APIs
//! for Classic Wow, though it's on their radar.

use std::sync::Arc;

use http::HeaderMap;

use crate::auth::AuthenticationContext;
use crate::errors::BubbleHearthResult;
use crate::localization::Locale;
use crate::regionality::AccountRegion;

pub mod realms;
pub mod regions;

/// A client for WoW Classic, utilizing the base client authentication.
#[derive(Debug)]
pub struct WorldOfWarcraftClassicClient {
    /// Reference to the internal client, configured for timeout and other defaults.
    http: Arc<reqwest::Client>,
    /// Configured account region.
    region: AccountRegion,
    /// Configured locale for all API calls.
    locale: Locale,
    /// Internally cached authentication context, allowing for token reuse and smart refreshing.
    authentication: Arc<AuthenticationContext>,
}

impl WorldOfWarcraftClassicClient {
    /// Constructs a new WoW Classic composed of the base client for querying and authorization.
    pub fn new(
        http: Arc<reqwest::Client>,
        region: AccountRegion,
        locale: Locale,
        authentication: Arc<AuthenticationContext>,
    ) -> Self {
        Self {
            http,
            region,
            locale,
            authentication,
        }
    }

    /// Gets the region-specific namespace based on the region localilty.
    fn get_namespace_locality(&self) -> String {
        format!("dynamic-classic-{}", self.region.get_region_abbreviation())
    }

    /// Sends a request with the required namespace and authentication token.
    async fn send_request(&self, url: String) -> BubbleHearthResult<reqwest::Response> {
        let token = self.authentication.get_access_token().await?;
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
}
