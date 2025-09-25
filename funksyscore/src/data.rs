use log::warn;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fs, fs::read_to_string, io::Result as IoResult, path::PathBuf};

/// Primary data trait, through [RON](https://docs.rs/ron/).
///
/// All data is handled through this trait, typically used as a supertrait.
/// It provides a few shortcut methods for implementations to use.
pub trait DataContext: Serialize + DeserializeOwned {
    /// Get the assigned filename of the structure with this trait.
    ///
    /// This will return an [Option] that contains a [String] if it's valid.
    /// Most implementations will typically use a static, compile-time only
    /// value, that unfortunately others may not even support, hence the usage
    /// of [Option] as part of the return value.
    ///
    /// If you want to get a runtime-only filename that is dependent on the
    /// instantiated structure, then use
    /// [get_current_filename](DataContext::get_current_filename).
    fn get_filename() -> Option<String>;
    /// Gets the current instance's filename.
    ///
    /// This is a runtime-dependent variant of
    /// [get_filename](DataContext::get_filename) that is dependent on the
    /// current instance of the structure. It will always return a [String],
    /// however, unlike its compile-time variant.
    ///
    /// Do note that the default implementation uses
    /// [get_filename](DataContext::get_filename) and expects it to return a
    /// [`Some`](Option::Some). If the behavior of that implementation
    /// contradicts this requirement, then make sure to provide the
    /// implementation of this method yourself. Otherwise, the method will
    /// panic.
    fn get_current_filename(self) -> String {
        match Self::get_filename() {
            Some(str) => str,
            None => {
                panic!(
                    "DataContext::get_current_filename's default \
                    implementation doesn't work when DataContext:;get_filename \
                    returns None; please provide a \
                    DataContext::get_current_filename implementation"
                );
            }
        }
    }
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
    fn read(path: PathBuf) -> Self {
        match read_to_string(ron_path(
            path.to_owned(),
            Self::get_filename().unwrap().as_str(),
        )) {
            Ok(contents) => match ron::from_str::<Self>(contents.as_str()) {
                Ok(data) => data,
                Err(_) => {
                    warn!(
                        "{}.ron is not valid in location {}, assuming Default; \
                            file may be overwritten at a later point",
                        Self::get_filename().unwrap(),
                        path.display()
                    );
                    Self::default()
                }
            },
            Err(_) => {
                let store: Self = Self::default();
                let _ = store.write(path);
                store
            }
        }
    }
    /// Writes the structure's fields to the assigned file.
    ///
    /// The structure's data is serialized and then written to a file, based on
    /// [ron_path]'s output. If [ron_path] fails, then it will instead used the
    /// working directory.
    fn write(&self, path: PathBuf) -> IoResult<()> {
        match fs::write(
            ron_path(path, Self::get_filename().unwrap().as_str()),
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
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Settings {
    /// Defines valid controls.
    pub controls: Keybindings,
}

impl DataContext for Settings {
    fn get_filename() -> Option<String> {
        Some("settings".to_string())
    }
    fn get_current_filename(self) -> String {
        Settings::get_filename().unwrap()
    }
}

impl SaveData for Settings {}

/// Generates a valid RON path.
///
/// This produces a valid RON path for either read or write operations to use.
/// It will fail if no save directory is available.
pub fn ron_path(mut path: PathBuf, filename: &str) -> PathBuf {
    path.push(filename);
    path.set_extension("ron");
    path
}
