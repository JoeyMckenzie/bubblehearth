//! Connectors for the Hearthstone Game Data APIs.
//! Offers searching for cards, decks, and general metadata.

use crate::client::BubbleHearthClient;
use crate::connectors::ClientConnector;

pub mod card_search;
pub mod cards;

/// A client for WoW Classic, utilizing the base client authentication.
#[derive(Debug)]
pub struct HearthstoneConnector<'a> {
    /// Parent client containing the HTTP client and authorization context.
    client: &'a BubbleHearthClient,
}

impl<'a> ClientConnector<'a> for HearthstoneConnector<'a> {
    /// Constructs a new WoW Classic composed of the base client for querying and authorization.
    fn new_connector(client: &'a BubbleHearthClient) -> Self {
        Self { client }
    }
}
