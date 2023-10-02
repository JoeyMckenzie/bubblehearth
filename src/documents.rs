//! General data types for document derived data from the Game Data, Profile, and Community APIs.

use serde::{Deserialize, Serialize};

/// A document key associated to all model responses from the Game Data APIs.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentKey {
    /// URL of the associated document.
    pub href: String,
}

/// Self reference link for retrieving individual realm data. Not particularly useful,
/// one should favor using the individual self ref for each realm instead.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Links {
    /// Self reference link.
    #[serde(rename = "self")]
    pub self_ref: DocumentKey,
}
