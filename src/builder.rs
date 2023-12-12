use std::time::Duration;

use crate::client::BubbleHearthClient;
use crate::errors::{BubbleHearthError, BubbleHearthResult};
use crate::localization::Locale;
use crate::regionality::AccountRegion;

#[derive(Debug)]
pub struct BubbleHearthClientBuilder {
    /// Configurable options for the client instance.
    options: Option<BubbleHearthClientOptions>,
}

impl BubbleHearthClientBuilder {
    /// Creates a new client builder instance with out any options.
    pub fn new() -> Self {
        Self { options: None }
    }

    pub fn with_client_id(self, client_id: String) -> Self {
        let options = match self.options {
            None => BubbleHearthClientOptions {
                client_id: Some(client_id),
                client_secret: None,
                region: None,
                locale: None,
                timeout: None,
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

    pub fn with_client_secret(self, client_secret: String) -> Self {
        let options = match self.options {
            None => BubbleHearthClientOptions {
                client_secret: Some(client_secret),
                client_id: None,
                region: None,
                locale: None,
                timeout: None,
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

    pub fn with_region(self, region: AccountRegion) -> Self {
        let options = match self.options {
            None => BubbleHearthClientOptions {
                region: Some(region),
                client_id: None,
                client_secret: None,
                locale: None,
                timeout: None,
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

    pub fn with_locale(self, locale: Locale) -> Self {
        let options = match self.options {
            None => BubbleHearthClientOptions {
                client_id: None,
                client_secret: None,
                region: None,
                locale: Some(locale),
                timeout: None,
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

    pub fn with_timeout(self, timeout: Duration) -> Self {
        let options = match self.options {
            None => BubbleHearthClientOptions {
                client_id: None,
                client_secret: None,
                region: None,
                locale: None,
                timeout: Some(timeout),
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
}

impl BubbleHearthClientOptions {
    pub fn has_required_options(&self) -> bool {
        if self.locale.is_none() {
            return false;
        }

        if self.locale.is_none() {
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
