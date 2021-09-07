#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FilePickerAcceptType)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FilePickerAcceptType` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerAcceptType`*"]
    pub type FilePickerAcceptType;
}
impl FilePickerAcceptType {
    #[doc = "Construct a new `FilePickerAcceptType`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerAcceptType`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerAcceptType`*"]
    pub fn description(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("description"),
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
impl Default for FilePickerAcceptType {
    fn default() -> Self {
        Self::new()
    }
}
