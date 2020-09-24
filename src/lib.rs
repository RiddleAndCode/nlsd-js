mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn to_string(val: JsValue) -> JsValue {
    set_panic_hook();
    let val = val.into_serde::<serde_json::Value>().unwrap_throw();
    nlsd_core::to_string(&val).unwrap_throw().into()
}

#[wasm_bindgen]
pub fn from_string(val: JsValue) -> JsValue {
    set_panic_hook();
    let string = val.as_string().expect_throw("Argument must be a string");
    let val: serde_json::Value = nlsd_core::from_str(&string).unwrap_throw();
    JsValue::from_serde(&val).unwrap_throw()
}
