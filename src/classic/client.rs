#![warn(dead_code)]

//! Client connectors to the World of Warcraft Classic Game Data APIs.

use std::sync::{Arc, Mutex};
use reqwest::header::HeaderMap;

use crate::auth::AuthenticationContext;
use crate::localization::Locale;
use crate::regionality::AccountRegion;

const NAMESPACE: &str = "dynamic-classic-us";

/// A client for WoW Classic, utilizing the base client authentication.
#[derive(Debug)]
pub struct WorldOfWarcraftClassicClient {
    /// Reference to the internal client, configured for timeout and other defaults.
    http: Arc<reqwest::Client>,
    /// Configured account region.
    region: AccountRegion,
    /// Referenced to the cached authentication context.
    authentication: Arc<Mutex<AuthenticationContext>>,
}

impl WorldOfWarcraftClassicClient {
    /// Constructs a new WoW Classic composed of the base client for querying and authorization.
    pub fn new(
        http: Arc<reqwest::Client>,
        region: AccountRegion,
        authentication: Arc<Mutex<AuthenticationContext>>,
    ) -> Self {
        Self {
            http,
            region,
            authentication,
        }
    }

    /// Retrieves data about all available realms.
    pub async fn get_realms(&self, locale: Locale) {
        let url = format!(
            "https://{}.api.blizzard.com/data/wow/realm/index?locale={}",
            self.region.get_region_prefix(),
            locale.get_locale(),
        );

        let mut headers = HeaderMap::new();
        headers.append("Battlenet-Namespace", NAMESPACE.parse().unwrap());
        headers.append("Access", NAMESPACE.parse().unwrap());
        let realms = self.http.get(url).headers(headers).bearer_auth(self.authentication)
    }
}
