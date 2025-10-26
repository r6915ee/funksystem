//! Primary functionality for
//! [FunkSystem](https://codeberg.org/r6915ee/funksystem/) clients.
//!
//! This library provides the main functionality behind **FunkSystem**, a
//! client-based fangame engine for [Friday Night Funkin'](https://funkin.me/).
//!
//! # Description
//!
//! `funksyscore`'s main responsibility is to handle most FunkSystem-specific
//! information for both clients *and* additional utilities. What this means
//! is that provides a unified interface for each of its operations.
//!
//! # Usage
//!
//! An example of using the library is, for example, modifying save data:
//!
//! ```
//! use funksyscore::data::Settings;
//! use funksyscore::data::SaveData;
//!
//! let settings: Settings = Settings::read(".".into());
//! settings.write(".".into());
//! ```
//!
//! Each couple of lines does something different.
//!
//! ```
//! use funksyscore::data::Settings;
//! use funksyscore::data::SaveData;
//! #
//! # let settings: Settings = Settings::read(".".into());
//! # settings.write(".".into());
//! ```
//!
//! [`Settings`](data::Settings) is the primary structure that is responsible
//! for settings configuration. [`SaveData`](data::SaveData) is a trait used
//! by [`Settings`](data::Settings) that implements two methods,
//! [`read`](data::SaveData::read) and [`write`](data::SaveData::write).
//!
//! ```
//! # use funksyscore::data::Settings;
//! # use funksyscore::data::SaveData;
//! #
//! let settings: Settings = Settings::read(".".into());
//! # settings.write(".".into());
//! ```
//!
//! [`read`](data::SaveData::read) uses the first parameter as a
//! [`PathBuf`](std::path::PathBuf), and then searches for a specific file
//! under that path. If it's valid [RON](https://docs.rs/ron/) data - the
//! primary language for data in `funksyscore` - then it returns the
//! deserialized structure. Otherwise, it will return a default structure.
//!
//! In this case, it will attempt to find the file in the working directory.
//!
//! ```
//! # use funksyscore::data::Settings;
//! # use funksyscore::data::SaveData;
//! #
//! # let settings: Settings = Settings::read(".".into());
//! settings.write(".".into());
//! ```
//!
//! [`write`](data::SaveData::write) uses the same parameter as the previous
//! method. However, it will instead attempt to write to the location searched
//! for in its reading counterpart.
//!
//! All implementations of [`SaveData`](data::SaveData) expose these methods to
//! simplify the usage of save data in general, due to the multi-file system of
//! save data in `funksyscore`.

/// A module defining chart-specific data solutions.
///
/// Charts are the primary focus of `funksyscore` and act very differently
/// from other solutions.
///
/// They use the following terminology:
///
/// * **Steps**: Steps are the primary unit of measurement for charts, and
///   are typically one fourth of a beat.
/// * **Beats**: Beats are a secondary unit of measurement for charts, and
///   are typically one fourth of a measure.
/// * **Measures**: Measures are an additional unit of measurement for charts,
///   and are typically distributed on their own.
/// * **Hooks**: Hooks are assigned a certain step. When that step is reached,
///   then the hook is run, being used in conjunction with the other hooks at
///   that step; this is typically used to create an entity.
/// * **Layers**: Layers are assigned a certain set of hooks and may be
///   activated and deactivated at will.
/// * **Sets**: Typically identified in-game as *difficulties*, sets activate
///   and disable certain layers.
///
/// This system allows reusability, in contrast to other chart systems, and
/// exists to make use of the Entity Component System paradigm.
pub mod chart;
