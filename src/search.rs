//! Search result data models and utilities for all Game Data, Community, and Profile APIs.

use serde::Deserialize;

use crate::documents::DocumentKey;

/// Paginated search results from various Game Data APIs.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult<T> {
    /// Current page of results.
    pub page: u32,
    /// Number of search results on the current page.
    pub page_size: u32,
    /// Maximum page size of search results.
    pub max_page_size: u32,
    /// The number of pages in the search result data.
    pub page_count: u32,
    /// List of search results data, generic over the type of data returned from the API.
    pub results: Vec<SearchResultItem<T>>,
}

/// Generic paged data returned from the search result.
#[derive(Debug, Deserialize)]
pub struct SearchResultItem<T> {
    /// Key of the search result item
    pub key: DocumentKey,
    /// Generic item data returned from the search.
    pub data: T,
}
