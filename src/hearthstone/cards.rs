//! Hearthstone card APIs for searching and retrieving cards and card backs.

use crate::errors::BubbleHearthResult;
use crate::hearthstone::card_search::CardSearchQuery;
use crate::hearthstone::HearthstoneConnector;

impl<'a> HearthstoneConnector<'a> {
    /// Searches for cards fronts and back based on the provided search criteria.
    pub async fn search_cards(
        &self,
        _query: Option<CardSearchQuery<'a>>,
    ) -> BubbleHearthResult<()> {
        self.client
            .send_request_and_deserialize(format!(
                "https://us.api.blizzard.com/hearthstone/cards?locale={}",
                self.client.locale.get_locale()
            ))
            .await?;

        Ok(())
    }
}
