BubbleHearth: Rust Blizzard Game Data API Wrapper

License
Overview

BubbleHearth is a Rust library designed to simplify the interaction with Blizzard's Game Data APIs. This library enables
developers to access a wide range of game-related data from titles like World of Warcraft, Diablo, Overwatch, and more.
Whether you're building an application, a bot, or analyzing game statistics, BubbleHearth provides an intuitive
interface to fetch and work with Blizzard game data seamlessly.
Features

    Easy Integration: Integrate Blizzard Game Data into your Rust projects effortlessly using BubbleHearth's well-structured API.

    Multi-Game Support: Fetch data for various Blizzard games within a single Rust library.

    Detailed Documentation: Extensive documentation with usage examples to help you get started quickly.

Getting Started
Prerequisites

Before using BubbleHearth, ensure you have the following prerequisites:

    Rust programming language and Cargo package manager installed.

Installation

To include BubbleHearth in your Rust project, add it as a dependency in your Cargo.toml:

toml

[dependencies]
bubblehearth = "0.1"

Usage

Here's a simple example of how to use BubbleHearth to retrieve character information from World of Warcraft:

rust

extern crate bubblehearth;

use bubblehearth::{BlizzardApi, Region, Game};
use std::env;

fn main() {
// Set your Blizzard API key as an environment variable.
let api_key = env::var("BLIZZARD_API_KEY").expect("BLIZZARD_API_KEY not set");

    // Create a BlizzardApi instance for the US region.
    let blizzard_api = BlizzardApi::new(&api_key, Region::US);

    // Fetch character information for a specific character in WoW.
    let character_info = blizzard_api.get_character_info(Game::Wow, "realm_name", "character_name");

    // Process character_info as needed.
    println!("{:?}", character_info);

}

Documentation

For comprehensive usage instructions and more code examples, please refer to the documentation.
Contributing

We welcome contributions to BubbleHearth! If you find issues, have suggestions, or want to contribute code, please open
an issue or submit a pull request.
License

BubbleHearth is open-source and licensed under the MIT License. See the LICENSE file for details.
Acknowledgments

    This library is a community-driven project and is not officially endorsed or supported by Blizzard Entertainment.

    We would like to express our gratitude to the Rust community for their continuous support and contributions.

This README provides an overview of the "BubbleHearth" Rust library, its features, installation instructions, usage
examples, contribution guidelines, and licensing information. Make sure to replace placeholder URLs with actual links,
and customize it with specific API usage details and any other relevant information for your library.
