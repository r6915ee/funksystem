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

/// Basic sample system.
pub fn hello_system() {
    println!("Hello, world!");
}

/// Provides most systems and components directly.
///
/// `FunkAspectPlugin` is the primary plugin loaded by the game; that is,
/// it is a [Bevy](https://bevy.org/) plugin which offers most common
/// systems and components for the game.
pub struct FunkAspectPlugin;

impl Plugin for FunkAspectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hello_system);
    }
}
