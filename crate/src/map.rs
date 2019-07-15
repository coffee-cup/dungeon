use serde::*;
use serde_derive::*;
use wasm_bindgen::prelude::*;

use crate::vector::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Tile {
    pub visible: bool,
    pub blocked: bool,
}

impl Tile {
    pub fn wall() -> Tile {
        Tile {
            visible: false,
            blocked: true,
        }
    }

    pub fn floor() -> Tile {
        Tile {
            visible: false,
            blocked: false,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Map {
    size: Vector,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new(size: Vector) -> Map {
        let width = size.x as usize;
        let height = size.y as usize;

        let mut tiles = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    tiles.push(Tile::wall());
                } else {
                    tiles.push(Tile::floor());
                };
            }
        }

        tiles[130] = Tile::wall();
        tiles[131] = Tile::wall();
        tiles[132] = Tile::wall();
        tiles[133] = Tile::wall();
        tiles[134] = Tile::wall();
        tiles[135] = Tile::wall();

        Map {
            tiles: tiles,
            size: size,
        }
    }

    pub fn pos_to_index(&self, pos: Vector) -> usize {
        (self.size.x * pos.y + pos.x) as usize
    }

    pub fn is_blocked(&self, pos: Vector) -> bool {
        let index = self.pos_to_index(pos);

        if index < 0 || index >= self.tiles.len() {
            return true;
        }

        self.tiles[index].blocked
    }

    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }
}
