use serde::*;
use serde_derive::*;
use std::cmp;
use wasm_bindgen::prelude::*;

use crate::fov::*;
use crate::utils::*;
use crate::vector::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

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

        let mut map = Map {
            tiles: tiles,
            size: size,
        };

        let walls: Vec<Vector> = vec![
            Vector::new(4, 2),
            Vector::new(5, 2),
            Vector::new(6, 2),
            Vector::new(7, 2),
            Vector::new(8, 2),
            Vector::new(9, 2),
            Vector::new(10, 2),
            Vector::new(11, 2),
            Vector::new(12, 2),
            Vector::new(13, 2),
            Vector::new(4, 5),
            Vector::new(5, 5),
            Vector::new(6, 5),
            Vector::new(7, 5),
            Vector::new(8, 5),
            Vector::new(9, 5),
            Vector::new(10, 5),
            Vector::new(11, 5),
            Vector::new(12, 5),
            Vector::new(13, 5),
            Vector::new(4, 13),
            Vector::new(4, 14),
            Vector::new(4, 15),
            Vector::new(5, 15),
            Vector::new(5, 14),
            Vector::new(6, 15),
        ];

        for v in walls.iter() {
            let index = map.pos_to_index(*v);
            map.tiles[index] = Tile::wall();
        }

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

    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
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
        if self.in_bounds(pos) {
            let index = self.pos_to_index(pos);
            self.tiles[index].visible = visibility;
        }
    }
}
