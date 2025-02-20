#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DirectoryPickerOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DirectoryPickerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`*"]
    pub type DirectoryPickerOptions;
}
impl DirectoryPickerOptions {
    #[doc = "Construct a new `DirectoryPickerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("id"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `startIn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`*"]
    pub fn start_in(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("startIn"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for DirectoryPickerOptions {
    fn default() -> Self {
        Self::new()
    }
}
