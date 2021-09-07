#![allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `WellKnownDirectory` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WellKnownDirectory`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WellKnownDirectory {
    Desktop = "desktop",
    Documents = "documents",
    Downloads = "downloads",
    Music = "music",
    Pictures = "pictures",
    Videos = "videos",
}
