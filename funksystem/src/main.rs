//! Main [FunkSystem](https://codeberg.org/r6915ee/funksystem/) client.
//!
//! **FunkSystem** is a fangame engine for [Friday Night
//! Funkin'](https://funkin.me/) that relies on [Rust](https://rust-lang.org/)
//! and [Bevy](https://bevy.org/) to provide a memory-safe, practical modding
//! experience. It also has a client-based architecture that allows common
//! engines to be built.

use bevy::prelude::*;
use funksyscore::FunkSystemPlugin;

/// Entry point for the program.
fn main() {
    clang_log::init_error(log::Level::Warn, log::Level::Error, "funksystem");
    App::new()
        .add_plugins((
            bevy_panic_handler::PanicHandler::new().build(),
            DefaultPlugins,
            FunkSystemPlugin,
        ))
        .run();
}
