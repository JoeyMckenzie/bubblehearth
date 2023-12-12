//! Common Blizzard HTTP interactions and downstream calls, leveraging [reqwest](https://docs.rs/reqwest/latest/reqwest/) for
//! connecting to Blizzard API endpoints.

#![forbid(unsafe_code)]
#![warn(
    dead_code,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_allocation,
    trivial_numeric_casts,
    clippy::single_char_pattern
)]

type BubbleHearthId = u32;

mod auth;
mod builder;
pub mod classic;
pub mod client;
mod connectors;
mod documents;
pub mod errors;
pub mod hearthstone;
pub mod localization;
pub mod regionality;
pub mod search;
pub mod timezone;
