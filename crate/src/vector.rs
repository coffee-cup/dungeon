use serde_derive::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

#[wasm_bindgen]
impl Vector {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32) -> Vector {
        Vector { x: x, y: y }
    }

    pub fn distance(&self, other: &Vector) -> i32 {
        let xx = other.x - self.x;
        let yy = other.y - self.y;
        (xx * xx) + (yy * yy)
    }
}
