//! Hearthstone card APIs for searching and retrieving cards and card backs.

use crate::errors::BubbleHearthResult;
use crate::hearthstone::HearthstoneConnector;

impl<'a> HearthstoneConnector<'a> {
    /// Searches for cards fronts and back based on the provided search criteria.
    pub async fn search_cards(&self) -> BubbleHearthResult<()> {
        self.client
            .send_request(format!(
                "https://us.api.blizzard.com/hearthstone/cards?locale={}",
                self.client.locale.get_locale()
            ))
            .await?;

        Ok(())
    }
}

/// A query search struct containing all the various filters available for card searching.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct CardSearchQuery<'a> {
    /// The slug of the set the card belongs to. If you do not supply a value, cards from all sets will be returned.
    pub set: Option<&'a str>,
    /// The slug of the card's class.
    pub slug: Option<&'a str>,
}

/// A query builder for fluently building card search queries.
///
/// ```rust
/// use bubblehearth::hearthstone::cards::CardSearchQueryBuilder;
///
/// let builder = CardSearchQueryBuilder::default();
/// let query = builder.with_set("").with_slug("38913-a-light-in-the-darkness").build().ok();
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CardSearchQueryBuilder<'a> {
    /// Internal search to be built.
    query: CardSearchQuery<'a>,
}

impl<'a> Default for CardSearchQueryBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CardSearchQueryBuilder<'a> {
    /// Constructs a new default instance without a configured query struct.
    pub fn new() -> Self {
        Self {
            query: CardSearchQuery::default(),
        }
    }

    /// Include a set for searching.
    pub fn with_set(&self, set: &'a str) -> Self {
        Self {
            query: CardSearchQuery {
                set: Some(set),
                ..self.query
            },
        }
    }

    /// Include a card slug for searching.
    pub fn with_slug(&self, slug: &'a str) -> Self {
        Self {
            query: CardSearchQuery {
                slug: Some(slug),
                ..self.query
            },
        }
    }

    /// Returns the structured query struct after being configured by the requester.
    pub fn build(self) -> BubbleHearthResult<CardSearchQuery<'a>> {
        Ok(self.query)
    }
}

#[cfg(test)]
mod card_search_queries {
    use crate::hearthstone::cards::CardSearchQueryBuilder;

    #[test]
    fn returns_ok_when_validly_built() {
        // Arrange
        let builder = CardSearchQueryBuilder::new();

        // Act
        let query = builder.with_set("set").with_slug("slug").build();
        let query_ok = query.is_ok();
        let query = query.unwrap();

        // Assert
        assert!(query_ok);
        assert_eq!(query.set, Some("set"));
        assert_eq!(query.slug, Some("slug"));
    }
}
