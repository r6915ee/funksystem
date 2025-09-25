use bevy::prelude::*;
use funksyscore::data::*;
use log::warn;
use std::{env::var, fs::create_dir_all, path::PathBuf};

/// Convenience function for getting the configuration directory.
///
/// See [dirs::config_dir] for more details.
fn config_dir() -> PathBuf {
    // The system will typically include the configuration directory. If it
    // doesn't, then the program needs to panic, since it is necessary for
    // safe operation.
    let mut config_path: PathBuf = dirs::config_dir().unwrap();
    config_path.push(var("CARGO_PKG_NAME").unwrap());
    create_dir_all(&config_path).unwrap();

    config_path
}

/// Basic sample system.
pub fn hello_system() {
    info!("Hello, world!");
}

/// System responsible for creating save data.
pub fn save_system() {
    let config_path: PathBuf = config_dir();

    let settings: Settings = Settings::read(config_path.to_owned());
    let _ = settings.write(config_path.to_owned());

    warn!(
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
