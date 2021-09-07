#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FileSystemPermissionDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemPermissionDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemPermissionDescriptor`*"]
    pub type FileSystemPermissionDescriptor;
}
impl FileSystemPermissionDescriptor {
    #[cfg(all(feature = "FileSystemHandle", feature = "PermissionName",))]
    #[doc = "Construct a new `FileSystemPermissionDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`, `FileSystemPermissionDescriptor`, `PermissionName`*"]
    pub fn new(name: PermissionName, handle: &FileSystemHandle) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.handle(handle);
        ret
    }
    #[cfg(feature = "PermissionName")]
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemPermissionDescriptor`, `PermissionName`*"]
    pub fn name(&mut self, val: PermissionName) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("name"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "FileSystemHandle")]
    #[doc = "Change the `handle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`, `FileSystemPermissionDescriptor`*"]
    pub fn handle(&mut self, val: &FileSystemHandle) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("handle"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "FileSystemPermissionMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemPermissionDescriptor`, `FileSystemPermissionMode`*"]
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
