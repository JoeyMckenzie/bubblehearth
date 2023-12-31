//! Card search query structures and builders for constructing search requests.

use crate::errors::BubbleHearthResult;

/// A query search struct containing all the various filters available for card searching.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct CardSearchQuery<'a> {
    /// The slug of the set the card belongs to. If you do not supply a value, cards from all sets will be returned.
    pub set: Option<&'a str>,
    /// The slug of the card's class.
    pub class: Option<&'a str>,
    /// The mana cost required to play the card. You can include multiple values in a comma-separated list of numeric values.
    pub mana_costs: Option<Vec<u32>>,
}

/// A query builder for fluently building card search queries.
///
/// ```rust
/// use bubblehearth::hearthstone::card_search::CardSearchQueryBuilder;
///
/// let builder = CardSearchQueryBuilder::default();
/// let query = builder.with_set("").with_class("38913-a-light-in-the-darkness").build().ok();
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CardSearchQueryBuilder<'a> {
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
    pub fn with_set(self, set: &'a str) -> Self {
        Self {
            query: CardSearchQuery {
                set: Some(set),
                ..self.query
            },
        }
    }

    /// Include a card slug for searching.
    pub fn with_class(self, slug: &'a str) -> Self {
        Self {
            query: CardSearchQuery {
                class: Some(slug),
                ..self.query
            },
        }
    }

    /// Include mana costs for searching.
    pub fn with_mana_cost(self, mana_costs: Vec<u32>) -> Self {
        match self.query.mana_costs {
            None => {
                let mut mana_costs = mana_costs.clone();
                mana_costs.sort();
                Self {
                    query: CardSearchQuery {
                        mana_costs: Some(mana_costs),
                        ..self.query
                    },
                }
            }
            Some(existing_mana_costs) => {
                let mut mana_costs = mana_costs.clone();
                mana_costs.sort();

                let mut updated_mana_costs = [mana_costs, existing_mana_costs].concat();
                updated_mana_costs.sort();

                Self {
                    query: CardSearchQuery {
                        mana_costs: Some(updated_mana_costs),
                        ..self.query
                    },
                }
            }
        }
    }

    /// Returns the structured query struct after being configured by the requester.
    pub fn build(self) -> BubbleHearthResult<CardSearchQuery<'a>> {
        Ok(self.query)
    }
}

#[cfg(test)]
mod card_search_queries {
    use crate::hearthstone::card_search::CardSearchQueryBuilder;

    #[test]
    fn returns_ok_when_validly_built() {
        // Arrange
        let builder = CardSearchQueryBuilder::new();

        // Act
        let query = builder.with_set("set").with_class("slug").build();
        let query_ok = query.is_ok();
        let query = query.unwrap();

        // Assert
        assert!(query_ok);
        assert_eq!(query.set, Some("set"));
        assert_eq!(query.class, Some("slug"));
    }

    #[test]
    fn returns_ok_when_built_with_lists() {
        // Arrange
        let expected = vec![100, 200];
        let builder = CardSearchQueryBuilder::new();

        // Act
        let query = builder.with_mana_cost(expected.clone()).build();
        let query_ok = query.is_ok();
        let query = query.unwrap();

        dbg!(&query);

        // Assert
        assert!(query_ok);
        assert_eq!(query.mana_costs, Some(expected));
    }

    #[test]
    fn returns_ok_when_built_with_lists_and_appended() {
        // Arrange
        let expected = vec![100, 200, 300];
        let builder = CardSearchQueryBuilder::new();

        // Act
        let query = builder
            .with_mana_cost(vec![100, 200])
            .with_mana_cost(vec![300])
            .build();

        let query_ok = query.is_ok();
        let query = query.unwrap();

        // Assert
        assert!(query_ok);
        assert_eq!(query.mana_costs, Some(expected));
    }

    #[test]
    fn returns_ok_when_built_with_unsorted_lists_and_appended() {
        // Arrange
        let expected = vec![50, 100, 150, 200];
        let builder = CardSearchQueryBuilder::new();

        // Act
        let query = builder
            .with_mana_cost(vec![200, 50])
            .with_mana_cost(vec![150, 100])
            .build();

        let query_ok = query.is_ok();
        let query = query.unwrap();

        // Assert
        assert!(query_ok);
        assert_eq!(query.mana_costs, Some(expected));
    }
}
