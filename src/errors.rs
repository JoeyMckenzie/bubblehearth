//! BubbleHearth errors that can occur during at any point
//! during the request cycle to Blizzard, mappings, builders, etc.

use thiserror::Error;

pub type BubbleHearthResult<T> = Result<T, BubbleHearthError>;

/// Errors that can occur within the client, including mapped errors from reqwest.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Error)]
pub enum BubbleHearthError {
    /// Represents an error occurring when attempting to construct a client without a required region.
    #[error("A region must be provided when building a client instance.")]
    RegionRequired,
    /// Represents an error occurring when attempting to build with a configured client.
    #[error("A client instance was unable to be constructed.")]
    ClientInstanceRequired,
}
