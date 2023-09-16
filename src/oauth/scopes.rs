/// Scope for requesting World of Warcraft (including Classic) profile data.
pub const SCOPE_WOW_PROFILE: &str = "wow.profile";

/// Scope for requesting Starcraft 2 profile data.
pub const SCOPE_SC2_PROFILE: &str = "sc2.profile";

/// Scope for request Diablo 3 profile data.
pub const SCOPE_DIABLO_3_PROFILE: &str = "d3.profile";

/// Scope for request current user data.
pub const SCOPE_OPENID: &str = "openid";

/// Token scopes for request profile data from various Blizzard games.
#[derive(Debug, Clone, Copy)]
pub enum ProfileTokenScope {
    /// Represents the token scope for World of Warcraft (including Classic).
    WoW,
    /// Represents the token scope for Starcraft 2.
    Sc2,
    /// Represents the token scope for Diablo 3.
    D3,
    /// Represents the token scope for the current user profile.
    OpenID,
}

impl From<ProfileTokenScope> for String {
    fn from(value: ProfileTokenScope) -> Self {
        match value {
            ProfileTokenScope::WoW => SCOPE_WOW_PROFILE.to_string(),
            ProfileTokenScope::Sc2 => SCOPE_SC2_PROFILE.to_string(),
            ProfileTokenScope::D3 => SCOPE_DIABLO_3_PROFILE.to_string(),
            ProfileTokenScope::OpenID => SCOPE_OPENID.to_string(),
        }
    }
}
