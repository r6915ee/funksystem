//! Primary functionality for
//! [FunkSystem](https://codeberg.org/r6915ee/funksystem/) clients.
//!
//! This library provides the main functionality behind **FunkSystem**, a
//! client-based fangame engine for [Friday Night Funkin'](https://funkin.me/).
//!
//! # Description
//!
//! `funksyscore`'s main responsibility is to handle most FunkSystem-specific
//! information for both clients *and* additional utilities. What this means
//! is that provides a unified interface for each of its operations.
//!
//! # Usage
//!
//! Clients will typically use [`FunkSystemPlugin`] if they're made in
//! [Bevy](https://bevy.org/). This is a plugin that introduces particular
//! systems that are useful to any client following a similar structure
//! to the main client.
//!
//! Including it is as simple as:
//!
//! ```
//! use bevy::prelude::*;
//! use funksyscore::FunkSystemPlugin;
//!
//! fn main() {
//!     App::new().add_plugins((FunkSystemPlugin)).run();
//! }
//! ```

/// A module defining most graphics capabilities.
///
/// Graphics are handled through this module. This includes thing such as
/// [positioning](graphics::Position), and more.
pub mod graphics;

/// A module defining settings integration.
///
/// Settings are managed through this module. They are parsed through [Rusty
/// Object Notation](https://docs.rs/ron/), a JSON-like language.
pub mod data;

use bevy::prelude::*;
use data::*;
use log::info;
use std::io::ErrorKind;

/// Basic sample system.
pub fn hello_system() {
    info!("Hello, world!");
}

/// System responsible for creating save data.
pub fn save_system() {
    match generate_save_dir() {
        Ok(_) => (),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => (),
            ErrorKind::NotFound => panic!("{}", e),
            _ => (),
        },
    }

    let settings: Settings = Settings::read();
    settings.write().unwrap();

    info!(
        "Parsed settings: {}",
        ron::ser::to_string_pretty(&settings, ron::ser::PrettyConfig::default()).unwrap()
    );
}

/// Provides most systems directly.
///
/// `FunkSystemPlugin` is the primary plugin loaded by the game; that is,
/// it is a [Bevy](https://bevy.org/) plugin which offers most common
/// systems for the game.
pub struct FunkSystemPlugin;

impl Plugin for FunkSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hello_system);
        app.add_systems(Startup, save_system);
    }
}
