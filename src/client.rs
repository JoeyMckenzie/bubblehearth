//! A top-level client client for interacting with Blizzard Game Data APIs,
//! including authentication and all publicly available APIs for Blizzard games.

use std::sync::Arc;
use std::time::Duration;

use crate::auth::AuthenticationContext;
use crate::classic::client::WorldOfWarcraftClassicClient;
use crate::regionality::AccountRegion;

const DEFAULT_TIMEOUT_SECONDS: u8 = 5;

/// The primary BubbleHearth client, acting as the gateway for connecting.
///
/// ```rust
/// use std::time::Duration;
/// use bubblehearth::client::BubbleHearthClient;
/// use bubblehearth::regionality::AccountRegion;
///
/// #[tokio::main]
/// async fn main() {
///     dotenvy::dotenv().expect("test client credentials unable to load");
///     let client_id = std::env::var("CLIENT_ID").expect("test client ID not found");
///     let client_secret = std::env::var("CLIENT_SECRET").expect("test client secret not found");
///     let client = BubbleHearthClient::new_with_timeout(
///         client_id,
///         client_secret,
///         AccountRegion::US,
///         Duration::from_secs(30),
///     );
///
///     // Retrieve an access token, with successive retrievals returning the cached token
///     let token = client.authentication.get_access_token().await.unwrap();
///     let cached_token = client.authentication.get_access_token().await.unwrap();
///     assert_eq!(token, cached_token);
/// }
#[derive(Debug)]
pub struct BubbleHearthClient {
    /// Internally cached authentication context, allowing for token reuse and smart refreshing.
    pub authentication: Arc<AuthenticationContext>,
    /// A client for querying World of Warcraft Classic game data.
    pub classic: WorldOfWarcraftClassicClient,
}

impl BubbleHearthClient {
    /// Constructs a new client with default configuration options, though requiring a region.
    pub fn new(client_id: String, client_secret: String, region: AccountRegion) -> Self {
        let default_timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECONDS.into());
        Self::new_with_timeout(client_id, client_secret, region, default_timeout)
    }

    /// Constructs a new client instance with a configurable timeout.
    pub fn new_with_timeout(
        client_id: String,
        client_secret: String,
        region: AccountRegion,
        timeout: Duration,
    ) -> Self {
        let client = reqwest::ClientBuilder::new()
            .timeout(timeout)
            .build()
            .unwrap();
        let ref_client = Arc::new(client);
        let authentication = AuthenticationContext::new(
            ref_client.clone(),
            region,
            client_id.clone(),
            client_secret.clone(),
        );
        let ref_authentication = Arc::new(authentication);

        Self {
            authentication: ref_authentication.clone(),
            classic: WorldOfWarcraftClassicClient::new(ref_client, region, ref_authentication),
        }
    }
}
