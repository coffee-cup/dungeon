use serde::*;
use serde_derive::*;
use serde_repr::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Copy, Serialize_repr)]
#[repr(u8)]
pub enum EntityType {
    Player = 0,
    Wall = 1,
    Floor = 2,
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Copy, Serialize_repr)]
#[repr(u8)]
pub enum Direction {
    N = 0,
    NE = 1,
    E = 2,
    SE = 3,
    S = 4,
    SW = 5,
    W = 6,
    NW = 7,
}

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
}

pub type Map = Vec<EntityType>;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Game {
    map: Map,
    player: Vector,
    pub size: Vector,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen]
    pub fn new(width: i32, height: i32) -> Game {
        let map = generate_map(Vector::new(width, height));

        Game {
            map: map,
            player: Vector::new(width / 2, height / 2),
            size: Vector::new(width, height),
        }
    }

    #[wasm_bindgen]
    pub fn get_map(&self) -> JsValue {
        let mut map = self.map.clone();

        map[self.posToIndex(self.player)] = EntityType::Player;

        JsValue::from_serde(&map).unwrap()
    }

    fn posToIndex(&self, pos: Vector) -> usize {
        (self.size.x * pos.y + pos.x) as usize
    }

    fn is_valid_pos(&self, pos: Vector) -> bool {
        let index = self.posToIndex(pos);

        if index < 0 || index >= self.map.len() {
            return false;
        }

        match self.map[index] {
            EntityType::Wall => false,
            _ => true,
        }
    }

    #[wasm_bindgen]
    pub fn move_player(&mut self, dir: Direction) {
        let player = self.player;

        let new_pos = match dir {
            Direction::N => Vector {
                y: player.y - 1,
                ..player
            },
            Direction::E => Vector {
                x: player.x + 1,
                ..player
            },
            Direction::S => Vector {
                y: player.y + 1,
                ..player
            },
            Direction::W => Vector {
                x: player.x - 1,
                ..player
            },
            Direction::NE => Vector {
                x: player.x + 1,
                y: player.y - 1,
                ..player
            },
            Direction::SE => Vector {
                x: player.x + 1,
                y: player.y + 1,
                ..player
            },
            Direction::NW => Vector {
                x: player.x - 1,
                y: player.y - 1,
                ..player
            },
            Direction::SW => Vector {
                x: player.x - 1,
                y: player.y + 1,
                ..player
            },
            _ => Vector { ..player },
        };

        if self.is_valid_pos(new_pos) {
            self.player = new_pos;
        }
    }
}

fn generate_map(size: Vector) -> Map {
    let width = size.x as usize;
    let height = size.y as usize;

    let mut map = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                map.push(EntityType::Wall)
            } else {
                map.push(EntityType::Floor);
            };
        }
    }

    map
}
