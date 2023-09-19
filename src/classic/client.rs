#![allow(dead_code)]

//! Client connectors to the World of Warcraft Classic Game Data APIs.

use crate::client::BubbleHearthClient;

/// A client for WoW Classic, utilizing the base client authentication.
#[derive(Debug)]
pub struct WorldOfWarcraftClassicClient {
    base_client: BubbleHearthClient,
}
