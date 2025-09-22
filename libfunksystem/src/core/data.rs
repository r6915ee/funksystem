use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::{fs::create_dir, io, path::PathBuf};

/// Defines all valid keybindings.
#[derive(Debug, Deserialize, Serialize)]
pub struct Keybindings {
    /// UI's left control.
    pub ui_left: char,
    /// UI's down control.
    pub ui_down: char,
    /// UI's up control.
    pub ui_up: char,
    /// UI's right control.
    pub ui_right: char,
    /// Gameplay's left control.
    pub note_left: char,
    /// Gameplay's down control.
    pub note_down: char,
    /// Gameplay's up control.
    pub note_up: char,
    /// Gameplay's right control.
    pub note_right: char,
}

impl Default for Keybindings {
    fn default() -> Keybindings {
        Keybindings {
            ui_left: 'a',
            ui_down: 's',
            ui_up: 'w',
            ui_right: 'd',
            note_left: 'a',
            note_down: 's',
            note_up: 'w',
            note_right: 'd',
        }
    }
}

/// A structure defining all valid settings.
///
/// Settings are managed through this simple structure. It contains things
/// such as [keybinding information](Keybindings), window settings, and more.
#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    /// Defines valid controls.
    pub controls: Keybindings,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            controls: Keybindings::default(),
        }
    }
}

/// Convenience function for quickly generating the save directory.
pub fn generate_save_dir() -> io::Result<()> {
    match config_dir() {
        Some(path) => {
            let mut path: PathBuf = path;
            path.push(env!("CARGO_CRATE_NAME"));
            create_dir(&path)
        }
        None => io::Result::Err(io::Error::new(
            io::ErrorKind::NotFound,
            "global config directory not found",
        )),
    }
}

/// Convenience function for fetching the save directory.
pub fn fetch_save_dir() -> io::Result<Option<PathBuf>> {
    match config_dir() {
        Some(path) => {
            let mut path: PathBuf = path;
            path.push(env!("CARGO_CRATE_NAME"));
            match path.try_exists() {
                Ok(exists) => match exists {
                    true => Ok(Some(path)),
                    false => Ok(None),
                },
                Err(e) => Err(e),
            }
        }
        None => io::Result::Err(io::Error::new(
            io::ErrorKind::NotFound,
            "global config directory not found",
        )),
    }
}

/// Fetches a certain RON path in the config directory.
pub fn get_ron(data: &str) -> io::Result<Option<PathBuf>> {
    match fetch_save_dir() {
        Ok(context) => match context {
            Some(path) => {
                let mut path: PathBuf = path;
                path.push(data);
                path.set_extension("ron");

                match path.try_exists() {
                    Ok(exists) => match exists {
                        true => Ok(Some(path)),
                        false => Ok(None),
                    },
                    Err(e) => Err(e),
                }
            }
            None => Ok(None),
        },
        Err(e) => Err(e),
    }
}

/// Reads the save data, and returns its output as [Settings].
///
/// Do note that this function in particular may construct the default values
/// for [Settings] if the save data cannot be read.
pub fn read_settings() -> Settings {
    match get_ron("settings") {
        Ok(context) => match context {
            Some(path) => match std::fs::read_to_string(path) {
                Ok(content) => {
                    let settings: Settings = match ron::from_str(content.as_str()) {
                        Ok(content) => content,
                        Err(_) => Settings::default(),
                    };
                    settings
                }
                Err(_) => Settings::default(),
            },
            None => Settings::default(),
        },
        Err(_) => Settings::default(),
    }
}
