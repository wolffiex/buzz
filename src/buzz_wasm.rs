use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    bare_bones();
}
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn bare_bones() {
    log("Hello from Rust!");
}

#[no_mangle]
pub extern fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {}