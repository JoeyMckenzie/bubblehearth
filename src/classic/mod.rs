//! Connectors for Classic WoW Game Data APIs. To date, Blizzard does not offer Community APIs
//! for Classic Wow, though it's on their radar.

use std::sync::Arc;

use crate::auth::AuthenticationContext;
use crate::regionality::AccountRegion;

pub mod realms;

/// A client for WoW Classic, utilizing the base client authentication.
#[derive(Debug)]
pub struct WorldOfWarcraftClassicClient {
    /// Reference to the internal client, configured for timeout and other defaults.
    http: Arc<reqwest::Client>,
    /// Configured account region.
    region: AccountRegion,
    /// Internally cached authentication context, allowing for token reuse and smart refreshing.
    authentication: Arc<AuthenticationContext>,
}

impl WorldOfWarcraftClassicClient {
    /// Constructs a new WoW Classic composed of the base client for querying and authorization.
    pub fn new(
        http: Arc<reqwest::Client>,
        region: AccountRegion,
        authentication: Arc<AuthenticationContext>,
    ) -> Self {
        Self {
            http,
            region,
            authentication,
        }
    }

    /// Gets the region-specific namespace based on the region localilty.
    fn get_namespace_locality(&self) -> String {
        format!("dynamic-classic-{}", self.region.get_region_abbreviation())
    }
}
