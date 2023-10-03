//! Region data models and various APIs for retrieval and searching.

use http::StatusCode;
use serde::Deserialize;

use crate::classic::WorldOfWarcraftClassicConnector;
use crate::documents::Links;
use crate::errors::BubbleHearthResult;

/// Response from the regions endpoint, containing a list of regions
/// endpoints to retrieve further detail on the region.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RegionsIndex {
    /// Top-level document link to follow for a selected region ID.
    #[serde(rename = "_links")]
    pub links: Links,
    /// List of available regions and their metadata.
    pub regions: Vec<Region>,
}

/// Region metadata returned from the index endpoint.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Region {
    /// Endpoint for retrieving further detail about the region.
    pub href: Option<String>,
    /// Region ID.
    pub id: Option<u32>,
    /// Name of the region.
    pub name: Option<String>,
    /// Regional tag, i.e. US, KR, etc.
    pub tag: Option<String>,
}

impl<'a> WorldOfWarcraftClassicConnector<'a> {
    /// Retrieves data about all available regions.
    pub async fn get_regions(&self) -> BubbleHearthResult<RegionsIndex> {
        let url = format!(
            "https://{}.api.blizzard.com/data/wow/region/index?locale={}",
            self.client.region.get_region_abbreviation(),
            self.client.locale.get_locale(),
        );

        let regions = self
            .client
            .send_request(url)
            .await?
            .json::<RegionsIndex>()
            .await?;

        Ok(regions)
    }

    /// Retrieves data about all available regions.
    pub async fn get_region(&self, region_id: u32) -> BubbleHearthResult<Option<Region>> {
        let url = format!(
            "https://{}.api.blizzard.com/data/wow/region/{region_id}?locale={}",
            self.client.region.get_region_abbreviation(),
            self.client.locale.get_locale(),
        );

        let region_response = self.client.send_request(url).await?;
        if region_response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let region = region_response.json::<Region>().await?;

        Ok(Some(region))
    }
}
