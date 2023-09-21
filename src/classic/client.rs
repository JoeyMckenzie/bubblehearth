#![warn(dead_code)]

//! Client connectors to the World of Warcraft Classic Game Data APIs.

use std::sync::Arc;

use reqwest::header::HeaderMap;

use crate::auth::AuthenticationContext;
use crate::errors::BubbleHearthResult;
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

    /// Retrieves data about all available realms.
    pub async fn get_realms(&self, locale: Locale) -> BubbleHearthResult<()> {
        let url = format!(
            "https://{}.api.blizzard.com/data/wow/realm/index?locale={}",
            self.region.get_region_abbreviation(),
            locale.get_locale(),
        );

        let token = self.authentication.get_access_token().await?;
        let mut headers = HeaderMap::new();
        headers.append("Battlenet-Namespace", NAMESPACE.parse().unwrap());
        headers.append("Access", NAMESPACE.parse().unwrap());
        let realms = self
            .http
            .get(url)
            .headers(headers)
            .bearer_auth(token)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        dbg!(realms);

        Ok(())
    }
}
