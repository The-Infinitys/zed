use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

#[with_fallible_options]
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]

pub struct BackgroundImageSettingsContent {
    /// Path to the background image file.
    pub path: Option<String>,
    /// Opacity of the background image.
    pub opacity: Option<f32>,
}
