//! Unix-based timezones for search queries.

/// Timezones for various endpoints, primarily for used for searching.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Timezone {
    /// Represents the America/Los_Angeles western timezone.
    AmericaLosAngeles,
    /// Represents the America/New_York eastern timezone.
    AmericaNewYork,
}

impl From<Timezone> for String {
    fn from(timezone: Timezone) -> Self {
        let zone = match timezone {
            Timezone::AmericaLosAngeles => "America/Los_Angeles",
            Timezone::AmericaNewYork => "America/New_York",
        };

        zone.to_string()
    }
}
