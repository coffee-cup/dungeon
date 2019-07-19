use serde::*;
use serde_derive::*;
use serde_repr::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Copy, Serialize_repr)]
#[repr(u8)]
pub enum EntityType {
    Player = 0,
}
