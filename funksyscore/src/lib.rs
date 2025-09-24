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

/// A module defining most data solutions.
///
/// General data is managed through this module. They are parsed through [Rusty
/// Object Notation](https://docs.rs/ron/), a JSON-like language.
pub mod data;

/// A module defining chart-specific data solutions.
///
/// Charts are the primary focus of `funksyscore` and act very differently
/// from other solutions.
///
/// They use the following terminology:
///
/// * **Steps**: Steps are the primary unit of measurement for charts, and
///   are typically one fourth of a beat.
/// * **Beats**: Beats are a secondary unit of measurement for charts, and
///   are typically one fourth of a measure.
/// * **Measures**: Measures are an additional unit of measurement for charts,
///   and are typically distributed on their own.
/// * **Hooks**: Hooks are assigned a certain step. When that step is reached,
///   then the hook is run, creating an entity with the hook's assigned
///   components at that point in time.
/// * **Layers**: Layers are assigned a certain set of hooks and may be
///   activated and deactivated at will.
/// * **Sets**: Typically identified in-game as *difficulties*, sets activate
///   and disable certain layers.
///
/// This system allows reusability, in contrast to other chart systems, and
/// exists to make use of the Entity Component System paradigm.
pub mod chart;

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
