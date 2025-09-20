//! Main [Funk Aspect](https://codeberg.org/r6915ee/funk-aspect/) crate.
//!
//! **Funk Aspect** is a fangame engine for [Friday Night
//! Funkin'](https://funkin.me/) that relies on [Rust](https://rust-lang.org/)
//! and [Bevy](https://bevy.org/) to provide a memory-safe, practical modding
//! experience.

use bevy::prelude::*;

/// Provides backend functionality for Funk Aspect.
///
/// The `core` provides necessary modules that the entire game uses. The core
/// is necessary for even basic usage of the engine.
pub mod core;

use core::FunkAspectPlugin;

/// Entry point for the program.
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FunkAspectPlugin))
        .run();
}
