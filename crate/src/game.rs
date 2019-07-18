use serde::*;
use serde_derive::*;
use serde_repr::*;
use wasm_bindgen::prelude::*;

use crate::fov;
use crate::map::*;
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

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Copy, Serialize_repr)]
#[repr(u8)]
pub enum EntityType {
    Player = 0,
    Hidden = 1,
    Wall = 2,
    Floor = 3,
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
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Game {
    map: Map,
    player: Vector,
    pub size: Vector,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen]
    pub fn new(width: i32, height: i32, map_str: &str) -> Game {
        let mut map = Map::new(Vector::new(width, height), map_str);

        console_log!("{}", map);

        // let player = Vector::new(4, 4);
        let player = Vector::new(1, 1);

        map.set_all_visiblity(true);
        fov::refresh_visiblity(&mut map, -1, player);

        Game {
            map: map,
            player: player,
            size: Vector::new(width, height),
        }
    }

    #[wasm_bindgen]
    pub fn get_map(&self) -> JsValue {
        let mut map: Vec<EntityType> = Vec::with_capacity(self.map.tiles().len());

        for tile in self.map.tiles() {
            if tile.visible {
                if tile.blocked {
                    map.push(EntityType::Wall);
                } else {
                    map.push(EntityType::Floor);
                }
            } else {
                map.push(EntityType::Hidden)
            }
        }

        map[self.map.pos_to_index(self.player)] = EntityType::Player;

        JsValue::from_serde(&map).unwrap()
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

        if !self.map.is_blocked(new_pos) {
            self.player = new_pos;
        }

        self.map.set_all_visiblity(true);
        fov::refresh_visiblity(&mut self.map, -1, self.player);
    }
}
