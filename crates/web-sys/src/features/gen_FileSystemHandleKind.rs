#![allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `FileSystemHandleKind` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FileSystemHandleKind`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileSystemHandleKind {
    File = "file",
    Directory = "directory",
}
