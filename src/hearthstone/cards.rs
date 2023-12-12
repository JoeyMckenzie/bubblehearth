//! Hearthstone card APIs for searching and retrieving cards and card backs.

use serde::{Deserialize, Serialize};

use crate::errors::BubbleHearthResult;
use crate::hearthstone::card_search::CardSearchQuery;
use crate::hearthstone::HearthstoneConnector;
use crate::BubbleHearthId;

/// A generalized cards response for all Hearthstone card-based responses.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cards {
    /// Cards based on the card search results.
    pub cards: Vec<Card>,
    /// Number of cards in the search result.
    pub card_count: BubbleHearthId,
    /// Number of pages based on the search result.
    pub page_count: BubbleHearthId,
    /// Current page of the search results.
    pub page: BubbleHearthId,
}

/// A card with all associated Hearthstone metadata for the result.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    /// ID of the card.
    pub id: BubbleHearthId,
    /// ID of the card collectible.
    pub collectible: BubbleHearthId,
    /// Slugified title of the card.
    pub slug: String,
    /// Class ID of the card.
    pub class_id: BubbleHearthId,
    pub multi_class_ids: Vec<BubbleHearthId>,
    pub spell_school_id: Option<BubbleHearthId>,
    pub card_type_id: BubbleHearthId,
    pub card_set_id: BubbleHearthId,
    pub rarity_id: BubbleHearthId,
    pub artist_name: String,
    pub mana_cost: BubbleHearthId,
    pub name: String,
    pub text: String,
    pub image: String,
    pub image_gold: String,
    pub flavor_text: String,
    pub crop_image: String,
    pub keyword_ids: Option<Vec<BubbleHearthId>>,
    pub duels: Option<Duels>,
    pub copy_of_card_id: Option<BubbleHearthId>,
    pub health: Option<BubbleHearthId>,
    pub attack: Option<BubbleHearthId>,
    pub minion_type_id: Option<BubbleHearthId>,
    pub child_ids: Option<Vec<BubbleHearthId>>,
    pub rune_cost: Option<RuneCost>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Duels {
    pub relevant: bool,
    pub constructed: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuneCost {
    pub blood: i64,
    pub frost: i64,
    pub unholy: i64,
}

impl<'a> HearthstoneConnector<'a> {
    /// Searches for cards fronts and back based on the provided search criteria.
    pub async fn search_cards(
        &self,
        _query: Option<CardSearchQuery<'a>>,
    ) -> BubbleHearthResult<Cards> {
        let cards: Cards = self
            .client
            .send_request_and_deserialize(format!(
                "https://us.api.blizzard.com/hearthstone/cards?locale={}",
                self.client.locale.get_normalized_locale()
            ))
            .await?;

        Ok(cards)
    }
}
