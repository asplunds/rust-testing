mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&("Hello, ".to_owned() + name));
}

#[wasm_bindgen]
pub fn square(num: &str) -> i32 {

    let as_string = num.to_string();

    let as_int = as_string.parse::<i32>().unwrap();

    return as_int * as_int;

}