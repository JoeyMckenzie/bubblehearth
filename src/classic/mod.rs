//! Connectors for Classic WoW Game Data APIs. To date, Blizzard does not offer Community APIs
//! for Classic Wow, though it's on their radar.

use crate::client::BubbleHearthClient;

pub mod realms;
pub mod regions;

/// A client for WoW Classic, utilizing the base client authentication.
#[derive(Debug)]
pub struct WorldOfWarcraftClassicConnector<'a> {
    client: &'a BubbleHearthClient,
}

impl<'a> WorldOfWarcraftClassicConnector<'a> {
    /// Constructs a new WoW Classic composed of the base client for querying and authorization.
    pub fn new_client(client: &'a BubbleHearthClient) -> Self {
        Self { client }
    }
}
