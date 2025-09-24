use dirs::config_dir;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fs,
    fs::{create_dir, read_to_string},
    io::{Error, ErrorKind, Result as IoResult},
    path::PathBuf,
};

/// Primary data trait, through [RON](https://docs.rs/ron/).
///
/// All data is handled through this trait, typically used as a supertrait.
/// It provides the methods necessary to properly use, serialize and
/// deserialize data easily.
pub trait DataContext: Serialize + DeserializeOwned {
    /// Get the assigned filename of the structure with this trait.
    fn get_filename() -> String;
}

/// Presents several methods to read and write from respective RON files.
///
/// In order to simplify most usages of data files, this trait is implemented
/// by structures that can be serialized and deserialize in order to allow
/// this.
pub trait SaveData: Default + DataContext {
    /// Reads the assigned file and copies it to the structure.
    ///
    /// If the data couldn't be copied for whatever reason, then this method
    /// will instead return the default structure, as allowed by [Default].
    fn read() -> Self
    where
        Self: DeserializeOwned,
    {
        match ron_path(Self::get_filename().as_str()) {
            Ok(path) => match read_to_string(path) {
                Ok(contents) => match ron::from_str::<Self>(contents.as_str()) {
                    Ok(data) => data,
                    Err(_) => Self::default(),
                },
                Err(_) => Self::default(),
            },
            Err(_) => Self::default(),
        }
    }
    /// Writes the structure's fields to the assigned file.
    ///
    /// The structure's data is serialized and then written to a file, based on
    /// [ron_path]'s output. If [ron_path] fails, then it will instead used the
    /// working directory.
    fn write(&self) -> IoResult<()>
    where
        Self: Serialize,
    {
        match fs::write(
            match ron_path(Self::get_filename().as_str()) {
                Ok(path) => path,
                Err(_) => PathBuf::from(Self::get_filename() + ".ron"),
            },
            ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()).unwrap(),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

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

impl DataContext for Settings {
    fn get_filename() -> String {
        "settings".to_string()
    }
}

impl SaveData for Settings {}

/// Convenience function for quickly generating the save directory.
pub fn generate_save_dir() -> IoResult<()> {
    match config_dir() {
        Some(path) => {
            let mut path: PathBuf = path;
            path.push(std::env::var("CARGO_PKG_NAME").unwrap_or("funksystem".to_string()));
            create_dir(&path)
        }
        None => IoResult::Err(Error::new(
            ErrorKind::NotFound,
            "global config directory not found",
        )),
    }
}

/// Convenience function for fetching the save directory.
pub fn fetch_save_dir() -> IoResult<PathBuf> {
    match config_dir() {
        Some(path) => {
            let mut path: PathBuf = path;
            path.push(std::env::var("CARGO_PKG_NAME").unwrap_or("funksystem".to_string()));
            match path.try_exists() {
                Ok(exists) => match exists {
                    true => Ok(path),
                    false => Err(Error::new(ErrorKind::NotFound, "save directory not found")),
                },
                Err(e) => Err(e),
            }
        }
        None => Err(Error::new(
            ErrorKind::NotFound,
            "global config directory not found",
        )),
    }
}

/// Generates a valid RON path.
///
/// This produces a valid RON path for either read or write operations to use.
/// It will fail if no save directory is available.
pub fn ron_path(filename: &str) -> IoResult<PathBuf> {
    match fetch_save_dir() {
        Ok(path) => {
            let mut path: PathBuf = path;
            path.push(filename);
            path.set_extension("ron");
            Ok(path)
        }
        Err(e) => Err(e),
    }
}
