use crate::http::reqwest::InternalHttpClient;

/// Battle.net OAuth authorization endpoint for global regions.
pub const GLOBAL_AUTHORIZE_ENDPOINT: &str = "https://oauth.battle.net/authorize";

/// Battle.net OAuth token endpoint for global regions.
pub const GLOBAL_TOKEN_ENDPOINT: &str = "https://oauth.battle.net/token";

/// Battle.net OAuth authorization endpoint for Chinese regions.
pub const CN_AUTHORIZE_ENDPOINT: &str = "https://oauth.battlenet.com.cn/authorize";

/// Battle.net OAuth token endpoint for Chinese regions.
pub const CN_TOKEN_ENDPOINT: &str = "https://oauth.battlenet.com.cn/token";

/// A client for interacting with Battle.net endpoints and authorization.
pub struct BattleNetClient {
    /// Common internal HTTP request client.
    http: InternalHttpClient,
}

impl BattleNetClient {}
