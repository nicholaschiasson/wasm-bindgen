#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FileSystemHandlePermissionDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemHandlePermissionDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandlePermissionDescriptor`*"]
    pub type FileSystemHandlePermissionDescriptor;
}
impl FileSystemHandlePermissionDescriptor {
    #[doc = "Construct a new `FileSystemHandlePermissionDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandlePermissionDescriptor`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "FileSystemPermissionMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandlePermissionDescriptor`, `FileSystemPermissionMode`*"]
    pub fn mode(&mut self, val: FileSystemPermissionMode) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("mode"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for FileSystemHandlePermissionDescriptor {
    fn default() -> Self {
        Self::new()
    }
}
