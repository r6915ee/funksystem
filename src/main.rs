//! Main [FunkSystem](https://codeberg.org/r6915ee/funksystem/) crate.
//!
//! **FunkSystem** is a fangame engine for [Friday Night
//! Funkin'](https://funkin.me/) that relies on [Rust](https://rust-lang.org/)
//! and [Bevy](https://bevy.org/) to provide a memory-safe, practical modding
//! experience. It also has a client-based architecture that allows common
//! engines to be built.

/// Provides backend functionality for FunkSystem.
///
/// The `core` provides necessary modules that the entire game uses. The core
/// is necessary for even basic usage of the engine.
pub mod core;

use bevy::prelude::*;
use core::FunkAspectPlugin;

/// Entry point for the program.
fn main() {
    App::new()
        .add_plugins((
            bevy_panic_handler::PanicHandler::new().build(),
            DefaultPlugins,
            FunkAspectPlugin,
        ))
        .run();
}
