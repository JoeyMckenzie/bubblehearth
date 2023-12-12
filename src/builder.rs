use std::time::Duration;

use crate::client::BubbleHearthClient;
use crate::errors::{BubbleHearthError, BubbleHearthResult};
use crate::localization::Locale;
use crate::regionality::AccountRegion;

/// A configurable set of options for the user's client,
/// containing a mix of optional and required properties.
/// If a required option is not present when attempting
/// to construct a client from client options, an error
/// will be returned.
///
/// ```rust
/// use std::time::Duration;
/// use bubblehearth::client::BubbleHearthClient;
/// use bubblehearth::regionality::AccountRegion;
/// use bubblehearth::builder::BubbleHearthClientOptions;
///
/// #[tokio::main]
/// async fn main() {
///     use bubblehearth::localization::Locale;
///     dotenvy::dotenv().expect("test client credentials unable to load");
///     let client_id = std::env::var("CLIENT_ID").expect("test client ID not found");
///     let client_secret = std::env::var("CLIENT_SECRET").expect("test client secret not found");
///     let options = BubbleHearthClientOptions {
///         client_id: Some(client_id),
///         client_secret: Some(client_secret),
///         region: Some(AccountRegion::US),
///         // Invalid, a locale is required
///         locale: None,
///         timeout: None,
///         http: None
///     };
///
///     // Trying to construct the client from invalid options
///     // will propagate errors back to the caller
///     let client = BubbleHearthClient::new_with_options(options);
///
///     assert!(client.is_err());
/// }
#[derive(Debug)]
pub struct BubbleHearthClientOptions {
    /// Client ID provided by Blizzard's developer portal.
    pub client_id: Option<String>,
    /// Client secret provided by Blizzard's developer portal.
    pub client_secret: Option<String>,
    /// Configured account region.
    pub region: Option<AccountRegion>,
    /// Configured locale for all API calls.
    pub locale: Option<Locale>,
    /// Configurable HTTP timeout, optional.
    pub timeout: Option<Duration>,
    /// Configurable HTTP timeout, optional and will trump all other configured HTTP options.
    pub http: Option<reqwest::Client>,
}

/// A configurable client instance builder for the BubbleHearth client.
#[derive(Debug, Default)]
pub struct BubbleHearthClientBuilder {
    /// Configurable options for the client instance.
    pub options: Option<BubbleHearthClientOptions>,
}

impl BubbleHearthClientBuilder {
    /// Creates a new client builder instance with out any options.
    pub fn new() -> Self {
        Self { options: None }
    }

    /// Sets the required client ID on the currently configured options.
    pub fn with_client_id(self, client_id: String) -> Self {
        let options = match self.options {
            None => BubbleHearthClientOptions {
                client_id: Some(client_id),
                client_secret: None,
                region: None,
                locale: None,
                timeout: None,
                http: None,
            },
            Some(options) => BubbleHearthClientOptions {
                client_id: Some(client_id),
                ..options
            },
        };

        Self {
            options: Some(options),
        }
    }

    /// Sets the required client secret on the currently configured options.
    pub fn with_client_secret(self, client_secret: String) -> Self {
        let options = match self.options {
            None => BubbleHearthClientOptions {
                client_secret: Some(client_secret),
                client_id: None,
                region: None,
                locale: None,
                timeout: None,
                http: None,
            },
            Some(options) => BubbleHearthClientOptions {
                client_secret: Some(client_secret),
                ..options
            },
        };

        Self {
            options: Some(options),
        }
    }

    /// Sets the required region on the currently configured options.
    pub fn with_region(self, region: AccountRegion) -> Self {
        let options = match self.options {
            None => BubbleHearthClientOptions {
                region: Some(region),
                client_id: None,
                client_secret: None,
                locale: None,
                timeout: None,
                http: None,
            },
            Some(options) => BubbleHearthClientOptions {
                region: Some(region),
                ..options
            },
        };

        Self {
            options: Some(options),
        }
    }

    /// Sets the required locale on the currently configured options.
    pub fn with_locale(self, locale: Locale) -> Self {
        let options = match self.options {
            None => BubbleHearthClientOptions {
                client_id: None,
                client_secret: None,
                region: None,
                locale: Some(locale),
                timeout: None,
                http: None,
            },
            Some(options) => BubbleHearthClientOptions {
                locale: Some(locale),
                ..options
            },
        };

        Self {
            options: Some(options),
        }
    }

    /// Sets the optional request timeout on the currently configured options.
    pub fn with_timeout(self, timeout: Duration) -> Self {
        let options = match self.options {
            None => BubbleHearthClientOptions {
                client_id: None,
                client_secret: None,
                region: None,
                locale: None,
                timeout: Some(timeout),
                http: None,
            },
            Some(options) => BubbleHearthClientOptions {
                timeout: Some(timeout),
                ..options
            },
        };

        Self {
            options: Some(options),
        }
    }

    /// Sets the optional request timeout on the currently configured options.
    pub fn with_http(self, client: reqwest::Client) -> Self {
        let options = match self.options {
            None => BubbleHearthClientOptions {
                client_id: None,
                client_secret: None,
                region: None,
                locale: None,
                timeout: None,
                http: Some(client),
            },
            Some(options) => BubbleHearthClientOptions {
                http: Some(client),
                ..options
            },
        };

        Self {
            options: Some(options),
        }
    }

    /// Attempts to construct the client instances using,
    /// propagating any errors for missing required client options.
    pub fn build(self) -> BubbleHearthResult<BubbleHearthClient> {
        match self.options {
            None => Err(BubbleHearthError::InvalidClientOptions),
            Some(options) => {
                if options.has_required_options() {
                    BubbleHearthClient::new_with_options(options)
                } else {
                    Err(BubbleHearthError::InvalidClientOptionField)
                }
            }
        }
    }
}

impl BubbleHearthClientOptions {
    /// Determines if the client has been validly configured
    /// with the mix of available required and optional properties.
    pub fn has_required_options(&self) -> bool {
        if self.locale.is_none() {
            return false;
        }

        if self.region.is_none() {
            return false;
        }

        if self.client_id.is_none() {
            return false;
        }

        if self.client_secret.is_none() {
            return false;
        }

        true
    }
}
