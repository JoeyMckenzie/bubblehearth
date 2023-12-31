//! Connectors for Classic WoW Game Data APIs. To date, Blizzard does not offer Community APIs
//! for Classic Wow, though it's on their radar.

use crate::client::BubbleHearthClient;
use crate::connectors::ClientConnector;

pub mod realms;
pub mod regions;

/// A client for WoW Classic, utilizing the base client authentication.
#[derive(Debug)]
pub struct WorldOfWarcraftClassicConnector<'a> {
    /// Parent client containing the HTTP client and authorization context.
    client: &'a BubbleHearthClient,
}

impl<'a> ClientConnector<'a> for WorldOfWarcraftClassicConnector<'a> {
    /// Constructs a new WoW Classic composed of the base client for querying and authorization.
    fn new_connector(client: &'a BubbleHearthClient) -> Self {
        Self { client }
    }
}
