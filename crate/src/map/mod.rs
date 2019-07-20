use serde::*;
use serde_repr::*;
use std::fmt;
use wasm_bindgen::prelude::*;

mod automata;
mod tile;

use crate::fov::*;
use crate::map::automata::*;
use crate::map::tile::*;
use crate::vector::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Map {
    size: Vector,
    tiles: Vec<Tile>,
}

#[wasm_bindgen]
impl Map {
    pub fn new(size: Vector, map_str: &str) -> Map {
        let width = size.x as usize;
        let height = size.y as usize;

        let tiles = generate_automata_map(size);

        // let mut tiles = Vec::with_capacity(width * height);

        // for y in 0..height {
        //     for x in 0..width {
        //         if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
        //             tiles.push(Tile::wall());
        //         } else {
        //             tiles.push(Tile::floor());
        //         };
        //     }
        // }

        let mut map = Map {
            tiles: tiles,
            size: size,
        };

        // for y in 0..height {
        //     for x in 0..width {
        //         let index = map.pos_to_index(Vector::new(x as i32, y as i32));
        //         let char = map_str.chars().nth(index).unwrap();

        //         if char == '#' {
        //             map.tiles[index] = Tile::wall();
        //         } else {
        //             map.tiles[index] = Tile::floor();
        //         }
        //     }
        // }

        map
    }

    pub fn in_bounds(&self, pos: Vector) -> bool {
        !(pos.x < 0 || pos.x >= self.size.x || pos.y < 0 || pos.y >= self.size.y)
    }

    pub fn get_tile(&self, pos: Vector) -> Option<Tile> {
        if self.in_bounds(pos) {
            let index = self.pos_to_index(pos);
            Some(self.tiles[index].clone())
        } else {
            None
        }
    }

    pub fn set_all_visiblity(&mut self, visible: bool) {
        for tile in self.tiles.iter_mut() {
            tile.visible = visible;
        }
    }

    fn set_visibility(&mut self, pos: Vector, visibility: bool) {
        if self.in_bounds(pos) {
            let index = self.pos_to_index(pos);
            self.tiles[index].visible = visibility;

            if visibility {
                self.remember_tile(pos);
            }
        }
    }

    pub fn remember_tile(&mut self, pos: Vector) {
        if self.in_bounds(pos) {
            let index = self.pos_to_index(pos);
            self.tiles[index].seen = true;
        }
    }

    pub fn pos_to_index(&self, pos: Vector) -> usize {
        (self.size.x * pos.y + pos.x) as usize
    }

    pub fn is_blocked(&self, pos: Vector) -> bool {
        if self.in_bounds(pos) {
            let index = self.pos_to_index(pos);
            self.tiles[index].blocked
        } else {
            true
        }
    }

    #[wasm_bindgen]
    pub fn get_tiles(&self) -> JsValue {
        JsValue::from_serde(&self.tiles).unwrap()
    }
}

impl Area for Map {
    fn transparent(&self, pos: Vector) -> bool {
        !self.is_blocked(pos)
    }

    fn in_bounds(&self, pos: Vector) -> bool {
        self.in_bounds(pos)
    }

    fn set_visibility(&mut self, pos: Vector, visibility: bool) {
        self.set_visibility(pos, visibility)
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map_str: String = "".to_owned();
        for row in 0..self.size.y {
            for col in 0..self.size.x {
                let index = self.pos_to_index(Vector::new(col, row));

                if self.in_bounds(Vector::new(col, row)) {
                    let s = if self.tiles[index].blocked { "#" } else { " " };
                    map_str.push_str(s)
                }
            }
            map_str.push_str("\n");
        }

        write!(f, "{}", map_str)
    }
}
