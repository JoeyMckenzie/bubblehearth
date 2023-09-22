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

use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::errors::BubbleHearthError;

/// A localization response model that can be deserialized as a string or struct.
#[derive(Debug, Clone, PartialEq)]
pub enum StringOrStructLocale {
    /// String-based locale.
    StringLocale(String),
    /// Struct-map based locale.
    StructLocale(Box<StructuredLocale>),
}

/// Localities supported by Blizzard's APIs.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize)]
pub enum Locale {
    /// Represents United States English locale.
    #[serde(rename = "en_US")]
    EnglishUS,
    /// Represents Great Britain English locale.
    #[serde(rename = "en_GB")]
    EnglishGB,
    /// Represents Mexican Spanish locale.
    #[serde(rename = "es_MX")]
    SpanishMX,
    /// Represents Spain-based Spanish locale.
    #[serde(rename = "es_ES")]
    SpanishES,
    /// Represents Portuguese locale.
    #[serde(rename = "pt_BR")]
    Portuguese,
    /// Represents German locale.
    #[serde(rename = "de_DE")]
    German,
    /// Represents French locale.
    #[serde(rename = "fr_FR")]
    French,
    /// Represents Italian locale.
    #[serde(rename = "it_IT")]
    Italian,
    /// Represents Russian locale.
    #[serde(rename = "ru_RU")]
    Russian,
    /// Represents Korean locale.
    #[serde(rename = "ko_KR")]
    Korean,
    /// Represents Traditional Chinese locale.
    #[serde(rename = "zh_TW")]
    ChineseTW,
    /// Represents Simplified Chinese locale.
    #[serde(rename = "zh_CN")]
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

/// A struct-based version of the locale, useful for deserializing the
/// JSON value returned from Blizzard into the typed enum locale variant.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StructuredLocale {
    /// Represents United States English locale.
    #[serde(rename = "en_US")]
    pub en_us: String,
    /// Represents Great Britain English locale.
    #[serde(rename = "en_GB")]
    pub en_gb: String,
    /// Represents Mexican Spanish locale.
    #[serde(rename = "es_MX")]
    pub es_mx: String,
    /// Represents Spain-based Spanish locale.
    #[serde(rename = "es_ES")]
    pub es_es: String,
    /// Represents Portuguese locale.
    #[serde(rename = "pt_BR")]
    pub pt_br: String,
    /// Represents German locale.
    #[serde(rename = "de_DE")]
    pub de_de: String,
    /// Represents French locale.
    #[serde(rename = "fr_FR")]
    pub fr_fr: String,
    /// Represents Italian locale.
    #[serde(rename = "it_IT")]
    pub it_it: String,
    /// Represents Russian locale.
    #[serde(rename = "ru_RU")]
    pub ru_ru: String,
    /// Represents Korean locale.
    #[serde(rename = "ko_KR")]
    pub ko_kr: String,
    /// Represents Traditional Chinese locale.
    #[serde(rename = "zh_TW")]
    pub zh_tw: String,
    /// Represents Simplified Chinese locale.
    #[serde(rename = "zh_CN")]
    pub zh_cn: String,
}

/// Maps a locale returned by Blizzard into a typed locale. Primarily
/// used for mapping string-based locales returned for various items.
impl FromStr for Locale {
    type Err = BubbleHearthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en_US" => Ok(Locale::EnglishUS),
            "en_GB" => Ok(Locale::EnglishGB),
            "es_MX" => Ok(Locale::SpanishMX),
            "es_ES" => Ok(Locale::SpanishES),
            "pt_BR" => Ok(Locale::Portuguese),
            "de_DE" => Ok(Locale::German),
            "fr_FR" => Ok(Locale::French),
            "it_IT" => Ok(Locale::Italian),
            "ru_RU" => Ok(Locale::Russian),
            "ko_KR" => Ok(Locale::Korean),
            "zh_TW" => Ok(Locale::ChineseTW),
            "zh_CN" => Ok(Locale::ChineseCN),
            &_ => Err(BubbleHearthError::LocaleUnknown(s.to_string())),
        }
    }
}

/// Allows dynamic serialization based on the locale data returned from an item.
/// For certain endpoints, Blizzard may return either a string for the locale
/// or an object. Taking realms, as an example, when retrieving one or many
/// without specifying a locale wil return the following:
///
/// ```json
/// {
///   "realms": [
///     {
///       "key": {
///         "href": "https://us.api.blizzard.com/data/wow/realm/4372?namespace=dynamic-classic-us"
///       },
///       "name": {
///         "en_US": "Atiesh",
///         "es_MX": "Atiesh",
///         "pt_BR": "Atiesh",
///         "de_DE": "Atiesh",
///         "en_GB": "Atiesh",
///         "es_ES": "Atiesh",
///         "fr_FR": "Atiesh",
///         "it_IT": "Atiesh",
///         "ru_RU": "Atiesh",
///         "ko_KR": "Atiesh",
///         "zh_TW": "阿泰絲",
///         "zh_CN": "埃提耶什"
///       },
///       "id": 4372,
///       "slug": "atiesh"
///     }
///   ]
/// }
/// ```
///
/// Without specifying a locale, we can also receive:
///
/// ```json
/// {
///   "realms": [
///     {
///       "key": {
///         "href": "https://us.api.blizzard.com/data/wow/realm/4372?namespace=dynamic-classic-us"
///       },
///       "name": "Atiesh",
///       "id": 4372,
///       "slug": "atiesh"
///     }
///   ]
/// }
/// ```
impl<'de> Deserialize<'de> for StringOrStructLocale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json_value: Value = Deserialize::deserialize(deserializer)?;

        match json_value {
            Value::String(s) => Ok(StringOrStructLocale::StringLocale(s)),
            Value::Object(_) => {
                dbg!(&json_value);
                let locale_struct: StructuredLocale = Deserialize::deserialize(json_value).unwrap();
                Ok(StringOrStructLocale::StructLocale(Box::new(locale_struct)))
            }
            _ => Err(serde::de::Error::custom("Invalid data type for locale.")),
        }
    }
}
