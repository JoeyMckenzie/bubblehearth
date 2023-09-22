//! Realm data and APIs for World of Warcraft Classic.

use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::classic::WorldOfWarcraftClassicClient;
use crate::errors::BubbleHearthResult;
use crate::localization::{Locale, StringOrStructLocale};

/// Response structure from the realms index endpoint, listing all available realms
/// with associated metadata like name, status, slug, etc.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RealmsIndex {
    /// Top-level document link to follow for a selected realm ID.
    #[serde(rename = "_links")]
    pub links: Links,
    /// List of available realms and their metadata.
    pub realms: Vec<Realm>,
}

/// Self reference link for retrieving individual realm data. Not particularly useful,
/// one should favor using the individual self ref for each realm instead.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Links {
    /// Self reference link.
    #[serde(rename = "self")]
    pub self_ref: Key,
}

/// Realm metadata for all available World of Warcraft Classic servers.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Realm {
    /// Top-level document link to follow of the selected realm ID.
    #[serde(rename = "_links")]
    pub links: Option<Links>,
    /// Document key for the realm, defaults to the URL.
    pub key: Option<Key>,
    /// Localized realm name.
    pub name: StringOrStructLocale,
    /// Numeric Realm ID.
    pub id: u64,
    /// Slugified realm name.
    pub slug: String,
    /// Realm category correlating to its location, i.e. US West, US East, Oceanic, etc.
    pub category: Option<StringOrStructLocale>,
    /// Realm locale.
    pub locale: Option<String>,
    /// Realm timezone name, i.e. America/New_York, America/Los_Angeles, etc.
    pub timezone: Option<String>,
    /// Flag for indicating if the realm is a PVP-based tournament realm.
    pub is_tournament: Option<bool>,
    /// Realm region, including document links.
    pub region: Option<RealmRegion>,
    /// Realm type metadata.
    #[serde(rename = "type")]
    pub realm_type: Option<RealmType>,
}

/// Realm region data, including document links and name.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RealmRegion {
    /// Document key of the realm region.
    pub key: Key,
    /// Name of the realm region.
    pub name: StringOrStructLocale,
    /// ID of the realm region.
    pub id: u64,
}

/// The greater connected realm data, housing multiple regional servers.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ConnectedRealm {
    /// Document key of the realm region.
    pub key: Key,
}

/// Realm type data, indicating PVP, PVE, Normal, etc.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RealmType {
    /// Base realm type, i.e. Normal, RP, etc.
    #[serde(rename = "type")]
    pub realm_type: String,
    /// Realm environment, i.e. PVP, PVE, etc.
    pub name: StringOrStructLocale,
}

/// A document key associated to all model responses from the Game Data APIs.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Key {
    /// URL of the associated document.
    pub href: String,
}

impl WorldOfWarcraftClassicClient {
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

    /// Retrieves data about all available realms.
    pub async fn get_realms(&self, locale: Locale) -> BubbleHearthResult<RealmsIndex> {
        let url = format!(
            "https://{}.api.blizzard.com/data/wow/realm/index?locale={}",
            self.region.get_region_abbreviation(),
            locale.get_locale(),
        );

        let realms = self.send_request(url).await?.json::<RealmsIndex>().await?;

        Ok(realms)
    }

    /// Retrieves a realm's metadata based on the realm slug.
    pub async fn get_realm(
        &self,
        slug: String,
        locale: Option<Locale>,
    ) -> BubbleHearthResult<Option<Realm>> {
        let mut url = format!(
            "https://{}.api.blizzard.com/data/wow/realm/{}",
            self.region.get_region_abbreviation(),
            slug,
        );

        if let Some(locale_for_query) = locale {
            url.push_str(&format!("?locale={}", locale_for_query.get_locale()));
        }

        let realm = self.send_request(url).await?;
        if realm.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let realm = realm.json::<Realm>().await?;

        Ok(Some(realm))
    }
}
