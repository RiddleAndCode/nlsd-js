#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use serde_json::json;

macro_rules! js {
    ($($contents:tt)*) => {
        JsValue::from_serde(&json!($($contents)*)).unwrap_throw()
    }
}

macro_rules! to_string {
    ($($contents:tt)*) => {
        nlsd::to_string(js!($($contents)*))
    };
}

macro_rules! assert_serde_eq {
    ($($contents:tt)*) => {
        assert_eq!(from_string(&to_string!($($contents)*).as_string().unwrap_throw()), json!($($contents)*))
    };
}

fn from_string(string: &str) -> serde_json::Value {
    nlsd::from_string(JsValue::from_str(string))
        .into_serde()
        .unwrap_throw()
}

#[wasm_bindgen_test]
fn ser_none() {
    assert_eq!(to_string!(null), "empty");
}

#[wasm_bindgen_test]
fn ser_bool() {
    assert_eq!(to_string!(true), "true");
    assert_eq!(to_string!(false), "false");
}

#[wasm_bindgen_test]
fn ser_string() {
    assert_eq!(to_string!("hello"), "`hello`");
}

#[wasm_bindgen_test]
fn ser_num() {
    assert_eq!(to_string!(1), "1");
    assert_eq!(to_string!(1.2), "1.2");
    assert_eq!(to_string!(-123), "-123");
}

#[wasm_bindgen_test]
fn ser_list() {
    assert_eq!(
        to_string!([1, 2, 3]),
        "the list where an item is 1 and another item is 2 and another item is 3"
    );
}

#[wasm_bindgen_test]
fn ser_object() {
    assert_eq!(
        to_string!({"a": 1, "b": 2}),
        "the object where `a` is 1 and `b` is 2"
    );
}

#[wasm_bindgen_test]
fn ser_nested() {
    assert_eq!(to_string!([{"a": 1}, {"b": 2}]) , "the list henceforth `the list` where an item is the object where `a` is 1 and another item of `the list` is the object where `b` is 2");
    assert_eq!(to_string!({"a": [1], "b": [2]}) , "the object henceforth `the object` where `a` is the list where an item is 1 and `b` of `the object` is the list where an item is 2");
}

#[wasm_bindgen_test]
fn de_none() {
    assert_eq!(json!(null), from_string("empty"));
}

#[wasm_bindgen_test]
fn de_bool() {
    assert_eq!(json!(true), from_string("true"));
    assert_eq!(json!(false), from_string("false"));
}

#[wasm_bindgen_test]
fn de_string() {
    assert_eq!(json!("hello"), from_string("`hello`"));
}

#[wasm_bindgen_test]
fn de_num() {
    assert_eq!(json!(1), from_string("1"));
    assert_eq!(json!(1.2), from_string("1.2"));
    assert_eq!(json!(-123), from_string("-123"));
}

#[wasm_bindgen_test]
fn de_list() {
    assert_eq!(
        json!([1, 2, 3]),
        from_string("the list where an item is 1 and another item is 2 and another item is 3")
    );
}

#[wasm_bindgen_test]
fn de_object() {
    assert_eq!(
        json!({"a": 1, "b": 2}),
        from_string("the object where `a` is 1 and `b` is 2")
    );
}

#[wasm_bindgen_test]
fn de_nested() {
    assert_eq!(json!([{"a": 1}, {"b": 2}]) , from_string("the list henceforth `the list` where an item is the object where `a` is 1 and another item of `the list` is the object where `b` is 2"));
    assert_eq!(json!({"a": [1], "b": [2]}) , from_string("the object henceforth `the object` where `a` is the list where an item is 1 and `b` of `the object` is the list where an item is 2"));
}

#[wasm_bindgen_test]
fn serde_none() {
    assert_serde_eq!(null);
}

#[wasm_bindgen_test]
fn serde_bool() {
    assert_serde_eq!(true);
    assert_serde_eq!(false);
}

#[wasm_bindgen_test]
fn serde_string() {
    assert_serde_eq!("hello");
}

#[wasm_bindgen_test]
fn serde_num() {
    assert_serde_eq!(1);
    assert_serde_eq!(1.2);
    assert_serde_eq!(-123);
}

#[wasm_bindgen_test]
fn serde_list() {
    assert_serde_eq!([1, 2, 3]);
}

#[wasm_bindgen_test]
fn serde_object() {
    assert_serde_eq!({"a": 1, "b": 2});
}

#[wasm_bindgen_test]
fn serde_nested() {
    assert_serde_eq!([{"a": 1}, {"b": 2}]);
    assert_serde_eq!({"a": [1], "b": [2]});
}
