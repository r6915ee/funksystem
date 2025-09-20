use bevy::prelude::*;

/// Provides backend functionality for Funk Aspect.
///
/// The `core` provides necessary modules that the entire game uses. The core
/// is necessary for even basic usage of the engine.
pub mod core;

use core::FunkAspectPlugin;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins((DefaultPlugins, FunkAspectPlugin))
        .run();
}
