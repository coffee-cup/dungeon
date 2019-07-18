use serde_derive::*;
use std::ops;
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

    pub fn square_distance(&self, other: &Vector) -> i32 {
        let xx = other.x - self.x;
        let yy = other.y - self.y;
        (xx * xx) + (yy * yy)
    }

    pub fn in_range(&self, other: &Vector, range_limit: i32) -> bool {
        self.square_distance(other) < range_limit * range_limit
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}
