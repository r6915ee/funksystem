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

#[derive(Deserialize, Serialize)]
pub struct Layer;
