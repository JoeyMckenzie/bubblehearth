//! BubbleHearth errors that can occur during at any point
//! during the request cycle to Blizzard, mappings, builders, etc.

use thiserror::Error;

/// Wrapped result type useful for marshalling between library and dependencies errors.
pub type BubbleHearthResult<T> = Result<T, BubbleHearthError>;

/// Errors that can occur within the client, including mapped errors from reqwest.
#[derive(Debug, Error)]
pub enum BubbleHearthError {
    /// Represents an error occurring when attempting to construct a client without a required region.
    #[error("A region must be provided when building a client instance.")]
    RegionRequired,
    /// Represents an error occurring when attempting to build with a configured client.
    #[error("A client instance was unable to be constructed.")]
    ClientInstanceRequired,
    /// Represents any reqwest that has failed, propagating the error context.
    #[error("{0}")]
    ClientRequestFailed(#[from] reqwest::Error),
    /// Represents an error that occurred attempting to retrieve a cached access token.
    #[error("No available access token was found.")]
    AccessTokenNotFound,
    /// Represents an error that occurred attempting to determine if refresh of a token is needed.
    #[error("No expiration was found associated to the current authentication context.")]
    ExpirationNotFound,
    /// Represents an error occurring when an internal mutex has failed to lock while determining authentication context.
    #[error("{0}")]
    AuthenticationLockFailed(String),
    /// Represents an error occurring when determining a typed locale based on the locale returned from Blizzard.
    #[error("Locale {0} is unknown.")]
    LocaleUnknown(String),
    /// Represents an error occurring when attempting construct queries.
    #[error("Query struct was not configured with any optional filters.")]
    SearchParametersNotProvided,
    /// Represents an error when attempting to construct a client instance with any options.
    #[error("The configured client is invalid and cannot be built in the current state.")]
    InvalidClientOptions,
    /// Represents an error when attempting to construct a client instance with any options.
    #[error("One more required client options is missing.")]
    InvalidClientOptionField,
}
