//! Common connector properties and traits, functionality,
//! and types shared amongst all client connectors.

use crate::client::BubbleHearthClient;

pub trait ClientConnector<'a> {
    /// Constructs a new connector composed of the base client for querying and authorization.
    fn new_connector(client: &'a BubbleHearthClient) -> Self
    where
        Self: Sized;
}
