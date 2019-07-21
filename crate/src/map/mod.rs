use serde::*;
use serde_repr::*;
use std::fmt;
use std::iter;
use wasm_bindgen::prelude::*;

mod tile;

use crate::fov::*;
use crate::map::tile::*;
use crate::random;
use crate::vector::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    tiles: Vec<Tile>,
}

#[wasm_bindgen]
impl Map {
    pub fn new(size: Vector, map_str: &str) -> Map {
        let width = size.x as usize;
        let height = size.y as usize;

        let mut tiles = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            tiles.push(Tile::wall());
        }

        let mut map = Map {
            tiles: tiles,
            width: width as i32,
            height: height as i32,
        };

        generate_automata_map(&mut map);
        // generate_str_map(&mut map, map_str);

        map
    }

    pub fn in_bounds(&self, pos: Vector) -> bool {
        !(pos.x < 0 || pos.x >= self.width as i32 || pos.y < 0 || pos.y >= self.height as i32)
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
        (self.width * pos.y + pos.x) as usize
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

fn generate_str_map(map: &mut Map, map_str: &str) {
    for y in 0..map.height {
        for x in 0..map.width {
            let index = map.pos_to_index(Vector::new(x as i32, y as i32));
            let char = map_str.chars().nth(index).unwrap();

            if char == '#' {
                map.tiles[index] = Tile::wall();
            } else {
                map.tiles[index] = Tile::floor();
            }
        }
    }
}

fn generate_automata_map(map: &mut Map) {
    fill_random_tiles(map);

    let num_iterations = random::range(2, 5);
    for _ in 0..num_iterations {
        run_iteration(map);
    }
}

fn run_iteration(map: &mut Map) {
    let mut next_tiles: Vec<Tile> = map.tiles.clone();

    for y in 0..map.height {
        for x in 0..map.width {
            let pos = Vector::new(x, y);
            let wall_count = num_wall_neighbours(pos, map);
            let curr_tile_type = map.get_tile(pos).unwrap().tile_type;
            let on_edge = x == 0 || x == map.width - 1 || y == 0 || y == map.height - 1;

            let index = map.pos_to_index(pos);
            if (curr_tile_type == TileType::Wall && wall_count >= 4)
                || (curr_tile_type == TileType::Floor && wall_count >= 5)
                || on_edge
            {
                next_tiles[index] = Tile::wall();
            } else {
                next_tiles[index] = Tile::floor();
            }
        }
    }

    map.tiles = next_tiles;
}

fn num_wall_neighbours(pos: Vector, map: &Map) -> u32 {
    let mut count = 0;
    for y in pos.y - 1..pos.y + 2 {
        for x in pos.x - 1..pos.x + 2 {
            if x == pos.x && y == pos.y {
                continue;
            }

            let tile = map.get_tile(Vector::new(x as i32, y as i32));
            if let Some(tile) = tile {
                if tile.tile_type == TileType::Wall {
                    count += 1;
                }
            }
        }
    }

    count
}

fn fill_random_tiles(map: &mut Map) {
    let wall_chance = 45;

    for y in 0..map.height {
        for x in 0..map.width {
            let index = map.pos_to_index(Vector::new(x, y));

            if x == 0 || x == map.width - 1 || y == 0 || y == map.height - 1 {
                map.tiles[index] = Tile::wall();
            } else {
                let r = random::percent();
                if r < wall_chance {
                    map.tiles[index] = Tile::wall();
                } else {
                    map.tiles[index] = Tile::floor();
                }
            };
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map_str: String = "".to_owned();
        for row in 0..self.height {
            for col in 0..self.height {
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
