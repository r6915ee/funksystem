//! Main [FunkSystem](https://codeberg.org/r6915ee/funksystem/) client.
//!
//! **FunkSystem** is a fangame engine for [Friday Night
//! Funkin'](https://funkin.me/) that relies on [Rust](https://rust-lang.org/)
//! to provide a memory-safe, practical modding experience. It also has a
//! client-based architecture that allows common engines to be built.
//!
//! This crate in specific is the main client, for which all libraries
//! stored in the monorepo are built for.
//!
//! # Usage
//!
//! This client is a complete, executable binary. There is no need to use it as
//! a library, since its setup doesn't allow this behavior.

use bevy::{log::LogPlugin, prelude::*};
use funksysbevy::FunkSystemPlugin;
use log::info;

/// Entry point for the program.
fn main() {
    clang_log::init(log::Level::Warn, "funksystem");
    info!("initializing game");
    App::new()
        .add_plugins((
            bevy_panic_handler::PanicHandler::new().build(),
            DefaultPlugins.build().disable::<LogPlugin>(),
            FunkSystemPlugin,
        ))
        .run();
}
