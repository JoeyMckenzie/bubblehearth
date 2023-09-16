//! Region-based metadata for targeting specific instances of the Blizzard APIs.

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
