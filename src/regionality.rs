//! Region-based metadata for targeting specific instances of the Blizzard APIs.

/// Authorize endpoint for global regionalities.
pub const GLOBAL_AUTHORIZE_ENDPOINT: &str = "https://oauth.battle.net/authorize";

/// Token endpoint for global regionalities.
pub const GLOBAL_TOKEN_ENDPOINT: &str = "https://oauth.battle.net/token";

/// Authorize endpoint for the China regionality.
pub const CN_AUTHORIZE_ENDPOINT: &str = "https://oauth.battlenet.com.cn/authorize";

/// Authorize endpoint for China regionality.
pub const CN_TOKEN_ENDPOINT: &str = "https://oauth.battlenet.com.cn/token";

/// Regions associated to their corresponding API gateways.
#[derive(Debug, Clone, Copy)]
pub enum AccountRegion {
    /// Represents the China region and China API gateway.
    CN,
    /// Represents the United States region and Global API gateway.
    US,
    /// Represents the European Union region and Global API gateway.
    EU,
    /// Represents the Korean region and Global API gateway.
    KR,
    /// Represents the Taiwan States region and Global API gateway.
    TW,
}

impl AccountRegion {
    /// Determines the appropriate token endpoint based on the client region.
    pub fn get_token_endpoint(&self) -> &str {
        match self {
            AccountRegion::CN => CN_AUTHORIZE_ENDPOINT,
            _ => GLOBAL_TOKEN_ENDPOINT,
        }
    }

    pub fn get_region_prefix(&self) -> &str {
        match self {
            AccountRegion::CN => "cn",
            AccountRegion::US => "us",
            AccountRegion::EU => "eu",
            AccountRegion::KR => "kr",
            AccountRegion::TW => "tw",
        }
    }
}
