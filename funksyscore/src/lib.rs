//! Primary functionality for
//! [FunkSystem](https://codeberg.org/r6915ee/funksystem/) clients.
//!
//! This library provides the main functionality behind **FunkSystem**, a
//! client-based fangame engine for [Friday Night Funkin'](https://funkin.me/).

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
use std::io::ErrorKind;
use log::info;

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

/// Provides most systems and components directly.
///
/// `FunkSystemPlugin` is the primary plugin loaded by the game; that is,
/// it is a [Bevy](https://bevy.org/) plugin which offers most common
/// systems and components for the game.
pub struct FunkSystemPlugin;

impl Plugin for FunkSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hello_system);
        app.add_systems(Startup, save_system);
    }
}
