#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = WritableStream , extends = :: js_sys :: Object , js_name = FileSystemWritableFileStream , typescript_type = "FileSystemWritableFileStream")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemWritableFileStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
    pub type FileSystemWritableFileStream;
    # [wasm_bindgen (method , structural , js_class = "FileSystemWritableFileStream" , js_name = seek)]
    #[doc = "The `seek()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/seek)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
    pub fn seek_with_u32(this: &FileSystemWritableFileStream, position: u32) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "FileSystemWritableFileStream" , js_name = seek)]
    #[doc = "The `seek()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/seek)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
    pub fn seek_with_f64(this: &FileSystemWritableFileStream, position: f64) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "FileSystemWritableFileStream" , js_name = truncate)]
    #[doc = "The `truncate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/truncate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
    pub fn truncate_with_u32(this: &FileSystemWritableFileStream, size: u32) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "FileSystemWritableFileStream" , js_name = truncate)]
    #[doc = "The `truncate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/truncate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
    pub fn truncate_with_f64(this: &FileSystemWritableFileStream, size: f64) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "FileSystemWritableFileStream" , js_name = write)]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
    pub fn write_with_buffer_source(
        this: &FileSystemWritableFileStream,
        data: &::js_sys::Object,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "FileSystemWritableFileStream" , js_name = write)]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
    pub fn write_with_u8_array(
        this: &FileSystemWritableFileStream,
        data: &mut [u8],
    ) -> ::js_sys::Promise;
    #[cfg(feature = "Blob")]
    # [wasm_bindgen (method , structural , js_class = "FileSystemWritableFileStream" , js_name = write)]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileSystemWritableFileStream`*"]
    pub fn write_with_blob(this: &FileSystemWritableFileStream, data: &Blob) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "FileSystemWritableFileStream" , js_name = write)]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
    pub fn write_with_str(this: &FileSystemWritableFileStream, data: &str) -> ::js_sys::Promise;
    #[cfg(feature = "WriteParams")]
    # [wasm_bindgen (method , structural , js_class = "FileSystemWritableFileStream" , js_name = write)]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`, `WriteParams`*"]
    pub fn write_with_write_params(
        this: &FileSystemWritableFileStream,
        data: &WriteParams,
    ) -> ::js_sys::Promise;
}
