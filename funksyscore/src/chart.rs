use crate::data::DataContext;
use serde::*;

/// The primary difficulty structure.
///
/// Difficulties are internally known as layer sets that include a certain
/// amount of layers.
#[derive(Deserialize, Serialize)]
pub struct LayerSet {
    filename: String,
    display_name: String,
    layers: Vec<Layer>,
}

impl DataContext for LayerSet {
    fn get_filename() -> Option<String> {
        Some("normal".to_string())
    }
    fn get_current_filename(self) -> String {
        self.filename
    }
}

#[derive(Deserialize, Serialize)]
pub struct Layer;
