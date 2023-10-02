//! Realm data and APIs for World of Warcraft Classic.

use reqwest::StatusCode;
use serde::Deserialize;

use crate::classic::WorldOfWarcraftClassicClient;
use crate::documents::{DocumentKey, Links};
use crate::errors::BubbleHearthResult;
use crate::localization::StringOrStructLocale;
use crate::search::SearchResult;
use crate::timezone::Timezone;

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

/// Realm metadata for all available World of Warcraft Classic servers.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Realm {
    /// Top-level document link to follow of the selected realm ID.
    #[serde(rename = "_links")]
    pub links: Option<Links>,
    /// Document key for the realm, defaults to the URL.
    pub key: Option<DocumentKey>,
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
    pub timezone: Option<Timezone>,
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
    pub key: Option<DocumentKey>,
    /// Name of the realm region.
    pub name: StringOrStructLocale,
    /// ID of the realm region.
    pub id: u64,
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

impl WorldOfWarcraftClassicClient {
    /// Retrieves data about all available realms.
    pub async fn get_realms(&self) -> BubbleHearthResult<RealmsIndex> {
        let url = format!(
            "https://{}.api.blizzard.com/data/wow/realm/index?locale={}",
            self.region.get_region_abbreviation(),
            self.locale.get_locale(),
        );

        let realms = self.send_request(url).await?.json::<RealmsIndex>().await?;

        Ok(realms)
    }

    /// Retrieves a realm's metadata based on the realm slug.
    pub async fn get_realm(&self, slug: String) -> BubbleHearthResult<Option<Realm>> {
        let url = format!(
            "https://{}.api.blizzard.com/data/wow/realm/{}?locale={}",
            self.region.get_region_abbreviation(),
            slug,
            self.locale.get_locale()
        );

        let realm = self.send_request(url).await?;
        if realm.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let realm = realm.json::<Realm>().await?;

        Ok(Some(realm))
    }

    /// Searches for realms with optional timezone, order by, and page query parameters.
    pub async fn search_realms(
        &self,
        timezone: Option<Timezone>,
        order_by: Option<String>,
        page: Option<u32>,
    ) -> BubbleHearthResult<SearchResult<Realm>> {
        let mut url = format!(
            "https://{}.api.blizzard.com/data/wow/search/realm?_page={}",
            self.region.get_region_abbreviation(),
            page.unwrap_or(1)
        );

        if let Some(zone) = timezone {
            let zone_string: String = zone.into();
            url.push_str(&format!("&timezone={}", zone_string));
        }

        if let Some(order) = order_by {
            url.push_str(&format!("&orderby={}", order));
        }

        let search_result = self
            .send_request(url)
            .await?
            .json::<SearchResult<Realm>>()
            .await?;

        Ok(search_result)
    }
}
