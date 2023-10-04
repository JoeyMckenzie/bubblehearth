//! Unix-based timezones for search queries.

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

/// Timezones for various endpoints, primarily for used for searching.
#[derive(Debug, Copy, Clone, Serialize, Eq, PartialEq)]
pub enum Timezone {
    /// Represents the America/Los_Angeles western timezone.
    AmericaLosAngeles,
    /// Represents the America/New_York eastern timezone.
    AmericaNewYork,
    /// Represents an unknown, detected when the provided timezone is not able to be parsed.
    Unknown,
}

impl From<Timezone> for String {
    fn from(timezone: Timezone) -> Self {
        let zone = match timezone {
            Timezone::AmericaLosAngeles => "America/Los_Angeles",
            Timezone::AmericaNewYork => "America/New_York",
            _ => "Unknown",
        };

        zone.to_string()
    }
}

impl From<String> for Timezone {
    fn from(value: String) -> Self {
        match value.as_str() {
            "America/Los_Angeles" => Timezone::AmericaLosAngeles,
            "America/New_York" => Timezone::AmericaNewYork,
            _ => Timezone::Unknown,
        }
    }
}

impl<'de> Deserialize<'de> for Timezone {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json_value: Value = Deserialize::deserialize(deserializer)?;

        match json_value {
            Value::String(s) => {
                let parsed_timezone: Timezone = s.into();
                Ok(parsed_timezone)
            }
            _ => Err(serde::de::Error::custom("Invalid data type for locale.")),
        }
    }
}

#[cfg(test)]
mod timezone_tests {
    use serde::Deserialize;

    use crate::timezone::Timezone;

    #[derive(Deserialize)]
    struct TestStruct {
        timezone: Timezone,
    }

    #[test]
    fn returns_timezone_when_match_found() {
        // Arrange
        let data = r#"
        {
            "timezone": "America/Los_Angeles"
        }
        "#;

        // Act
        let timezone_struct: TestStruct = serde_json::from_str(data).unwrap();

        // Assert
        assert_eq!(timezone_struct.timezone, Timezone::AmericaLosAngeles);
    }

    #[test]
    fn returns_timezone_unknown_when_no_match_found() {
        // Arrange
        let data = r#"
        {
            "timezone": "Unrecognized_timezone"
        }
        "#;

        // Act
        let timezone_struct: TestStruct = serde_json::from_str(data).unwrap();

        // Assert
        assert_eq!(timezone_struct.timezone, Timezone::Unknown);
    }
}
