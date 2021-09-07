#![allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `FileSystemPermissionMode` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FileSystemPermissionMode`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileSystemPermissionMode {
    Read = "read",
    Readwrite = "readwrite",
}
