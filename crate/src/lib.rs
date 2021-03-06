use wasm_bindgen::prelude::*;

mod entity;
mod fov;
mod game;
mod map;
mod random;
mod utils;
mod vector;

use game::*;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
