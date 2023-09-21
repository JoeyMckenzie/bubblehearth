//! Localities and internationalize data used to query Blizzard, currently supporting:
//!
//! ```text
//! English (United States) => en_US
//! English (Great Britain) => en_GB
//! Spanish (Mexico) => es_MX
//! Spanish (Spain) => es_ES
//! Portuguese => pt_BR
//! German => de_DE
//! French => fr_FR
//! Italian => it_IT
//! Russian => ru_RU
//! Korean => ko_KR
//! Chinese (Traditional) => zh_TW
//! Chinese (Simplified) => zh_CN
//!```

/// Localities supported by Blizzard's APIs.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Locale {
    /// Represents United States English locale.
    EnglishUS,
    /// Represents Great Britain English locale.
    EnglishGB,
    /// Represents Mexican Spanish locale.
    SpanishMX,
    /// Represents Spain-based Spanish locale.
    SpanishES,
    /// Represents Portuguese locale.
    Portuguese,
    /// Represents German locale.
    German,
    /// Represents French locale.
    French,
    /// Represents Italian locale.
    Italian,
    /// Represents Russian locale.
    Russian,
    /// Represents Korean locale.
    Korean,
    /// Represents Traditional Chinese locale.
    ChineseTW,
    /// Represents Simplified Chinese locale.
    ChineseCN,
}

impl Locale {
    /// Gets the corresponding locality code for the locale.
    pub fn get_locale(&self) -> &str {
        match self {
            Locale::EnglishUS => "en_US",
            Locale::EnglishGB => "en_GB",
            Locale::SpanishMX => "es_MX",
            Locale::SpanishES => "es_ES",
            Locale::Portuguese => "pt_BR",
            Locale::German => "de_DE",
            Locale::French => "fr_FR",
            Locale::Italian => "it_IT",
            Locale::Russian => "ru_RU",
            Locale::Korean => "ko_KR",
            Locale::ChineseTW => "zh_TW",
            Locale::ChineseCN => "zh_CN",
        }
    }
}
