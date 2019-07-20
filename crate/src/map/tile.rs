use serde::*;
use serde_repr::*;
use std::fmt;
use wasm_bindgen::prelude::*;

use crate::random::*;
use crate::vector::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Copy, Serialize_repr)]
#[repr(u8)]
pub enum TileType {
    Wall = 2,
    Floor = 3,
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Tile {
    pub visible: bool,
    pub blocked: bool,
    pub seen: bool,
    pub tile_type: TileType,
}

impl Tile {
    pub fn wall() -> Tile {
        Tile {
            visible: false,
            blocked: true,
            seen: false,
            tile_type: TileType::Wall,
        }
    }

    pub fn floor() -> Tile {
        Tile {
            visible: false,
            blocked: false,
            seen: false,
            tile_type: TileType::Floor,
        }
    }
}
